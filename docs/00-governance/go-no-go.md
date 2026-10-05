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

- Documento: DOC-GOV-GNG-001 · Propietario: Arquitectura · Fecha: 2026-10-05
- Resultado: **No-Go: planificación en curso.**
- Alcance: autorizar el inicio de `MVP-0` (código funcional de producto).

## Motivo (resumen)

La planificación avanzó, pero **no puede autorizarse el desarrollo** porque:

1. No todos los gates G0–G10 están `complete` (cobertura derivada más abajo).
2. Persisten **riesgos P0 residuales**; su estado se lee en `docs/00-governance/risk-register.md`.
3. La **CI no se ha ejecutado en verde** sobre un commit protegido (AUD-022).
4. El **SBOM, las firmas reales y la provenance** quedan pendientes de artefactos de producto
   (DEC-005/DEC-006; Etapa C parcial).
5. Quedan **pendientes de ejecución**: validación de accesibilidad (G6), cierre de la verificación
   de implementación de Seguridad (G8) y owners del backlog (G10).

## Progreso

El estado de las decisiones (`DEC-*`), los ADR, los riesgos (`RSK-*`) y los gates no se repite
aquí: se lee en sus autoridades únicas (`docs/00-governance/decision-register.md`,
`docs/02-architecture/adr/`, `docs/00-governance/risk-register.md` y los manifiestos
`contracts/governance/gates/`). Este documento conserva únicamente el resultado motivado y deriva
la cobertura de gates de los manifiestos.

<!-- BEGIN GATES-DERIVED -->
## Cobertura de gates (derivada)

Bloque generado desde `contracts/governance/gates/G*.json` por `tools/gates.py`; no editar a mano.

| Gate | Estado | Fases (doc/diseño/impl/verif/release) | Falta para `complete` |
|---|---|---|---|
| G0 Fuentes controladas | complete | complete/n/a/complete/complete/n/a | — |
| G1 Problema y usuarios | in-progress | complete/n/a/n/a/pending/n/a | G1-C03 |
| G2 Alcance y requisitos | in-progress | complete/n/a/n/a/pending/n/a | G2-C04 |
| G3 Arquitectura | in-progress | complete/complete/n/a/pending/n/a | G3-C04 |
| G4 Datos y contratos | in-progress | in-progress/in-progress/pending/pending/n/a | G4-C02, G4-C03, G4-C04 |
| G5 Interfaces y CorePort | in-progress | in-progress/pending/pending/pending/n/a | G5-C01, G5-C02, G5-C03 |
| G6 UX y design system | in-progress | in-progress/in-progress/pending/pending/n/a | G6-C01, G6-C03 |
| G7 Validación y corpus | in-progress | in-progress/n/a/in-progress/pending/n/a | G7-C01, G7-C03 |
| G8 Seguridad | in-progress | in-progress/in-progress/pending/pending/n/a | G8-C01, G8-C04 |
| G9 Calidad y CI | in-progress | in-progress/n/a/in-progress/pending/n/a | G9-C01, G9-C02, G9-C03 |
| G10 Delivery y walking skeleton | in-progress | in-progress/n/a/pending/pending/pending | G10-C01, G10-C02, G10-C03 |

Resultado derivado: **No-Go** — no todos los gates están `complete` (faltan G1, G2, G3, G4, G5, G6, G7, G8, G9, G10).
<!-- END GATES-DERIVED -->

## Criterios de salida (para pasar a Go)

- G0–G10 en `complete` con evidencia ejecutable y sin P0 abiertos asociados.
- Contratos P0 con corpus válido **e** inválido validado en CI (job de corpus activo).
- CI en verde (documental + meta-validación + corpus).
- `sha256sum --check reference/v5.1/SHA256SUMS` en verde y sin regresiones de trazabilidad.
- Cierre de RSK-003/004/007 y de la verificación de implementación de Seguridad; owners del
  backlog (G10).

## Consecuencia inmediata

Autorizado únicamente: trabajo documental, captura de fuentes primarias y validaciones. **No**
autorizado: código funcional, prototipo ejecutable, acceso a discos/root, shell arbitraria ni
runner real.
