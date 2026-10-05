# Glosario normativo

- ID: DOC-GOV-001
- Estado: draft
- Propietario: Arquitectura
- Última revisión: 2026-10-05
- Requisitos relacionados: FR-DRAFT-001, FR-CAT-001, FR-PRESET-001, FR-RESOLVE-001, FR-MANIFEST-001, FR-EXPORT-001, FR-RUN-001, FR-ENT-001
- Sustituye / sustituido por: —

## Reglas de uso

1. Los términos de este glosario son **normativos**. Ningún documento, contrato, DTO, evento o identificador puede usar uno de estos términos con un significado distinto.
2. Si un artefacto (código, schema, ADR o migración) usa un término fuera de esta definición, se abre una entrada en `contradiction-register.md`.
3. Un término ambiguo **no** se resuelve por convención implícita: se resuelve aquí o queda como `DECISION-REQUIRED`.
4. `Draft`, `Catalog`, `Preset`, `Rule` son de **producto/local**. `Policy` y el control plane son **Enterprise**. `InstallationPlan` y `Runner` son **v1**. No se mezclan fases.
5. Rust es la autoridad de la semántica de cada término (`ADR-0001`); la UI solo presenta.

## Términos canónicos

| Término | Definición canónica | No confundir con | Productor | Identidad / ciclo |
|---|---|---|---|---|
| **Draft** | Intención editable del usuario: referencias a definiciones del catálogo más valores `SelectionValue`. Es el único agregado que la UI muta directamente. Contiene intención manual y referencias, nunca estado derivado. | `Resolution` (derivada, efímera) y `Manifest` (cerrado, hasheado). Un Draft no es una selección resuelta. | UI / `createDraft`, `importDraft` | `DM-DRAFT` · UUID · editable · ver `lifecycles.md` |
| **Catalog** | Conjunto versionado e inmutable de definiciones (steps, options, capabilities, metadatos, evidencias y reglas). Declara qué se puede elegir y con qué consecuencias. **Nunca** contiene comandos, hooks ni shell ejecutables. | `Preset` (patch sobre el catálogo) y `Artifact` (salida). El catálogo no es "la configuración". | Maintainer de catálogo (PER-003) | `DM-CATALOG` · `namespace/id/version` + digest · inmutable |
| **Preset** | Patch nombrado y versionado sobre un Draft/Catalog: un conjunto de selecciones y reglas de derivación. Aplicarlo produce un `ChangeSet`; **no** es un Draft ni lo sustituye silenciosamente. | `Draft` (destino del patch) y `Catalog` (fuente de las definiciones). | Maintainer / `applyPreset` | `DM-PRESET` · `namespace/id/version` · inmutable |
| **Resolution** | Resultado determinista y efímero de resolver un Draft contra un Catalog: selección efectiva, capabilities aportadas/requeridas, derivaciones y conflictos. Recalculable a partir del mismo input; **nunca** se persiste como verdad. | `Manifest` (materialización cerrada) y `Draft` (intención). | `archmaker-resolver` · `resolveDraft` | `DM-RESOLUTION` · digest de input · recalculable |
| **Manifest** | Documento canónico, cerrado, ordenado y hasheado que materializa una Resolution **sin bloqueos**. Es la entrada de los exporters y del plan de v1. | `Resolution` (abierta, efímera) y `Artifact` (archivo de salida). | `buildManifest` | `DM-MANIFEST` · digest · inmutable |
| **Artifact** | Salida material de un exporter: p. ej. perfil ArchMaker, perfil `archinstall` (experimental) o reporte de revisión. Declara target, productor, versiones, `mediaType` y digests. | `Manifest` (fuente canónica) y `InstallationPlan` (v1). | `exportArtifact` | `DM-ARTIFACT` · digest/`mediaType` · inmutable |
| **InstallationPlan** | *(v1)* Secuencia ordenada e inmutable de operaciones **tipadas** (`enum`) derivadas de un Manifest, con prechequeos y precondiciones. **Nunca** es un script ni contiene shell. | `Manifest` (intención canónica) y `RunnerSession` (ejecución). | `archmaker-plan` | `DM-PLAN` · UUID + hash de manifest · inmutable |
| **Rule** | Expresión declarativa tipada (AST) sobre el dominio que evalúa `condición → efecto` (`block` / `suggest` / `derive` / `change` / `lock`). Se evalúa en el core; la UI no la reimplementa. | `Validator` (ejecuta comprobaciones) y `Policy` (restricción de organización). | Core (`archmaker-rules`) | `RULE-*` · versionada en el catálogo · ver `operator-spec.md` |
| **Validator** | Componente que ejecuta una etapa o familia de comprobaciones (esquema, integridad referencial, reglas, target, procedencia) y emite `Diagnostic`. Un validator puede envolver muchas Rules. | `Rule` (declaración evaluada dentro de etapas). | Core (`archmaker-validation`) | `VAL-*` · ver `pipeline.md` |
| **Policy** | *(Enterprise)* Restricción organizacional firmada que el resolver/validator debe respetar (`allow` / `deny` / `require` / `lock`). Es externa al producto local y opcional. | `Rule` (de producto) y `Capability` (habilidad técnica). | Control plane Enterprise | `DM-POLICY` · `namespace/id/version/signature` · inmutable |
| **Capability** | Habilidad técnica que una selección **aporta** (`provides`) o **requiere** (`requires`): p. ej. `fs.btrfs`, `gpu.nvidia`, `boot.uefi`. El resolver computa el cierre y detecta conflictos. | Una opción (elegible) o un paquete (instalable). La capability es semántica. | Catálogo / resolver | `CAP-*` · referenciada por reglas |
| **Runner** | *(v1)* Proceso separado, elevado temporalmente, que recibe un `InstallationPlan` inmutable por protocolo tipado y ejecuta operaciones **allowlisted** con journal append-only. Nunca interpreta scripts, catálogo ni HTML. | `Adapter` (traduce plan→API fijada, dentro del runner) y el core (sin privilegios). | `archmaker-runner` | `DM-SESSION` · ver `runner-protocol.md` |

## Términos relacionados (no ambiguos, fijados aquí para consistencia)

| Término | Definición |
|---|---|
| `SelectionValue` | Unión discriminada `single`/`multiple`/`boolean`/`number`/`text`/`secretRef`. `secretRef` nunca serializa el secreto. |
| `ChangeSet` | Diferencia explicable entre dos estados de un Draft (manual vs derivado). Producido por `applyPreset`/`compareDrafts`. |
| `Diagnostic` | Resultado tipado y localizado de una etapa de validación. Ver `errors-events.md`. |
| `CoreError` | Error estructurado de una operación de `CorePort`. Nunca string libre. Ver `errors-events.md`. |
| `ExportTarget` | Destino de exportación (`archmaker-profile`, `archinstall-profile`, `report`). Su conjunto exacto depende de **DEC-001**. |
| `Evidence` / `Claim` | Afirmación mutable de catálogo con fuente primaria, fecha de captura, verificador y caducidad. Ver `source-register.md`. |
| `Scope` | Conjunto mínimo de rutas/permisos concedidos a una capability de Tauri. Ver `tauri-policy.md`. |

## Vínculos con decisiones abiertas

| `DECISION-REQUIRED` | Término afectado | Efecto |
|---|---|---|
| DEC-001 (export MVP) | `Artifact`, `ExportTarget` | Qué produce el MVP y si `archinstall` es experimental. |
| DEC-002 (ámbito de IDs) | `Catalog`, `Draft`, `Preset` | Global `vendor.kind.id` vs scoped; afecta detección de duplicados (`CON-004`). |
| DEC-004 (transporte runner) | `Runner` | Unix socket / stdio / D-Bus y modelo de elevación. |
| DEC-006 (catálogo firmado) | `Catalog`, `Policy` | Minisign / Sigstore / ambos. |
| DEC-008 (targets soportados) | `Artifact`, `InstallationPlan` | Arch x86_64 como principal; aarch64 experimental. |

## Anti-patrones explícitamente prohibidos en el vocabulario

- Llamar "preset" a un Draft guardado.
- Llamar "plantilla" o "template" al Catalog.
- Tratar una Resolution como persistible.
- Introducir comandos (`cmd`), hooks o `pacstrap` como parte de Catalog, Rule o Manifest (`CON-005`, `CON-006`, `CON-012`).
- Reimplementar reglas en React ("regla de UI") en vez de en el core (`ADR-0001`).

