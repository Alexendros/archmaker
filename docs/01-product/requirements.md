# Requisitos

## Funcionales P0

| ID | Requisito | Aceptación | Fase |
|---|---|---|---|
| FR-DRAFT-001 | Crear y guardar drafts locales. | Round-trip sin pérdida y escritura atómica. | MVP |
| FR-IMPORT-001 | Importar y migrar v5.1. | Informe previo; original intacto; pérdidas explícitas. | MVP |
| FR-CAT-001 | Cargar catálogo versionado. | Digest, procedencia y validación disponibles. | MVP |
| FR-PRESET-001 | Aplicar presets como patches. | ChangeSet muestra cambios manuales y derivados. | MVP |
| FR-RESOLVE-001 | Resolver capacidades/conflictos. | Determinista e idempotente. | MVP |
| FR-VALIDATE-001 | Ejecutar pipeline de validación. | Diagnósticos tipados, localizados y estables. | MVP |
| FR-MANIFEST-001 | Construir manifest canónico. | Imposible con errores bloqueantes. | MVP |
| FR-EXPORT-001 | Exportar artefacto y reporte. | Incluye digests, versiones y target. | MVP |
| FR-RUN-001 | Ejecutar plan aprobado. | Confirmación ligada a hash y journal completo. | v1 |
| FR-ENT-001 | Aplicar perfiles/policies por organización. | Aislamiento tenant y auditoría. | Enterprise |

## No funcionales P0

| ID | Requisito | Evidencia |
|---|---|---|
| NFR-SEC-001 | Deny-by-default y mínimo privilegio. | Capabilities, scopes y tests negativos. |
| NFR-DET-001 | Mismo input/versiones → mismo resultado. | Golden parity Tauri/WASM. |
| NFR-ACC-001 | WCAG 2.2 AA. | Axe, teclado y revisión manual. |
| NFR-OFF-001 | MVP usable sin CDN ni servidor. | E2E offline. |
| NFR-MIG-001 | Migraciones explícitas y no destructivas. | Corpus histórico. |
| NFR-PORT-001 | Dominio independiente de adaptadores. | Dependency checks. |
| NFR-OBS-001 | Errores/eventos estructurados y redactados. | Contract tests. |

## Detalle de requisitos funcionales

Estructura según `templates/requirement.md`. Estado de todos los apartados: `draft`.

### FR-DRAFT-001 — Crear y guardar drafts locales

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001 / UC-001

#### Enunciado

El sistema permite crear y guardar **drafts** locales como intención editable: referencias a definiciones del catálogo más valores `SelectionValue`. El draft no contiene estado derivado.

#### Motivo

PER-001 y PER-002 necesitan una unidad de trabajo persistente y editable que sirva de base a resolución, validación y exportación, sin exigir terminal (OBJ-001).

#### Criterios Given/When/Then

- **Positivo:** Given un catálogo cargado (FR-CAT-001), When el usuario crea un draft y lo guarda, Then se persiste con escritura atómica y un round-trip de lectura reproduce exactamente las referencias y valores (`DM-DRAFT`).
- **Positivo:** Given un draft existente, When se reabre, Then su identidad UUID y su contenido se conservan sin migración implícita.
- **Negativo:** Given un draft con un `SelectionValue` inválido respecto al esquema, When se intenta guardar, Then la operación falla con `CoreError` tipado y no se escribe ningún archivo parcial.
- **Negativo:** Given un fallo de E/S durante la escritura, When ocurre, Then el draft previo permanece intacto (sin truncado).

#### Datos e interfaces

`DM-DRAFT` (UUID), `SelectionValue`, `secretRef`. Operaciones `createDraft`/`saveDraft` de `CorePort` (`docs/04-interfaces`).

#### Validación y seguridad

Validación de esquema del draft (`VAL-DOC`); los secretos se referencian como `secretRef`, nunca se serializan (`NFR-SEC-001`); amenaza asociada THR-IMP-001.

#### Pruebas

TST-DRAFT-001 (round-trip y entrada corrupta); paridad Tauri/WASM (`NFR-DET-001`).

#### Exclusiones

No resuelve capacidades (FR-RESOLVE-001); no persiste `Resolution`; no define formato de exportación.

### FR-IMPORT-001 — Importar y migrar v5.1

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-002 / UC-002

#### Enunciado

El sistema importa documentos heredados v5.1, detecta el formato, simula la migración y produce un nuevo draft más un informe, sin modificar el original.

#### Motivo

PER-001 y PER-002 necesitan reaprovechar configuraciones existentes sin pérdida silenciosa (OBJ-003). *(Formato v5.1: no verificado — fuente primaria pendiente.)*

#### Criterios Given/When/Then

- **Positivo:** Given un documento local v5.1 válido, When se importa, Then se genera un nuevo draft con UUID nuevo y un informe de migración con cambios y pérdidas.
- **Positivo:** Given el documento original, When finaliza la importación, Then el original queda byte-idéntico.
- **Negativo:** Given un documento con campos desconocidos o corruptos, When se intenta migrar, Then se marcan pérdidas explícitas o se rechaza con `CoreError`; nunca se descarta en silencio.
- **Negativo:** Given una migración simulada con pérdidas bloqueantes y sin confirmación del usuario, When el usuario no confirma, Then no se crea draft.

#### Datos e interfaces

`MIG-5.1`; operación `importDraft`; informe de migración tipado. Fuente primaria del formato: no verificado — fuente primaria pendiente.

#### Validación y seguridad

`VAL-MIG`; migración no destructiva (`NFR-MIG-001`); no se ejecuta contenido incrustado (`cmd`, hooks, shell). Amenaza THR-IMP-001.

#### Pruebas

TST-MIG-001 sobre corpus histórico; verificación de original byte-idéntico.

#### Exclusiones

No instala; no interpreta scripts; no migra versiones distintas de v5.1 mientras no haya decisión.

### FR-CAT-001 — Cargar catálogo versionado

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001 / UC-001, UC-003

#### Enunciado

El sistema carga catálogos versionados e inmutables con digest, procedencia y validación disponibles.

#### Motivo

El catálogo es la fuente normativa de definiciones y la base de la resolución (OBJ-001, OBJ-002).

#### Criterios Given/When/Then

- **Positivo:** Given un catálogo con `namespace/id/version` y digest, When se carga, Then queda disponible con procedencia y resultado de validación.
- **Positivo:** Given un catálogo ya cargado con el mismo digest, When se recarga, Then la operación es idempotente y no duplica definiciones.
- **Negativo:** Given un catálogo cuyo digest no coincide o cuya firma es inválida, When se carga, Then se rechaza con `Diagnostic` y no se usa.
- **Negativo:** Given una definición con campos ejecutables prohibidos, When se valida, Then se rechaza (CON-005, CON-006, CON-012).

#### Datos e interfaces

`DM-CATALOG`, `CAP-*`, `Rule`, `Evidence`/`Claim`. Mecanismo de firma: DEC-006 pendiente.

#### Validación y seguridad

`VAL-REF`/`VAL-RULE`; deny-by-default; el catálogo nunca contiene comandos, hooks ni shell.

#### Pruebas

Carga válida, digest divergente, firma inválida y definición con contenido prohibido.

#### Exclusiones

No publica catálogos (autoría de PER-003 fuera del alcance de la app); entrega de fuentes sujeta a DEC-009.

### FR-PRESET-001 — Aplicar presets como patches

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001 / UC-001, UC-004

#### Enunciado

El sistema aplica presets versionados como patches sobre un draft/catálogo, produciendo un `ChangeSet` explicable.

#### Motivo

PER-002 necesita reutilizar configuraciones sin sobrescribir silenciosamente su intención (OBJ-001, OBJ-002).

#### Criterios Given/When/Then

- **Positivo:** Given un draft y un preset compatible, When se aplica, Then se produce un `ChangeSet` que distingue cambios manuales y derivados.
- **Positivo:** Given un preset aplicado, When se compara antes/después, Then cada cambio es explicable y revertible desde el draft.
- **Negativo:** Given un preset incompatible con una selección `lock`/manual, When se aplica, Then se reporta conflicto y no se muta el draft en silencio.
- **Negativo:** Given un preset con versión no disponible, When se aplica, Then falla con `CoreError` y el draft queda intacto.

#### Datos e interfaces

`DM-PRESET` (`namespace/id/version`); operaciones `applyPreset`/`compareDrafts`; `ChangeSet`.

#### Validación y seguridad

Un preset no sustituye al draft; precedencia manual/derived/locked (UC-004); `VAL-RULE`.

#### Pruebas

Aplicación, comparación e idempotencia del diff; caso de conflicto.

#### Exclusiones

No cubre la autoría de presets (PER-003); un draft guardado no es un preset (anti-patrón de `glossary.md`).

### FR-RESOLVE-001 — Resolver capacidades/conflictos

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001 / UC-003

#### Enunciado

El sistema resuelve un draft contra un catálogo y produce una `Resolution` con selección efectiva, capabilities aportadas/requeridas, derivaciones y conflictos.

#### Motivo

Es la base del resultado válido (OBJ-002) y de la construcción del manifest.

#### Criterios Given/When/Then

- **Positivo:** Given un draft y un catálogo con versiones fijas, When se resuelve, Then el mismo input produce la misma `Resolution` (digest reproducible).
- **Positivo:** Given un draft, When se resuelve dos veces, Then la operación es idempotente y efímera; la `Resolution` no se persiste.
- **Negativo:** Given un conflicto de capabilities (proveídas/requeridas incompatibles), When se resuelve, Then la `Resolution` marca conflicto bloqueante y no se promueve a manifest.
- **Negativo:** Given referencias colgantes a definiciones ausentes, When se resuelve, Then se emite `Diagnostic` y la resolución no es válida.

#### Datos e interfaces

`DM-RESOLUTION`, `CAP-*`, operación `resolveDraft`.

#### Validación y seguridad

`VAL-REF`, `VAL-RULE`; determinismo (`NFR-DET-001`); una `Resolution` nunca se persiste como verdad.

#### Pruebas

TST-RES-001; paridad Tauri/WASM; casos de conflicto y referencia colgante.

#### Exclusiones

No valida el target final (FR-VALIDATE-001); no exporta.

### FR-VALIDATE-001 — Ejecutar pipeline de validación

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001 / UC-003, UC-004

#### Enunciado

El sistema ejecuta el pipeline de validación (esquema, integridad referencial, reglas, target, procedencia) y emite `Diagnostic` tipados, localizados y estables.

#### Motivo

Garantiza un resultado válido y explicable antes de materializar (OBJ-002).

#### Criterios Given/When/Then

- **Positivo:** Given un draft/catálogo, When se ejecuta el pipeline, Then se emiten `Diagnostic` tipados, localizados y estables.
- **Positivo:** Given un input idéntico, When se repite la validación, Then los diagnósticos coinciden y el orden es estable.
- **Negativo:** Given un error bloqueante, When se valida, Then el estado resultante impide construir manifest (FR-MANIFEST-001).
- **Negativo:** Given un `Diagnostic` sin localización (elemento o regla), When se produce, Then se trata como defecto de validador y falla la prueba de contrato.

#### Datos e interfaces

Validadores `VAL-*`; `Diagnostic` (`docs/04-interfaces/errors-events.md`); `Rule`.

#### Validación y seguridad

`VAL-DOC`/`REF`/`RULE`/`TARGET`; Rust es la autoridad semántica (`ADR-0001`); la UI no reimplementa reglas.

#### Pruebas

Contract tests de `Diagnostic`; casos positivos y negativos por etapa.

#### Exclusiones

No persuade ni guía al usuario (UX); no repara automáticamente el draft.

### FR-MANIFEST-001 — Construir manifest canónico

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001 / UC-005

#### Enunciado

El sistema construye un manifest canónico, cerrado, ordenado y hasheado cuando no existen bloqueos.

#### Motivo

Materializa la `Resolution` y es la entrada de exporters y del plan de v1 (OBJ-004).

#### Criterios Given/When/Then

- **Positivo:** Given una `Resolution` sin bloqueos y validación correcta, When se construye, Then se obtiene un manifest inmutable con digest y orden canónico.
- **Positivo:** Given un manifest, When se recalcula con los mismos inputs y versiones, Then el digest es idéntico.
- **Negativo:** Given cualquier error bloqueante, When se intenta construir, Then la operación se rechaza.
- **Negativo:** Given un input no canónico (orden o valores no normalizados), When se construye, Then se normaliza o se rechaza; nunca se emite un digest divergente para el mismo contenido.

#### Datos e interfaces

`DM-MANIFEST`; operación `buildManifest`; digest canónico.

#### Validación y seguridad

Determinismo (`NFR-DET-001`); sin comandos ni hooks; amenaza THR-IMP-001.

#### Pruebas

Golden digest; bloqueo ante error; paridad Tauri/WASM.

#### Exclusiones

No exporta (FR-EXPORT-001); no genera el plan de v1 (FR-RUN-001).

### FR-EXPORT-001 — Exportar artefacto y reporte

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001 / UC-005

#### Enunciado

El sistema exporta al menos un artefacto más un reporte, declarando target, productor, versiones, `mediaType` y digests.

#### Motivo

Entrega el resultado utilizable (OBJ-004).

#### Criterios Given/When/Then

- **Positivo:** Given un manifest válido y un target habilitado, When se exporta, Then se producen `Artifact`(s) con digests, versiones y `mediaType`, más el reporte asociado.
- **Positivo:** Given el mismo manifest, When se reexporta, Then artefacto y digests son idénticos.
- **Negativo:** Given un target no soportado o no habilitado, When se exporta, Then se rechaza con `CoreError` y no se emite artefacto parcial.
- **Negativo:** Given un fallo de escritura parcial, When ocurre, Then no queda un artefacto válido a medias; se marca incompleto y se informa.

#### Datos e interfaces

`DM-ARTIFACT`; operación `exportArtifact`; `ExportTarget`. Conjunto exacto de targets: DEC-001 pendiente; el perfil `archinstall` es experimental.

#### Validación y seguridad

`VAL-TARGET`; integridad por digest; sin shell; el artefacto declara su procedencia.

#### Pruebas

TST-EXP-001; determinismo por reexportación; fallo de E/S.

#### Exclusiones

No instala; no publica; no firma releases (DEC-006).

### FR-RUN-001 — Ejecutar plan aprobado

- Prioridad: P0
- Fase: v1
- Estado: draft
- Journey/UC: JNY-003 / UC-006

#### Enunciado

El sistema ejecuta un `InstallationPlan` aprobado mediante un runner separado, con preflight, dry-run, confirmación ligada a hash y journal append-only.

#### Motivo

Instalación segura (OBJ-005), fuera del MVP.

#### Criterios Given/When/Then

- **Positivo:** Given un runner compatible y un plan derivado de un manifest, When se confirma el hash, Then se ejecuta con preflight y journal completo de operaciones tipadas.
- **Positivo:** Given una solicitud de dry-run, When se ejecuta, Then no se muta el sistema y se reporta el plan.
- **Negativo:** Given un hash de confirmación distinto del plan, When se intenta ejecutar, Then la ejecución se rechaza.
- **Negativo:** Given una operación fuera del allowlist o una entrada no tipada, When llega al runner, Then se rechaza; nunca se interpreta script, `sh` ni HTML.

#### Datos e interfaces

`DM-PLAN`, `DM-SESSION`; `archmaker-plan`/`archmaker-runner`; protocolo tipado. Transporte: DEC-004 pendiente; motor: DEC-003 pendiente.

#### Validación y seguridad

`VAL-PLAN`; elevación temporal; deny-by-default; journal append-only. Amenaza THR-RUN-001.

#### Pruebas

TST-RUN-001; hash divergente; dry-run; operación no permitida.

#### Exclusiones

El MVP no usa root, discos, shell ni instalación real; requisito fuera del MVP.

### FR-ENT-001 — Aplicar perfiles/policies por organización

- Prioridad: P0
- Fase: Enterprise
- Estado: draft
- Journey/UC: JNY-004 / UC-007

#### Enunciado

El sistema aplica perfiles y `Policy` por organización con aislamiento de tenant y auditoría.

#### Motivo

Gobernar flotas (OBJ-006); capacidad Enterprise, no producto local.

#### Criterios Given/When/Then

- **Positivo:** Given una organización y un `PolicyBundle` firmado, When se aplica a un draft/perfil, Then se respetan `allow`/`deny`/`require`/`lock` y se registra auditoría.
- **Positivo:** Given una operación, When se consulta su traza, Then queda auditada con identidad y decisión.
- **Negativo:** Given un acceso cruzado entre tenants, When se intenta, Then se deniega y se registra (THR-TEN-001).
- **Negativo:** Given una policy con firma inválida, When se aplica, Then se rechaza.

#### Datos e interfaces

`PolicyBundle`, `DM-POLICY`, superficie `/api/v1`, RBAC. Firma: DEC-006 pendiente.

#### Validación y seguridad

`VAL-POL`; aislamiento de tenant; amenaza THR-TEN-001.

#### Pruebas

TST-TEN-001; aislamiento entre tenants; auditoría.

#### Exclusiones

Enterprise queda fuera de MVP y v1; el control plane no forma parte del producto local.

## Detalle de requisitos no funcionales

### NFR-SEC-001 — Deny-by-default y mínimo privilegio

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001, JNY-003, JNY-004 / UC-001, UC-006, UC-007

#### Enunciado

La aplicación opera deny-by-default con capacidades y scopes mínimos; el runner se eleva de forma temporal y acotada.

#### Motivo

Reducir la superficie de ataque (THR-IMP-001, THR-RUN-001).

#### Criterios Given/When/Then

- **Positivo:** Given la app Tauri, When una capability no está declarada, Then se deniega por defecto.
- **Positivo:** Given una operación permitida, When se ejecuta, Then lo hace dentro del scope mínimo declarado.
- **Negativo:** Given un intento de ampliar el scope en runtime, When ocurre, Then se deniega y se registra.
- **Negativo:** Given un secreto, When se serializa estado o evento, Then solo aparece `secretRef`, nunca el valor en claro.

#### Datos e interfaces

`Scope` (`tauri-policy.md`); `secretRef`; `CorePort`.

#### Validación y seguridad

Tests negativos de capabilities y auditoría de scopes.

#### Pruebas

Tests negativos; revisión de scopes declarados.

#### Exclusiones

No cubre la seguridad del sistema operativo anfitrión.

### NFR-DET-001 — Determinismo

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001 / UC-003, UC-005

#### Enunciado

El mismo input y las mismas versiones producen el mismo resultado (digests reproducibles).

#### Motivo

Confianza, reproducibilidad y paridad entre adaptadores.

#### Criterios Given/When/Then

- **Positivo:** Given el mismo draft/catálogo, When se resuelve y construye en Tauri y en WASM, Then los digests coinciden (golden parity).
- **Positivo:** Given el mismo input, When se repite en otra máquina, Then el digest coincide.
- **Negativo:** Given una divergencia de digest, When se detecta en CI, Then el build falla.
- **Negativo:** Given una dependencia de orden no determinista (p. ej. mapas sin ordenar), When se procesa, Then se detecta como defecto.

#### Datos e interfaces

Digest de `Resolution`, `Manifest` y `Artifact`.

#### Validación y seguridad

No aplica control de acceso; propiedad verificada por paridad.

#### Pruebas

Golden parity Tauri/WASM; TST-RES-001, TST-EXP-001.

#### Exclusiones

No exige mismo rendimiento ni misma presentación de UI.

### NFR-ACC-001 — WCAG 2.2 AA

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001 / UC-001..UC-005

#### Enunciado

La interfaz cumple WCAG 2.2 AA.

#### Motivo

PER-001 requiere prevención, progresión y recuperación accesibles.

#### Criterios Given/When/Then

- **Positivo:** Given cualquier pantalla, When se audita, Then cumple WCAG 2.2 AA (axe sin violaciones críticas más revisión manual).
- **Positivo:** Given un flujo completo, When se usa solo con teclado, Then es operable de extremo a extremo con foco visible.
- **Negativo:** Given un componente sin nombre accesible o sin foco visible, When se audita, Then falla el gate.
- **Negativo:** Given color como único portador de significado, When se revisa, Then se rechaza.

#### Datos e interfaces

Componentes y estados de `docs/05-ux`; tokens de `docs/06-design-system`.

#### Validación y seguridad

Auditoría automatizada (axe) más revisión manual y de teclado.

#### Pruebas

Auditoría automatizada y manual; recorrido por teclado.

#### Exclusiones

No define diseño visual nuevo; cualquier cambio estético exige decisión trazable.

### NFR-OFF-001 — Operación offline

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001, JNY-002 / UC-001..UC-005

#### Enunciado

El MVP es usable sin CDN ni servidor.

#### Motivo

Los objetivos de producto/local (OBJ-001..OBJ-004) no dependen de red.

#### Criterios Given/When/Then

- **Positivo:** Given un entorno sin red, When se ejecuta el flujo MVP completo, Then funciona de extremo a extremo.
- **Positivo:** Given catálogo y fuentes locales, When se cargan, Then no se contacta la red.
- **Negativo:** Given cualquier llamada de red en runtime MVP, When se detecta en E2E offline, Then falla el gate.
- **Negativo:** Given un recurso ausente localmente, When se necesita, Then se informa error claro; no se descarga en silencio.

#### Datos e interfaces

Fuentes empaquetadas/system (DEC-009); catálogo local.

#### Validación y seguridad

E2E con red bloqueada.

#### Pruebas

E2E offline.

#### Exclusiones

No aplica a Enterprise, que dispone de servicio.

### NFR-MIG-001 — Migraciones explícitas y no destructivas

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-002 / UC-002

#### Enunciado

Las migraciones son explícitas y no destructivas respecto al original.

#### Motivo

Preservar el patrimonio del usuario durante la migración (OBJ-003).

#### Criterios Given/When/Then

- **Positivo:** Given un documento legado, When se migra, Then el original queda intacto y las pérdidas quedan explícitas.
- **Positivo:** Given una migración, When se repite, Then es idempotente y no reaplica efectos ya aplicados.
- **Negativo:** Given una pérdida silenciosa, When se detecta, Then falla la prueba.
- **Negativo:** Given una migración sin informe, When finaliza, Then se rechaza.

#### Datos e interfaces

`MIG-5.1`; informe de migración tipado.

#### Validación y seguridad

`VAL-MIG`.

#### Pruebas

Corpus histórico (TST-MIG-001); original byte-idéntico; idempotencia.

#### Exclusiones

No migra formatos desconocidos ni versiones distintas de v5.1 mientras no haya decisión.

### NFR-PORT-001 — Dominio independiente de adaptadores

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001 / UC-001..UC-005

#### Enunciado

El dominio es independiente de los adaptadores y comparte una única semántica.

#### Motivo

Rust es la autoridad semántica y se requiere paridad Tauri/WASM (`ADR-0001`).

#### Criterios Given/When/Then

- **Positivo:** Given un módulo de dominio, When se inspeccionan sus dependencias, Then no depende de Tauri, UI ni adaptadores.
- **Positivo:** Given el mismo dominio, When se compila para Tauri y WASM, Then se ejecuta sin cambios de semántica.
- **Negativo:** Given un import de adaptador dentro del dominio, When se ejecuta el dependency check, Then falla.
- **Negativo:** Given una regla duplicada en TypeScript, When se revisa, Then se rechaza (`ADR-0001`).

#### Datos e interfaces

Grafo de dependencias del workspace.

#### Validación y seguridad

Comprobaciones de arquitectura en CI.

#### Pruebas

Dependency checks en CI.

#### Exclusiones

No prohíbe que existan adaptadores fuera del dominio.

### NFR-OBS-001 — Errores y eventos estructurados y redactados

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: JNY-001, JNY-003, JNY-004 / UC-001, UC-006, UC-007

#### Enunciado

Los errores y eventos son estructurados y redactados.

#### Motivo

Diagnóstico fiable sin filtrar secretos.

#### Criterios Given/When/Then

- **Positivo:** Given cualquier error, When se emite, Then es `CoreError`/`Diagnostic` estructurado, no string libre.
- **Positivo:** Given un evento, When se registra, Then los campos sensibles quedan redactados (`secretRef`).
- **Negativo:** Given un error emitido como string libre, When se detecta, Then falla el contract test.
- **Negativo:** Given un evento con secreto en claro, When se audita, Then se rechaza.

#### Datos e interfaces

`CoreError`, `Diagnostic` (`docs/04-interfaces/errors-events.md`).

#### Validación y seguridad

Redacción de sensibles; contract tests.

#### Pruebas

Contract tests de errores y eventos.

#### Exclusiones

No define backend de telemetría (DEC-010: ninguna en MVP).
