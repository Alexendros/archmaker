---
id: DOC-SEC-RUN-001
phase: v1
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
  - id: ADR-0004
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
---

# Análisis de peligros del Runner v1

Fase R9 (issue `AUD-018`). Analiza los peligros del Runner v1 y los controles que los mitigan. El
diseño se lee en `docs/04-interfaces/runner-protocol.md` y `ADR-0004`; el modelo de privilegios en
`docs/08-security/privilege-model.md`. Amenazas asociadas: `THR-RUN-001` (el WebView controla root)
y `THR-RUN-002` (TOCTOU tras confirmar). Riesgo: `RSK-008`.

> **El Runner v1 SIGUE BLOQUEADO.** Este documento es análisis de peligros y diseño de controles;
> no autoriza implementación ni ejecución. Permanece fuera del MVP (`AGENTS.md`, `privilege-model.md`)
> y no se desbloquea hasta los gates de v1 y la verificación de implementación con evidencia.

## Alcance y frontera

- El Runner es un **proceso separado** que recibe un `InstallationPlan` inmutable y ejecuta solo
  **operaciones allowlisted** (`enum`); nunca interpreta scripts, catálogo ni HTML.
- La frontera de ejecución es el protocolo tipado; el WebView no controla el proceso elevado.
- La elevación ocurre **fuera del WebView** (`DEC-004`).

## Peligros y controles

| ID | Peligro | Consecuencia | Control |
|---|---|---|---|
| HZ-RUN-001 | Sesión no autenticada o suplantada. | Un tercero controla el Runner. | Autenticación de sesión y de identidad peer por el transporte (Unix socket, `DEC-004`). |
| HZ-RUN-002 | Replay de mensajes. | Reejecución de una operación confirmada. | Nonce, expiración y protección anti-replay por mensaje. |
| HZ-RUN-003 | TOCTOU entre preflight y ejecución. | Se ejecuta sobre un estado distinto del validado. | Re-preflight, snapshot del target y hashes ligados a la confirmación. |
| HZ-RUN-004 | Confirmación no ligada al plan. | Se ejecuta un plan distinto del mostrado. | Confirmación ligada al **hash de plan y de manifest**. |
| HZ-RUN-005 | Journal no verificable o alterado. | No hay auditoría fiable. | Journal **append-only** redactado y verificable. |
| HZ-RUN-006 | Cancelación o recuperación insegura. | Estado inconsistente o daño parcial. | Cancelación solo en **safe points**; recuperación documentada. |
| HZ-RUN-007 | Operación fuera de allowlist. | Ejecución arbitraria. | **Allowlist tipada** de operaciones compiladas; nada interpretado. |
| HZ-RUN-008 | Elevación alcanzable desde el WebView. | Escalada de privilegios. | Elevación fuera del WebView y transporte autenticado. |
| HZ-RUN-009 | Versión de protocolo incompatible. | Comportamiento indefinido. | Negociación de `protocolVersion` antes de aceptar el manifest. |
| HZ-RUN-010 | Cambio de inventario tras confirmar. | Se opera sobre un target distinto. | Un cambio de inventario **invalida el plan**. |

## Controles detallados

### Autenticación de sesión

- El transporte autentica **peer y sesión** antes de aceptar cualquier mensaje.
- Sin sesión autenticada no hay preflight, plan ni ejecución.
- La elevación es un componente dedicado fuera del contenido WebView.

### Nonce, caducidad y anti-replay

- Cada mensaje lleva `nonce` y expiración; un `nonce` usado no se acepta de nuevo.
- Una confirmación expirada es inválida y exige nueva confirmación.
- Límites y timeouts por mensaje (`runner-protocol.md`).

### TOCTOU entre preflight y ejecución

- Entre el preflight y la ejecución se toma un **snapshot** del target y se **re-preflight**.
- La confirmación liga el hash del plan y del manifest; si el estado cambió, el hash no coincide y
  la ejecución se rechaza.
- Un cambio de inventario invalida el plan (`ADR-0004`).

### Confirmación ligada al hash del plan

- La confirmación destructiva expira y se liga a `plan hash` + `manifest hash`.
- Un plan o un inventario distintos producen un hash distinto y una confirmación inválida
  (`TST-RUN-001`, `VAL-PLAN`).

### Journal verificable

- Journal **append-only** redactado; registra operaciones, resultados y cancelaciones sin secretos.
- Su integridad es verificable; la exportación (`journal.export`) es solo de lectura.

### Recuperación y cancelación

- La cancelación solo ocurre en **safe points**; no deja el target en estado inconsistente.
- Existe procedimiento de recuperación ante fallo parcial.

### Allowlist tipada

- Solo se ejecutan operaciones compiladas y tipadas (`enum`); nunca scripts ni comandos del
  catálogo.
- Toda operación no allowlisted se rechaza por diseño.

## Pruebas negativas (diseño)

| ID | Given | When | Then |
|---|---|---|---|
| TST-RUN-001 | Plan o inventario cambiado tras confirmar | Se intenta ejecutar | Confirmación inválida; ejecución rechazada. |
| TST-RUN-002 | Mensaje reenviado con `nonce` ya usado | El Runner lo recibe | Rechazado (anti-replay). |
| TST-RUN-003 | Confirmación expirada | Se envía | Rechazada; exige nueva confirmación. |
| TST-RUN-004 | Operación fuera de allowlist | Se envía | Rechazada por diseño. |
| TST-RUN-005 | WebView sin autorización | Intenta abrir el socket o invocar al Runner | Denegado. |
| TST-RUN-006 | Cancelación en punto no seguro | Se solicita | Rechazada o diferida a un safe point. |
| TST-RUN-007 | `protocolVersion` incompatible | Se negocia | Manifest no aceptado. |
| TST-RUN-008 | Journal manipulado | Se verifica | La alteración se detecta. |

## Criterio de desbloqueo

El Runner v1 no se desbloquea hasta:

1. Gates de v1 superados y `ADR-0004` verificado con evidencia ejecutable.
2. Pruebas negativas anteriores **implementadas y en verde**.
3. Cierre de `RSK-008` con evidencia de implementación (hoy `pendiente-de-verificacion`).

Hasta entonces, `RSK-001`, `RSK-002` y `RSK-008` permanecen con verificación pendiente
(`DOC-SEC-RISK-001`).

## Trazabilidad

| Referencia | Relación |
|---|---|
| `THR-RUN-001` / `THR-RUN-002` | Amenazas modeladas por este análisis. |
| `RSK-001` / `RSK-002` / `RSK-008` | Riesgos asociados; verificación pendiente. |
| `ADR-0004` | Runner separado, transporte y elevación. |
| `DEC-004` | Transporte Unix socket con autenticación de sesión. |
| `DOC-IF-RUN-001` | Protocolo y máquina de estados del Runner. |
| `G8` | Gate de seguridad; el Runner permanece bloqueado hasta los gates de v1. |

## Límites declarados

- **Análisis y diseño, no implementación**: no se escribe Runner, transporte ni journal.
- **El Runner sigue bloqueado** hasta los gates de v1; este documento no lo desbloquea.
- No modifica `reference/**` ni los contratos de `ADR-0007`/R5.
