---
id: ADR-0006
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: not-started
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# ADR-0006: Ejes de versionado independientes

- Estado: accepted
- Fecha: 2026-10-05
- Propietario: Arquitectura
- Requisitos: FR-CAT-001, FR-MANIFEST-001, FR-EXPORT-001, FR-RUN-001, NFR-DET-001, NFR-MIG-001

## Contexto

El documento heredado declara artefactos, catálogo, target y producto sin un modelo de versiones separable (`CON-011`: afirmaciones de versión sin fuente). `docs/03-data/versioning-migrations.md` ya enumera seis versiones independientes. Mezclarlas en una sola "versión del formato" impide mutar el catálogo sin romper contratos, o evolucionar el protocolo sin tocar el documento de dominio. La determinación de resultados (`NFR-DET-001`) exige que la identidad de una salida incluya las versiones exactas de sus entradas.

Documentos de apoyo: `docs/03-data/versioning-migrations.md`, `docs/03-data/domain-model.md`, `docs/04-interfaces/runner-protocol.md`.

## Drivers

- `NFR-DET-001`: reproducibilidad ligada a versiones exactas.
- `NFR-MIG-001`: migraciones por eje, no monolíticas.
- Independencia entre evolución del documento, del catálogo, del target y del protocolo.
- Trazabilidad de procedencia de cada artefacto.

## Opciones consideradas

1. **Versión única de formato**: un solo número para todo.
2. **Seis ejes independientes**: `documentVersion`, `schemaVersion`, `catalogRef.version`, `targetRef.version`, `producer.version`, `protocolVersion`.
3. **Versionado por producto** (una versión por release de la aplicación).

## Decisión propuesta

Se adoptan **seis ejes independientes**:

| Eje | Ámbito | Compatibilidad propuesta |
|---|---|---|
| `documentVersion` | Contrato del documento de dominio | Cambio incompatible incrementa mayor; compatible incrementa menor. |
| `schemaVersion` | JSON Schema del contrato | Sigue la regla del ADR-0005; cambio de esquema → nueva versión + migration note. |
| `catalogRef.version` | Catálogo referenciado (`DM-CATALOG`) | Inmutable por versión; el digest fija la identidad exacta. |
| `targetRef.version` | Target de exportación (adapter) | Versionado por adapter; independiente del dominio (ADR-0003). |
| `producer.version` | Herramienta que produce el `Artifact` | Informativa/procedencia; no altera la semántica. |
| `protocolVersion` | Protocolo runner (`runner-protocol.md`) | Negociada antes de aceptar manifest; cambio incompatible rechaza la sesión. |

La **política de compatibilidad es por eje**: una salida solo es reproducible si coinciden los ejes que la determinan (`catalogRef.version`, `targetRef.version`, `schemaVersion` y `documentVersion`), mientras `producer.version` se registra para procedencia y `protocolVersion` se negocia en la frontera runner.

## Consecuencias

- Cada eje puede evolucionar sin forzar cambios en los demás.
- Los manifests y artefactos declaran explícitamente los ejes relevantes.
- Se asume coste de gobernanza: toda mutación de eje requiere migration note y actualización de fixtures.
- Riesgo de confusión si un eje se omite en un contrato; se controla por schema cerrado (ADR-0005).

## Riesgos

- `RSK-004` (migración pierde selecciones): se mitiga versionando y planificando por eje.
- `RSK-005` (compatibilidad obsoleta): el eje de target hace visible la obsolescencia del adapter.
- Ambigüedad de compatibilidad si la política por eje no se aplica uniformemente.

## Verificación

- Fixtures que prueben cada eje por separado y su combinación reproducible (`NFR-DET-001`).
- Prueba de negociación de `protocolVersion` en la frontera runner.
- Chequeo de que un cambio de `catalogRef.version` cambia el digest del manifest (vector en `canonicalization.md`).
- Cualquier versión concreta de dependencia o estándar queda **no verificada — fuente primaria pendiente** hasta su captura en `source-register.md`.

## Sustituye

—

## Sustituido por

—

## Requisitos relacionados

FR-CAT-001, FR-MANIFEST-001, FR-EXPORT-001, FR-RUN-001, NFR-DET-001, NFR-MIG-001. Relaciona `DM-CATALOG`, `DM-MANIFEST`, `DM-ARTIFACT`.
