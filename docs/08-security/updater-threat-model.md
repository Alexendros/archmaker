---
id: DOC-SEC-UPD-001
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
  - id: ADR-0009
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
---

# Threat model del updater

Modelado de `THR-UPD-001` (actualización comprometida) y de sus controles (fase R9, issue
`AUD-019`). La amenaza se declara en `docs/08-security/threat-model.md` (`DOC-SEC-THR-001`); este
documento la desarrolla y **no** duplica el estado de las decisiones, que se leen en
`docs/00-governance/decision-register.md` (`DOC-GOV-DEC-001`). Enlaza `ADR-0009`, `DEC-006` y
`DEC-007`. El riesgo residual y su aceptación se registran en
`docs/08-security/residual-risk-model.md` (`DOC-SEC-RISK-001`).

## Contexto

El updater de Tauri introduce una superficie de red en el MVP y convierte `THR-UPD-001` y
`RSK-007` en no diferibles. La firma de artefactos arrastra la política de firma de releases
(`DEC-006`) y la operación debe seguir siendo plenamente usable sin red (`NFR-OFF-001`).

## Activos y fronteras

| Activo | Frontera | Amenaza |
|---|---|---|
| Binario y artefactos de release | Release/CI → canal → cliente | `THR-UPD-001`: artefacto manipulado o suplantado. |
| Manifiesto de actualización | Canal HTTPS → updater | Versión o digest falsificados; downgrade forzado. |
| Clave de firma | Identidad del workflow → Fulcio/Rekor | Robo o uso indebido de identidad. |
| Canal de distribución | Endpoint → cliente | Redirección, canal cruzado o endpoint no autorizado. |
| Estado de la app instalada | updater → disco | Actualización fallida que deja la app inservible. |

## THR-UPD-001 — escenarios

1. **Artefacto manipulado**: un atacante altera el binario en tránsito o en el canal de release.
2. **Suplantación de origen**: un tercero publica una actualización con identidad no autorizada.
3. **Downgrade forzado**: se induce la instalación de una versión vulnerable anterior.
4. **Canal cruzado**: se sirve beta como stable (o al revés) para exponer código no promovido.
5. **Clave comprometida**: se compromete la identidad de firma del workflow de release.
6. **Bloqueo por red**: la comprobación de actualización impide usar la app sin conectividad.
7. **Actualización fallida**: la instalación se interrumpe y deja la aplicación inutilizable.
8. **Ejecución no autorizada**: el WebView intenta invocar el updater o forzar una instalación.

## Controles

### Canales estable/beta separados

- Dos canales independientes (`stable`, `beta`) con endpoints distintos en allowlist.
- El canal se fija en build; la promoción entre canales **no recompila** (mismos artefactos
  firmados).
- El cliente no puede cambiar de canal sin decisión explícita; no hay canal cruzado silencioso.

### Firma y custodia de claves

- Cada artefacto de actualización se **verifica por firma antes de aplicarse** (`ADR-0009`).
- Firma **keyless** ligada a la identidad **OIDC del workflow** de release; **no hay clave privada
  persistente** que custodiar (`DEC-006`).
- **Fulcio** emite el certificado efímero ligado a la identidad y **Rekor** registra el evento de
  firma (transparencia). La custodia de claves se reduce a proteger la identidad del workflow y el
  registro.
- La verificación de releases usa `cosign` con las raíces de Fulcio y Rekor **fijadas**
  (`trusted_root.json` embebido), de modo que no depende de red (`DEC-006`).

### Rotación y revocación

- La rotación no gestiona claves de larga duración: depende de la identidad OIDC y del registro de
  transparencia. Cambiar la identidad de firma exige actualizar la política y re-firmar.
- La revocación se apoya en el registro de transparencia y en la política de versión mínima: una
  release comprometida se retira del canal y se publica una superior firmada.
- Las raíces fijadas se actualizan por release controlada cuando Sigstore rota su confianza.

### Anti-rollback o política explícita

- El updater aplica una **política de versión mínima**: rechaza versiones inferiores a la mínima
  permitida y downgrades no autorizados.
- Cuando un downgrade sea legítimo (incidente), se publica una release nueva superior firmada, no
  se rebaja la versión mínima de forma silenciosa.
- La política de versión mínima, compatibilidad y reversión se documenta en
  `docs/10-delivery/packaging-release.md`.

### Fail-open offline (`NFR-OFF-001`)

- Si no hay red, la aplicación **arranca y opera con normalidad**; la comprobación de
  actualizaciones solo ocurre con conectividad y **nunca bloquea** el uso.
- La ausencia de red no es un error de arranque ni impide las funciones del MVP.
- Un recurso de actualización ausente no se descarga en silencio ni degrada la app.

### Rollback ante actualización fallida

- Una actualización que falla en verificación o instalación **no se aplica** y deja la versión
  anterior operativa.
- La instalación es atómica en lo posible; ante interrupción, la app vuelve a un estado usable.
- El rollback se prueba como caso negativo (ver más abajo).

### Separación comprobación / descarga / instalación

- **Check**: consulta de versión disponible; no descarga ni aplica.
- **Download**: descarga del artefacto tras comprobar canal y versión mínima.
- **Install**: verificación de firma **obligatoria** y aplicación; solo tras verificación.
- La separación permite comprobar sin efectos, auditar y negar la instalación si la firma falla.

### Aislamiento del updater por capability

- El updater vive en una **capability propia**; ninguna otra ventana ni permiso lo alcanza.
- El WebView no invoca el updater directamente: la comprobación es una acción del host.
- Deny-by-default: sin `shell:*`, sin contenido remoto en WebView y con CSP restrictiva
  (`tauri-policy.md`).

## Pruebas negativas (diseño)

| ID | Given | When | Then |
|---|---|---|---|
| TST-UPD-001 | Artefacto con firma inválida o manipulado | El updater verifica | Se rechaza y no se aplica; la app sigue operativa. |
| TST-UPD-002 | Entorno sin red | La app arranca y se usa | Funciona de extremo a extremo (fail-open, `NFR-OFF-001`). |
| TST-UPD-003 | Endpoint fuera de allowlist | El updater intenta contactar | No se contacta; se reporta diagnóstico tipado. |
| TST-UPD-004 | Manifiesto con versión inferior a la mínima | El updater evalúa | Downgrade rechazado (anti-rollback). |
| TST-UPD-005 | Artefacto de canal beta servido en stable | El updater evalúa canal | Canal cruzado rechazado. |
| TST-UPD-006 | Instalación interrumpida | El proceso se corta | La versión anterior queda operativa (rollback). |
| TST-UPD-007 | Ventana sin capability de updater | Intenta invocarlo | Denegado; test negativo de capability (`THR-IPC-001`). |
| TST-UPD-008 | Verificación offline con `trusted_root.json` fijado | No hay red en la verificación | La verificación de firma de release resuelve sin red. |

Estas pruebas se apoyan en la fila `Tauri/WASM` y `Runner` de `docs/09-quality/test-matrix.md` y
quedan **diseñadas**, no implementadas.

## Trazabilidad

| Referencia | Relación |
|---|---|
| `THR-UPD-001` | Amenaza modelada por este documento. |
| `RSK-007` | Riesgo de supply-chain; residuo aceptado en `DOC-SEC-RISK-001`. |
| `ADR-0009` | Decisión de actualización firmada en el MVP. |
| `DEC-006` / `DEC-007` | Firma de releases y decisión de updater; citadas, no reafirmadas. |
| `NFR-OFF-001` | Operación offline; base del fail-open. |
| `G8` / `G9` | Gates de seguridad y calidad; `THR-UPD-001` se cierra antes de MVP-0. |

## Límites declarados

- **Diseño, no implementación**: no se escribe código de updater ni pipeline de firma real.
- **`THR-UPD-001` no se declara cerrado**: el cierre exige evidencia ejecutable antes de MVP-0.
- El updater permanece **deshabilitado por defecto en builds de desarrollo** (`ADR-0009`).
- No se modifica `reference/**` ni los contratos de `ADR-0007`/R5.
