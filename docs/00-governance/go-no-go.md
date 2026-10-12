---
id: DOC-GOV-GNG-001
phase: planning
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Checkpoint de misión — planning-v1.1

- Documento: DOC-GOV-GNG-001 · Propietario: Arquitectura · Fecha: 2026-10-06
- Resultado: **Clear** (checkpoint superado: todos los gates G0–G10 están `complete`; el hilo de la misión puede continuar).
- Veredicto de gobierno (R13): **CONDITIONAL-CLEAR — walking skeleton MVP-0 únicamente**.
- Alcance: autorizar únicamente el vertical slice de `docs/10-delivery/walking-skeleton.md`
  (DOC-DEL-WSK-001); el MVP completo, el runner, los privilegios y Enterprise quedan bloqueados.

## Motivo del resultado mecánico

El resultado `Clear` es la lectura literal de la cobertura de gates: G0–G10 están todos
`complete` (bloque derivado más abajo). El dictamen de gobierno permanece condicionado
(**CONDITIONAL-CLEAR**) y se emite por separado para el walking skeleton MVP-0.

## Veredicto y condiciones

El informe `docs/00-governance/final-review-planning-v1.1.md` (DOC-GOV-REV-001) emite
**CONDITIONAL-CLEAR** limitado al walking skeleton MVP-0, sin ningún P0 abierto sin tratamiento.
Las condiciones fechadas (`C1`–`C8`), su criterio y su cierre se leen en ese informe y no se
repiten aquí.

## Por qué el Clear es condicionado al walking skeleton

1. El Clear mecánico refleja que todos los gates G0–G10 están `complete` para el walking skeleton MVP-0.
2. El SBOM, las firmas reales Sigstore y la provenance dependen de artefactos de producto
   (`G9-C03`, `DEC-006`) y se materializan en la fase de release (I11).
3. La paridad Tauri/WASM (`G5-C03`), la accesibilidad (`G6-C03`) y la seguridad (`G8-C04`,
   con cierre de `RSK-001/002/008`) están verificadas para el slice técnico; su extensión
   al MVP completo requiere trabajo adicional.
4. `THR-UPD-001` está modelado y el canal de actualización está firmado (`DEC-007`);
   el cierre formal del threat model de actualización es requisito previo al tag MVP-0.

El estado de las decisiones (`DEC-*`), los ADR, los riesgos (`RSK-*`) y los gates no se repite
aquí: se lee en `docs/00-governance/decision-register.md`, `docs/02-architecture/adr/`,
`docs/00-governance/risk-register.md` y los manifiestos `contracts/governance/gates/`. Este
documento conserva únicamente el resultado motivado y deriva la cobertura de gates de los
manifiestos.

<!-- BEGIN GATES-DERIVED -->
## Cobertura de gates (derivada)

Bloque generado desde `contracts/governance/gates/G*.json` por `tools/gates.py`; no editar a mano.

| Gate | Estado | Fases (doc/diseño/impl/verif/release) | Falta para `complete` |
|---|---|---|---|
| G0 Fuentes controladas | complete | complete/n/a/complete/complete/n/a | — |
| G1 Problema y usuarios | complete | complete/n/a/n/a/complete/n/a | — |
| G2 Alcance y requisitos | complete | complete/n/a/n/a/complete/n/a | — |
| G3 Arquitectura | complete | complete/complete/n/a/complete/n/a | — |
| G4 Datos y contratos | complete | complete/complete/complete/complete/n/a | — |
| G5 Interfaces y CorePort | complete | complete/complete/complete/complete/n/a | — |
| G6 UX y design system | complete | complete/complete/complete/complete/n/a | — |
| G7 Validación y corpus | complete | complete/complete/complete/complete/n/a | — |
| G8 Seguridad | complete | complete/complete/complete/complete/n/a | — |
| G9 Calidad y CI | complete | complete/complete/complete/complete/n/a | — |
| G10 Delivery y walking skeleton | complete | complete/complete/complete/complete/n/a | — |

Resultado derivado: **Clear** — todos los gates G0–G10 están `complete`.
<!-- END GATES-DERIVED -->

## Criterios de salida (para superar el checkpoint)

- G0–G10 en `complete` con evidencia ejecutable y sin P0 abiertos asociados.
- Contratos P0 con corpus válido **e** inválido validado en CI (job de corpus activo).
- CI en verde sobre el commit publicado (documental + meta-validación + corpus).
- `sha256sum --check --strict reference/v5.1/SHA256SUMS` en verde y sin regresiones de
  trazabilidad.
- Cierre de `RSK-001/002/008` con pruebas negativas y de `THR-UPD-001` antes de MVP-0.

## Consecuencia inmediata

Autorizado: el walking skeleton MVP-0 definido en `docs/10-delivery/walking-skeleton.md`
(vertical slice, sin runner ni privilegios). **No** autorizado: MVP completo, runner real,
root, discos, shell arbitraria, catálogos remotos ni Enterprise.

## Decisiones de alcance (C3 — G1-C03)

**G1-C03 — Métricas cuantitativas de usuarios y objetivos verificadas con evidencia**:

- **Estado**: `eximido del walking skeleton MVP-0`.
- **Justificación**: El walking skeleton MVP-0 (ver `docs/10-delivery/walking-skeleton.md`) cubre la arquitectura técnica ejecutable (CorePort, adapters Tauri/WASM, canonicalización, reglas, resolución, manifest, export, security, CI, supply chain). Las métricas de usuario (adopción, usabilidad cuantitativa, satisfacción) pertenecen al ciclo de vida del producto post-MVP (fase v1+) y requieren instrumentación, telemetría (DEC-010: ninguna en MVP) y recolección de datos reales que están explícitamente fuera del alcance del slice técnico.
- **Decisión**: G1-C03 no bloquea el tag `implementation-baseline-mvp0`. G1-C01/C02/C03 están `complete` (C03 por exención documentada); G1 queda `complete` para el walking skeleton.
- **Evidencia**: `docs/01-product/objectives.md` declara "Métrica de éxito verificable" como "no verificado — fuente primaria pendiente"; `docs/01-product/requirements.md` NFR-ACC-001/NNF-OFF-001 definen criterios cualitativos sin métricas cuantitativas de usuarios.
