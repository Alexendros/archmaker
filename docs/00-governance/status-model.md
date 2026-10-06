---
id: DOC-GOV-STATUS-001
phase: planning
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - governance
reviewers:
  - independent-reviewer
---

# Modelo de estados multidimensional

ID `DOC-GOV-STATUS-001` · Estado `accepted` · Propietario Gobierno · Fecha 2026-10-06.

Autoridad única de los estados gobernados (fase R1, issues `AUD-002`/`AUD-003`). Resuelve
`AUD-F-02` del `audit-baseline.md`: los estados unidimensionales no distinguen documento
completo, diseño listo, implementación lista, verificado y liberación lista. Este documento
define las máquinas de estado por entidad, los campos de metadato y las reglas de transición.
Los estados se leen de aquí y del front matter de cada documento; no se repiten narrativamente
en los consumidores.

## Principios

- Una sola autoridad por dato de gobierno.
- Aprobación, implementación, verificación y liberación son ejes independientes.
- Ningún vocabulario fuera de los definidos aquí es válido.
- Un gate solo cambia por evidencia, no por declaración manual.
- La coherencia se verifica con `tools/validate_front_matter.py` y con
  `contracts/governance/document-metadata.schema.json`.

## Campos de metadato

El front matter YAML de todo documento gobernado declara los siguientes campos.

| Campo | Tipo | Descripción |
|---|---|---|
| `id` | string | Identificador único del documento o entidad. |
| `phase` | enum | Fase de producto a la que pertenece. |
| `priority` | enum | Prioridad de gobierno. |
| `documentStatus` | enum | Estado del artefacto documental. |
| `approvalStatus` | enum | Estado de aprobación por la autoridad competente. |
| `implementationStatus` | enum | Grado de materialización de lo que el documento prescribe. |
| `verificationStatus` | enum | Grado de verificación con evidencia. |
| `releaseStatus` | enum | Elegibilidad de liberación. |
| `owners` | array | Roles responsables (no vacío). |
| `reviewers` | array | Roles revisores (no vacío, disjunto de `owners`). |
| `evidence` | array (opcional) | Referencias de evidencia enlazada. |
| `dependsOn` | array (opcional) | Dependencias con el estado esperado del referenciado. |

`evidence` y `dependsOn` son campos de trazabilidad opcionales; su presencia es obligatoria
cuando una regla de transición los exige (por ejemplo, `verificationStatus: passed`).

## Vocabularios cerrados

### `phase`

`planning`, `MVP`, `v1`, `Enterprise`.

### `priority`

`P0`, `P1`, `P2`.

### `documentStatus`

`draft` → `in-review` → `accepted` → `superseded`. `draft` es contenido incompleto;
`in-review` es contenido completo pendiente de aprobación; `accepted` es aprobado y vigente;
`superseded` es reemplazado por un sucesor.

### `approvalStatus`

`pending` → `approved` | `rejected`. Representa la decisión de la autoridad, no la madurez del
texto.

### `implementationStatus`

`not-started` → `partial` → `complete`. Mide si lo prescrito existe en artefactos o producto.

### `verificationStatus`

`not-verified` → `partial` → `passed` | `failed`. `passed` exige evidencia enlazada.

### `releaseStatus`

`ineligible` → `eligible` → `released`. `eligible` exige implementación completa, aprobación y
verificación superadas.

### Roles (`owners` / `reviewers`)

`product`, `architecture`, `data`, `security`, `ux`, `quality`, `release`, `governance`,
`independent-reviewer`. Corresponden a la tabla de ROLES de
`docs/00-governance/remediation-plan-v1.1.md`; `governance` cubre Gobierno y
`independent-reviewer` es el revisor sin autoría directa.

## Entidades y máquinas de estado

El tipo de entidad se deriva del `id` y de la ruta del documento; no se declara un campo extra.

| Entidad | Derivación de `id` | Eje dominante |
|---|---|---|
| Documento | `DOC-*`, `ISSUE-*`, `G0-SOURCE-DISPOSITION` | `documentStatus` |
| ADR | `ADR-\d{4}` | `approvalStatus` |
| Requisito (FR/NFR) | `FR-*`, `NFR-*` o documento de requisitos | `implementationStatus` |
| Riesgo (RSK) | `RSK-*` o registro de riesgos | `implementationStatus` del control |
| Gate (G0–G10) | `G0`…`G10` o documento de gates | `verificationStatus` |
| Contrato | `DOC-CON-*` o `contracts/**` | `documentStatus` |

### Documento

`documentStatus` sigue `draft → in-review → accepted → superseded`. `implementationStatus`
mide la completitud del propio documento (`complete` = todas sus secciones existen). El resto de
ejes describen el objeto gobernado por el documento.

### ADR

Estados narrativos de ADR y su equivalencia con los campos:

- `proposed` ≡ `documentStatus: in-review` + `approvalStatus: pending`.
- `accepted` ≡ `documentStatus: accepted` + `approvalStatus: approved`.
- `rejected` ≡ `approvalStatus: rejected`.
- `superseded` ≡ `documentStatus: superseded`.

Un ADR `proposed` no satisface una dependencia que exija decisión `accepted`.

### Requisito (FR/NFR)

`documentStatus` refleja la aceptación del requisito; `implementationStatus` refleja su
implementación en producto (`not-started` mientras el gate global sea No-Go);
`verificationStatus` refleja su verificación con evidencia; `releaseStatus` su elegibilidad.

### Riesgo (RSK)

`implementationStatus` mide la implementación del control; `verificationStatus` su verificación
con evidencia. Un control «mitigado por diseño» con verificación pendiente es `partial`.

### Gate (G0–G10)

- `documentStatus`: `draft` (definido) → `in-review` → `accepted` (criterios fijados).
- `implementationStatus`: cobertura de criterios (`not-started` → `partial` → `complete`).
- `verificationStatus`: resultado de la evaluación (`not-verified` → `partial` → `passed` | `failed`).

Un gate está **completo** cuando `implementationStatus: complete` y
`verificationStatus: passed`. Un gate completo no puede depender de elementos incompletos.

### Contrato

`documentStatus` sigue `draft → in-review → accepted` (congelación `v0`); `implementationStatus`
mide la materialización del schema; `verificationStatus` mide meta-validación y corpus.

## Reglas de transición

R1. `verificationStatus: passed` exige `evidence` no vacío (evidencia enlazada).

R2. `releaseStatus: eligible` exige, simultáneamente, `implementationStatus: complete`,
`approvalStatus: approved` y `verificationStatus: passed`. `releaseStatus: released` exige
además `evidence` no vacío.

R3. Un ADR `proposed` (`documentStatus: in-review` + `approvalStatus: pending`) no satisface una
dependencia que exija decisión `accepted`. Una dependencia con `expectedDocumentStatus:
accepted` solo se satisface si el referenciado tiene `documentStatus: accepted` y
`approvalStatus: approved`.

R4. Un gate `complete` no puede depender de ítems incompletos: si un gate está completo, toda
entrada de `dependsOn` debe cumplir sus estados esperados.

R5. Vocabularios cerrados: cualquier valor fuera de los definidos en este documento es inválido.

R6. `documentStatus: accepted` exige `approvalStatus: approved`.

R7. `approvalStatus: rejected` es incompatible con `documentStatus: accepted`.

R8. `owners` y `reviewers` no pueden estar vacíos ni solaparse; la revisión exige independencia
respecto de la autoría.

## Evidencia y dependencias

`evidence` es una lista de referencias (commit, run de CI, artefacto, informe o comando) que
sustentan `verificationStatus`. `dependsOn` es una lista de objetos con `id` y, opcionalmente,
`expectedDocumentStatus`, `expectedApprovalStatus` y `expectedVerificationStatus`.

```yaml
dependsOn:
  - id: ADR-0007
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
```

## Validación

`tools/validate_front_matter.py`:

- Valida el front matter de todo documento gobernado (`docs/**/*.md` y `contracts/**/*.md`)
  contra `contracts/governance/document-metadata.schema.json`.
- Aplica las reglas R1–R8 y falla con código distinto de cero ante incumplimiento.
- Resuelve `dependsOn` entre documentos y reporta las inconsistencias (R3/R4) sin corregirlas.

Las inconsistencias entre estados declarados y contenido aceptado se registran para su
resolución en R3/R5; no se corrigen cambiando estados en R1.
