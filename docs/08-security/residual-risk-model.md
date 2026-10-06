---
id: DOC-SEC-RISK-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - security
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-GOV-STATUS-001
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
---

# Modelo de riesgo residual

Autoridad del modelo de puntuación de riesgo **inherente** frente a **residual** y del registro de
riesgos residuales del walking skeleton (fase R9, issue `AUD-018`). El vocabulario de estados y
roles se rige por `status-model.md` (`DOC-GOV-STATUS-001`); los riesgos y su nivel de gobierno se
leen en `docs/00-governance/risk-register.md` (`DOC-GOV-RSK-001`), que este documento no duplica.
Las decisiones se citan por su identificador (`DEC-*`), nunca reafirmando su estado.

## Objetivo

Convertir los controles declarados en el registro de riesgos en **riesgos gobernables**: cada
riesgo P0 declara su puntuación inherente, el control que lo reduce, la evidencia del control, la
puntuación residual, un propietario y la autoridad que acepta el residuo. Sin evidencia del control
no hay aceptación.

## Alcance

- Aplica al corte del **walking skeleton** y a los riesgos P0 con control decidido
  (`RSK-001`, `RSK-002`, `RSK-003`, `RSK-004`, `RSK-007`, `RSK-008`).
- No autoriza producto: la aceptación es de gobierno y no sustituye la verificación de
  implementación.
- No redefine niveles ni propietarios del registro de riesgos; los enlaza.

## Escalas cerradas

### Probabilidad (`P`)

| Valor | Ordinal | Significado |
|---|---:|---|
| `baja` | 1 | El evento requiere una cadena de fallos improbables o no hay superficie activa. |
| `media` | 2 | El evento es posible con un fallo único o una condición de entorno habitual. |
| `alta` | 3 | El evento es esperable en uso normal o ya observado en el prototipo heredado. |

### Impacto (`I`)

| Valor | Ordinal | Significado |
|---|---:|---|
| `bajo` | 1 | Molestia recuperable sin pérdida de datos ni privilegios. |
| `medio` | 2 | Pérdida de trabajo recuperable o degradación funcional acotada. |
| `alto` | 3 | Pérdida de datos del usuario, determinismo roto o entrega incorrecta. |
| `critico` | 4 | Compromiso de seguridad, borrado destructivo o control de privilegios. |

### Puntuación

`puntuacion = ordinal(P) × ordinal(I)` en el rango `1..12`.

Bandas orientativas: `1–2` bajo, `3–5` medio, `6–12` alto. El **nivel de gobierno** (`P0`/`P1`/`P2`)
es el declarado en `risk-register.md` y es autoritativo; puede elevarse por criticidad de producto
(determinismo, seguridad, integridad de datos) por encima de la banda.

`puntuacionResidual` debe ser **menor o igual** que `puntuacionInherente`. Todo descenso cita el
control que lo produce.

## Campos del registro de riesgo residual

| Campo | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `id` | string | sí | Identificador `RSK-*`, coherente con `risk-register.md`. |
| `riesgo` | string | sí | Descripción breve del riesgo. |
| `probabilidadInherente` / `impactoInherente` | enum | sí | Ejes inherentes antes del control. |
| `puntuacionInherente` | entero | sí | Producto de los ejes inherentes. |
| `control` | string | sí | Control o conjunto de controles que reduce el riesgo. |
| `evidenciaControl` | string[] | sí | Enlaces a la evidencia que sustenta el control. |
| `probabilidadResidual` / `impactoResidual` | enum | sí | Ejes tras el control. |
| `puntuacionResidual` | entero | sí | Producto de los ejes residuales. |
| `owner` | rol | sí | Rol propietario del control (tabla de roles de `status-model.md`). |
| `fechaAceptacion` | fecha | condicional | Obligatoria si `estado = aceptado`. Formato `AAAA-MM-DD`. |
| `autoridadAcepta` | rol | condicional | Obligatoria si `estado = aceptado`; rol competente. |
| `estado` | enum | sí | Estado del tratamiento residual. |

### Estados del riesgo residual

| Estado | Significado |
|---|---|
| `abierto` | Sin control decidido; no admite aceptación. |
| `mitigado-por-diseno` | Control diseñado pero sin evidencia enlazada. |
| `pendiente-de-verificacion` | Control con evidencia de diseño, sin verificación de implementación. |
| `aceptado` | Residuo aceptado por autoridad para el corte, con evidencia del control. |
| `cerrado` | Control verificado (`verificationStatus: passed`) con evidencia; riesgo no reabierto. |

## Reglas de aceptación

1. **Ningún riesgo P0 se marca `aceptado` sin `evidenciaControl` enlazada.** La aceptación sin
   evidencia es inválida y se registra como bloqueo.
2. **Cada control enlaza al menos a una evidencia.** Una fila sin `evidenciaControl` no pasa de
   `mitigado-por-diseno`.
3. **`puntuacionResidual ≤ puntuacionInherente`.** Toda reducción cita el control que la produce;
   un control que no reduce ningún eje debe justificarse.
4. **La aceptación exige `fechaAceptacion` y `autoridadAcepta`** de la tabla de roles. La autoridad
   que acepta un P0 no puede ser la misma que aporta en solitario la evidencia sin revisión
   independiente (`independent-reviewer`).
5. **`cerrado` exige verificación de implementación con evidencia.** Mientras la implementación no
   esté verificada, el máximo alcanzable es `aceptado` (para el corte) o
   `pendiente-de-verificacion`.
6. **Reapertura**: un riesgo `aceptado` vuelve a `pendiente-de-verificacion` si cambia su control,
   su evidencia o el corte que lo aceptó.
7. **Coherencia de estados**: el `implementationStatus` del registro mide la implementación del
   control; `verificationStatus` mide su verificación con evidencia. Un control «mitigado por
   diseño» con verificación pendiente permanece `partial` (`status-model.md`).

## Registro de riesgos residuales (walking skeleton)

Corte 2026-10-06. `P`/`I` según las escalas anteriores. Ningún P0 queda `cerrado`: la
implementación de producto no está verificada; los riesgos con control y evidencia de diseño se
aceptan para el corte y los demás quedan `pendiente-de-verificacion`.

| ID | Riesgo | P_inh | I_inh | Score_inh | Control | Evidencia del control | P_res | I_res | Score_res | Owner | Fecha aceptación | Autoridad que acepta | Estado |
|---|---|---|---:|---:|---|---|---:|---:|---|---|---|---|---|
| RSK-003 | Divergencia WASM/Tauri. | media | alto | 6 | Core Rust único como autoridad semántica; paridad Rust nativo/WASM por golden parity tests sobre el manifest canónico. | `ADR-0007`; `docs/03-data/canonicalization.md`; `contracts/json-schema/manifest.schema.json`; `contracts/json-schema/examples/05-manifest.valid.json`; `docs/09-quality/test-matrix.md` (fila Tauri/WASM). | baja | alto | 3 | architecture | 2026-10-06 | architecture | aceptado |
| RSK-004 | Migración pierde selecciones. | alta | alto | 9 | Plan de migración previo, conservación del original y corpus golden de migración; un digest distinto tras migrar es evidencia de cambio, no pérdida silenciosa. | `docs/03-data/versioning-migrations.md`; `contracts/json-schema/examples/migration/v5.1-instance.sample.json`; `contracts/json-schema/examples/migration/vnext-draft.expected.json`; `docs/03-data/canonicalization.md`; `ADR-0007`. | media | alto | 6 | data | 2026-10-06 | data | aceptado |
| RSK-007 | Supply-chain Rust/JS comprometida. | media | critico | 8 | Lockfiles, auditoría, SBOM, provenance y firmas Sigstore keyless (Fulcio/Rekor) con verificación offline por raíces fijadas; canal de actualización firmado. | `DEC-006`; `ADR-0008`; `ADR-0009`; `docs/10-delivery/packaging-release.md`; `docs/09-quality/ci-cd.md`; `docs/08-security/supply-chain-policy.md`. | baja | critico | 4 | release | 2026-10-06 | security | aceptado |
| RSK-001 | Borrado del disco equivocado. | media | critico | 8 | Inventario estable, plan hash, doble confirmación y pruebas en VM; elevación fuera del WebView. | `DEC-004`; `ADR-0004`; `docs/08-security/privilege-model.md`; `docs/08-security/mvp-negative-tests.md`. | baja | critico | 4 | security | — | security | pendiente-de-verificacion |
| RSK-002 | Ejecución arbitraria desde catálogo/UI. | media | critico | 8 | Catálogo sin comandos, operaciones `enum`, ausencia de shell, firmas y capabilities por ventana. | `ADR-0008`; `docs/04-interfaces/tauri-commands.md`; `docs/08-security/tauri-policy.md`; `docs/08-security/mvp-negative-tests.md`. | baja | critico | 4 | security | — | security | pendiente-de-verificacion |
| RSK-008 | Elevación insegura del runner. | media | critico | 8 | Proceso separado, transporte Unix socket con autenticación de sesión y elevación fuera del WebView. | `DEC-004`; `ADR-0004`; `docs/08-security/privilege-model.md`; `docs/08-security/runner-hazard-analysis.md`. | baja | critico | 4 | security | — | security | pendiente-de-verificacion |

Los riesgos `RSK-005`, `RSK-006`, `RSK-009` y `RSK-010` no cambian de estado en este corte y
permanecen según `risk-register.md`.

### Lectura del corte

- `RSK-003`, `RSK-004` y `RSK-007` quedan **aceptados para el walking skeleton**: control decidido
  y evidencia de diseño enlazada (vectores golden de canonicalización de R5, corpus de migración y
  contratos). No se declaran `cerrados` porque la implementación de producto no está verificada.
- `RSK-001`, `RSK-002` y `RSK-008` quedan **`pendiente-de-verificacion`**: control decidido y
  documentado, verificación de implementación pendiente (pruebas negativas de capabilities y de
  elevación). No admiten `cerrado` hasta que exista evidencia ejecutable.

## Trazabilidad

| Referencia | Relación |
|---|---|
| `AUD-018` | Issue de origen de este modelo. |
| `AUD-019` | Consume este modelo; amenaza `THR-UPD-001` y `RSK-007`. |
| `DOC-GOV-RSK-001` | Registro autoritativo de riesgos y niveles; este documento no los duplica. |
| `DOC-GOV-STATUS-001` | Vocabulario cerrado de estados y roles. |
| `G8` | Gate de seguridad; exige riesgo residual P0 aceptado y pruebas negativas diseñadas. |

## Límites declarados

- No implementa controles ni ejecuta pruebas; el diseño de pruebas negativas vive en
  `docs/08-security/mvp-negative-tests.md` y `docs/08-security/runner-hazard-analysis.md`.
- No eleva ningún riesgo a `cerrado` sin evidencia de implementación.
- No modifica `risk-register.md` ni los contratos de `ADR-0007`/R5; solo los enlaza.
