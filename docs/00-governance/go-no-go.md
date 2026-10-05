---
id: DOC-GOV-GNG-001
phase: planning
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Resultado Go/No-Go — planning-v1

- Documento: DOC-GOV-GNG-001 · Estado: draft · Propietario: Arquitectura · Fecha: 2026-10-05
- Resultado: **No-Go: planificación en curso.**
- Alcance: autorizar el inicio de `MVP-0` (código funcional de producto).

## Motivo (resumen)

La planificación avanzó (ver «Progreso»), pero **no puede autorizarse el desarrollo** porque:

1. Persisten **riesgos P0 residuales**: RSK-003 (divergencia WASM/Tauri), RSK-004 (migración pierde selecciones) y RSK-007 (supply-chain) siguen abiertos; RSK-001/002/008 tienen control definido y sign-off, con verificación de implementación pendiente.
2. La **CI no se ha ejecutado en verde**: los jobs de documentación, meta-validación y corpus existen (Etapas C), pero aún no hay una ejecución verde registrada.
3. El **SBOM, las firmas reales y la provenance** quedan pendientes de artefactos de producto (DEC-005/DEC-006 aceptadas; Etapa C parcial).
4. Quedan **pendientes de ejecución**: validación de accesibilidad (G6), cierre de la verificación de implementación de Seguridad (G8) y owners del backlog (G10).

## Progreso desde la evaluación anterior

- **DEC-001..DEC-008 accepted** (2026-10-05): alcance de exportación, IDs globales `vendor.kind.id`, adapter archinstall, transporte del runner (Unix socket + auth), licencia (Apache-2.0/dual), firma (Sigstore + offline), **updater firmado en MVP** (desviación), **solo Arch x86_64** (desviación).
- **ADR-0001..ADR-0009 accepted**.
- **8 JSON Schema Draft 2020-12** cerrados (`additionalProperties:false`) + corpus válido/inválido + fixture de migración v5.1.
- **Fuentes SRC-001..SRC-008 verificadas** (SRC-002/003/006 en Paso 2; SRC-005/007/008 en Etapa B).
- **CI documental** con acciones fijadas por SHA; `sha256sum --check` en verde; job `corpus-validate` (válido/inválido) añadido (Etapa C).
- **Etapa D**: sign-off de Seguridad registrado 2026-10-05; RSK-001/002/008 con control definido (verificación de implementación pendiente).
- **Etapa A (A1-A6)**: marcadores obsoletos resueltos, índice ADR, plantilla de issue, severidad canónica, catálogo AM-* completo.
- **Etapa E (2026-10-05)**: aceptados FR/NFR, modelo de dominio, C4/módulos, personas, objetivos, journeys y casos de uso; ADR-0007 rebajado a `proposed`; operadores `required` ratificado y `target_is` diferido a v1; sign-off de Seguridad aprobado formalmente.

## Cobertura de gates

| Gate | Estado | Falta para `complete` |
|---|---|---|
| G0 Fuentes | complete | — (evidencia v5.1 íntegra; capturas primarias pendientes como tareas) |
| G1 Problema/usuarios | complete | — (docs accepted; métricas cuantitativas no verificadas) |
| G2 Alcance/requisitos | complete | — (docs accepted; DEC resueltas) |
| G3 Arquitectura | complete | — (docs accepted; ADR-0001..0009 accepted, ADR-0007 proposed) |
| G4 Datos | partial | CI en verde; aprobar modelo e identidad |
| G5 Interfaces | partial | aprobar CorePort/DTO; ratificar cobertura CorePort |
| G6 UX/diseño | partial | validación de accesibilidad; aprobar tokens preservados |
| G7 Validación | partial | ratificar `required`; RULE-GPU-001 no implementable |
| G8 Seguridad | partial | verificación de implementación (pruebas negativas de capabilities/elevación); RSK-001/002/008 con control definido y sign-off |
| G9 Calidad | partial | CI en verde; SBOM/firmas reales |
| G10 Delivery | partial | owners en backlog; releases |

## Criterios de salida (para pasar a Go)

- G0–G10 en `complete` con evidencia ejecutable y sin P0 abiertos asociados.
- Contratos P0 con corpus válido **e** inválido validado en CI (job de corpus activo).
- CI en verde (documental + meta-validación + corpus).
- `sha256sum --check reference/v5.1/SHA256SUMS` en verde y sin regresiones de trazabilidad.
- Cierre de RSK-003/004/007 y de la verificación de implementación de Seguridad; owners del backlog (G10).

## Consecuencia inmediata

Autorizado únicamente: trabajo documental, captura de fuentes primarias y validaciones. **No** autorizado: código funcional, prototipo ejecutable, acceso a discos/root, shell arbitraria ni runner real.
