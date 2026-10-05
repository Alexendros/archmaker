---
id: DOC-DEL-GATES-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0007
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
---

# Gates G0–G10

Tabla **generada** desde `contracts/governance/gates/G*.json` por `tools/gates.py`.
No editar a mano: `python3 tools/gates.py` falla si esta tabla contradice los manifiestos.
La semántica de los estados se lee en `docs/00-governance/status-model.md` (DOC-GOV-STATUS-001).

| Gate | Título | Fases (doc/diseño/impl/verif/release) | Estado | Criterios pendientes |
|---|---|---|---|---|
| G0 | Fuentes controladas | complete/n/a/complete/complete/n/a | complete | — |
| G1 | Problema y usuarios | complete/n/a/n/a/pending/n/a | in-progress | G1-C03 |
| G2 | Alcance y requisitos | complete/n/a/n/a/pending/n/a | in-progress | G2-C04 |
| G3 | Arquitectura | complete/complete/n/a/pending/n/a | in-progress | G3-C04 |
| G4 | Datos y contratos | in-progress/in-progress/pending/pending/n/a | in-progress | — |
| G5 | Interfaces y CorePort | in-progress/pending/pending/pending/n/a | in-progress | G5-C03 |
| G6 | UX y design system | in-progress/in-progress/pending/pending/n/a | in-progress | G6-C01, G6-C03 |
| G7 | Validación y corpus | in-progress/n/a/in-progress/pending/n/a | in-progress | — |
| G8 | Seguridad | in-progress/in-progress/pending/pending/n/a | in-progress | G8-C01, G8-C04 |
| G9 | Calidad y CI | in-progress/n/a/in-progress/pending/n/a | in-progress | G9-C03 |
| G10 | Delivery y walking skeleton | in-progress/n/a/pending/pending/pending | in-progress | G10-C03 |

## Criterios por gate

### G0 — Fuentes controladas (`complete`)

- `G0-C01` · document-approved · complete — Disposición G0 de fuentes aceptada: SRC-001 controlled-incomplete y ausentes declared-only. · 1 evidencia(s)
- `G0-C02` · metric-verified · complete — Integridad de los once originales v5.1 verificada por SHA-256 (11/11 OK). · 2 evidencia(s)
- `G0-C03` · metric-verified · complete — Alias NEUBAT byte a byte verificado (cmp idéntico). · 1 evidencia(s)
- `G0-C04` · metric-verified · complete — Manifiestos de disposición JSON válidos (ALIASES, UNAVAILABLE-SOURCES, PROVENANCE). · 2 evidencia(s)
- `G0-C05` · document-approved · complete — Baseline de auditoría congelada (tag, rama y hito). · 2 evidencia(s)
- `G0-C06` · artifact · complete — Evaluación de gates reproducible: manifiestos, tabla derivada y validador de trazabilidad. · 2 evidencia(s)

### G1 — Problema y usuarios (`in-progress`)

- `G1-C01` · document-approved · complete — Documentos de personas y objetivos aprobados. · 2 evidencia(s)
- `G1-C02` · document-approved · complete — Product brief aprobado dentro de requirements.md. · 1 evidencia(s)
- `G1-C03` · metric-verified · pending — Métricas cuantitativas de usuarios y objetivos verificadas con evidencia (documento aprobado no equivale a métrica verificada).

### G2 — Alcance y requisitos (`in-progress`)

- `G2-C01` · document-approved · complete — Requisitos FR/NFR con Given/When/Then aprobados. · 1 evidencia(s)
- `G2-C02` · document-approved · complete — Casos de uso y journeys aprobados. · 2 evidencia(s)
- `G2-C03` · decision · complete — Decisiones DEC-001..DEC-010 resueltas en la autoridad única del registro. · 2 evidencia(s)
- `G2-C04` · metric-verified · pending — Cobertura de trazabilidad FR/NFR → prueba materializada (sin referencias TST/VAL sin definir).

### G3 — Arquitectura (`in-progress`)

- `G3-C01` · document-approved · complete — C4 corregido (Rust Core como componente interno) y viewpoints ISO/IEC/IEEE 42010. · 3 evidencia(s)
- `G3-C02` · document-approved · complete — Mapa de módulos y reglas de dependencia aprobados. · 2 evidencia(s)
- `G3-C03` · decision · complete — ADR-0007 aceptado con las decisiones normativas de canonicalización (R5). · 1 evidencia(s)
- `G3-C04` · review · pending — Revisión arquitectónica independiente (R13, AUD-024).

### G4 — Datos y contratos (`in-progress`)

- `G4-C01` · decision · complete — ADR-0007 aceptado con las doce decisiones normativas de canonicalización (R5). · 2 evidencia(s)
- `G4-C02` · artifact · complete — Schemas MVP v0 (draft, catalog, diagnostic, manifest) aceptados (R6). · 2 evidencia(s)
- `G4-C03` · artifact · complete — Schemas comunes common.schema.json, core-error.schema.json y changeset.schema.json (AUD-011/014/015). · 2 evidencia(s)
- `G4-C04` · metric-verified · complete — Vectores golden de canonicalización y corpus válido/inválido en verde en CI (R5/R11). · 2 evidencia(s)

### G5 — Interfaces y CorePort (`in-progress`)

- `G5-C01` · document-approved · complete — CorePort v0 congelado con precondiciones, postcondiciones y errores tipados (R7). · 2 evidencia(s)
- `G5-C02` · artifact · complete — DTO y catálogo de errores v0 aceptados. · 3 evidencia(s)
- `G5-C03` · review · pending — Contract tests definidos con paridad Tauri/WASM sin semántica divergente. · 1 evidencia(s)

### G6 — UX y design system (`in-progress`)

- `G6-C01` · document-approved · in-progress — Design system, matriz de interacción y contratos de componente (R10). · 2 evidencia(s)
- `G6-C02` · artifact · complete — Baseline visual aprobado (golden screenshots y tokens heredados preservados). · 2 evidencia(s)
- `G6-C03` · metric-verified · pending — Matriz WCAG 2.2 AA con evidencia automatizada y manual separada (AUD-021).

### G7 — Validación y corpus (`in-progress`)

- `G7-C01` · artifact · complete — Operadores normalizados y corpus por operador (R8). · 3 evidencia(s)
- `G7-C02` · decision · complete — Operadores required ratificados y target_is diferido a v1 (decisión R8). · 1 evidencia(s)
- `G7-C03` · metric-verified · complete — Corpus positivo y negativo ejecutable en CI con diagnósticos reproducibles. · 2 evidencia(s)

### G8 — Seguridad (`in-progress`)

- `G8-C01` · document-approved · in-progress — Threat model, privilege model y política Tauri deny-by-default (R9). · 3 evidencia(s)
- `G8-C02` · decision · complete — DEC-004: transporte por Unix socket con autenticación de sesión y elevación fuera del WebView. · 2 evidencia(s)
- `G8-C03` · review · complete — Sign-off formal de Seguridad registrado. · 1 evidencia(s)
- `G8-C04` · metric-verified · pending — Pruebas negativas de capabilities/elevación y cierre de RSK-001/002/008 con evidencia de implementación.

### G9 — Calidad y CI (`in-progress`)

- `G9-C01` · artifact · complete — CI documental y de contratos con acciones fijadas por SHA y versiones pip fijadas. · 3 evidencia(s)
- `G9-C02` · metric-verified · complete — Ejecución verde sobre commit protegido con evidencia conservada (AUD-022). · 3 evidencia(s)
- `G9-C03` · artifact · pending — SBOM, firmas reales y provenance de artefactos (DEC-006).

### G10 — Delivery y walking skeleton (`in-progress`)

- `G10-C01` · document-approved · complete — Roadmap, backlog y next-issues con alcance del walking skeleton (R12). · 4 evidencia(s)
- `G10-C02` · metric-verified · complete — Owners y dependencias del backlog asignados. · 2 evidencia(s)
- `G10-C03` · review · pending — Revisión independiente y publicación del baseline planning-v1.1 (AUD-024/025).

## Derivación

- Resultado global derivado: **No-Go** — no todos los gates están `complete` (faltan G1, G2, G3, G4, G5, G6, G7, G8, G9, G10).
- `docs/00-governance/go-no-go.md` reproduce esta cobertura desde los mismos manifiestos.

