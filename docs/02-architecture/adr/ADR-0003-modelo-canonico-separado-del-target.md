---
id: ADR-0003
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

# ADR-0003: Modelo canónico separado del target de instalación

- Estado: accepted
- Fecha: 2026-10-05
- Propietario: Arquitectura
- Requisitos: FR-CAT-001, FR-MANIFEST-001, FR-EXPORT-001, NFR-PORT-001, NFR-DET-001

## Contexto

El prototipo heredado mezcla catálogo, reglas, comandos, estado y UI en un mismo artefacto (`CON-009`), y su pipeline incluye `pacstrap` en la configuración de UI (`CON-012`). La decisión abierta DEC-003 plantea si el motor v1 es `archinstall`, un motor `libalpm` propio o scripts. `archinstall` y cualquier otro consumidor de la instalación son *targets* externos con su propio esquema versionado, no parte de la semántica del dominio. Las reglas de dependencia exigen que el dominio no conozca serialización externa salvo value types necesarios y que los exporters traduzcan el manifest sin alterar el draft.

Documentos de apoyo: `docs/03-data/domain-model.md` (`DM-CATALOG`, `DM-MANIFEST`, `DM-ARTIFACT`), `docs/02-architecture/dependency-rules.md`, `docs/00-governance/glossary.md`.

## Drivers

- `NFR-PORT-001`: el dominio debe ser independiente de adaptadores.
- `NFR-DET-001`: mismo input y versiones producen el mismo resultado.
- Evitar acoplar la semántica del dominio al esquema cambiante de un instalador concreto.
- Mantener `archinstall` y otros motores como frontera sustituible y versionada.
- Preservar la prohibición de shell y ejecución en catálogo/dominio.

## Opciones consideradas

1. **Acoplar el dominio a `archinstall`**: el modelo de dominio replica el profile schema del instalador.
2. **Modelo canónico propio con adapters versionados**: el dominio define `Catalog`, `Draft`, `Resolution`, `Manifest`; cada target (p. ej. perfil `archinstall`) es un adapter de exportación versionado.
3. **Scripts propios de instalación**: el dominio genera o incrusta scripts ejecutables.

## Decisión propuesta

Adoptar la opción 2. El modelo canónico de dominio se separa del target de instalación. `archinstall` y cualquier otro consumidor son **adapters versionados** que traducen un `Manifest` canónico a su propio `Artifact`; nunca forman parte del dominio ni de sus invariantes. El dominio permanece agnóstico del target y no contiene comandos, hooks ni shell; el conjunto exacto de targets de v1 queda supeditado a DEC-003 (motor) y DEC-008 (targets soportados).

## Consecuencias

- El dominio y sus pruebas no dependen del esquema de ningún instalador.
- Cada target requiere un adapter explícito, versionado y con sus propios fixtures.
- Añadir o retirar un target no modifica la semántica del dominio ni el manifest.
- Se asume coste de traducción y de mantener evidencia de compatibilidad por adapter.
- Los adapters pueden quedar desalineados con cambios del instalador; requiere verificación continua.

## Riesgos

- `RSK-005` (paquetes o compatibilidad obsoletos): un adapter puede quedar obsoleto respecto al target.
- `RSK-002` (ejecución arbitraria desde catálogo/UI): el adapter no debe reintroducir comandos.
- `RSK-010` (Enterprise contamina el MVP): el aislamiento por adapter evita acoplar targets empresariales.

## Verificación

- Chequeo de dependencias (`NFR-PORT-001`): el dominio no importa tipos de adapter ni de infraestructura.
- Golden parity: dos adapters distintos producen el mismo manifest digest para el mismo draft (vector en `docs/03-data/canonicalization.md`).
- `VAL-TARGET` y `TST-EXP-001` verifican el artefacto y sus versiones/digests.
- Prueba negativa: el catálogo y el dominio no admiten `cmd`, hooks ni `pacstrap` (`CON-005`, `CON-006`, `CON-012`).

## Sustituye

—

## Sustituido por

—

## Requisitos relacionados

FR-CAT-001, FR-MANIFEST-001, FR-EXPORT-001, NFR-PORT-001, NFR-DET-001. Depende de DEC-003 y DEC-008. Relaciona `DM-CATALOG`, `DM-MANIFEST`, `DM-ARTIFACT`.
