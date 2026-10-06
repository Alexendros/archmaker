---
id: DOC-SEC-NEG-EVI-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - security
reviewers:
  - independent-reviewer
---

# Evidencia de pruebas negativas del MVP (I9)

Fecha: 2026-10-06. Cubre T-I9-01..T-I9-05 del plan `DOC-DEL-MVP0-IMP-001`.
Diseño: `mvp-negative-tests.md` (`DOC-SEC-NEG-001`); política: `tauri-policy.md`;
comandos: `tauri-commands.md`; updater: `updater-threat-model.md` (`DOC-SEC-UPD-001`).

## 1. Estado de threat model y privilege model (T-I9-01)

`threat-model.md` y `privilege-model.md` siguen `in-review`/`not-verified`: la aceptación
formal exige revisión independiente y no se auto-declara aquí. La verificación de
implementación de este corte consta abajo; el estado gobernable máximo alcanzado es
`pendiente-de-verificacion`.

## 2. Resultados ejecutados (T-I9-02, T-I9-03)

### Rust — `cargo test -p archmaker-core --test negative_capabilities`: 5/5 PASS

| Test | TST-NEG | Resultado |
|---|---|---|
| `neg_fs_outside_scope_never_written` (handle `/etc/passwd` opaco, sin escritura, error `AM-*`, sin parcial) | 002, 003, 008 | PASS |
| `neg_errors_are_typed_catalog_codes` (`createDraft` denegado → `AM-SCHEMA-001` tipado) | 001 | PASS |
| `neg_capability_deny_by_default` (ventana `main`, 7 permisos `core:*`, `shell:*`+`updater` denegados) | 001, 004 | PASS |
| `neg_shell_blocked_in_tauri_conf` (CSP `default-src 'self'`, sin `unsafe-*`, devtools off) | 004, 005 | PASS |
| `neg_web_has_no_remote_or_shell_surface` (CSP self, sin CDN, allowlist 7 invokes, sin shell, 10 alias `--rh-*`) | 001, 004, 005, 007 | PASS |

### Estático — `python3 tools/check_negative_capabilities.py`: 14/14 PASS, exit 0

Deny-by-default por ventana, 7 comandos, `shell:*` denegado, CSP self (conf + HTML),
devtools off, sin CDN/red en WebView, invoke solo allowlist, sin shell en TS,
`destinationHandle` opaco, 10 alias `--rh-*`, corrección `.dark text-subtle`,
`prefers-reduced-motion`.

### Cobertura TST-NEG-001..008

| ID | Estado |
|---|---|
| 001 ventana no autorizada → denegado + `AM-PROTO-001` | Verificado (capability + test) |
| 002 `..` no escapa del scope | Verificado por diseño (handle opaco + test 002/003) |
| 003 symlink fuera del scope rechazado | Verificado por diseño (idem) |
| 004 shell/string heredado sin superficie | Verificado (sin `shell:*`, grep TS) |
| 005 recurso remoto bloqueado por CSP | Verificado (CSP self conf + HTML) |
| 006 nesting/profundidad aborta (`THR-IMP-001`) | Diseño; pendiente de test con límites en `loadCatalog` |
| 007 sin red en comandos (`NFR-OFF-001`) | Verificado estático (sin hosts remotos ni `http:*`) |
| 008 `rename` fallido sin parcial | Verificado por diseño (atómica + stub sin escritura) |

## 3. Riesgos RSK-001/002/008 (T-I9-04)

Con evidencia de implementación de este corte, sin verificación independiente: estado
`pendiente-de-verificacion` (sin cambio de nivel respecto a `residual-risk-model.md`).

| Riesgo | Control evidenciado |
|---|---|
| RSK-001 disco equivocado | Scopes mínimos, handle opaco, escritura atómica, sin rutas arbitrarias |
| RSK-002 ejecución arbitraria | Sin shell/sidecars, 7 ops enum, capabilities por ventana, errores `AM-*` |
| RSK-008 elevación insegura | Sin root/discos en MVP; elevación fuera del WebView (diseño `privilege-model.md`) |

## 4. THR-UPD-001 modelado (T-I9-05, DEC-007)

Según `updater-threat-model.md`: canal firmado keyless (Fulcio/Rekor, `trusted_root.json`
embebido), HTTPS + pin de canal, anti-rollback por versión mínima, fail-open offline
(`NFR-OFF-001`), rollback ante fallo, capability propia de updater (`updater:*` denegado
en `main`). Pruebas TST-UPD-001..008 en diseño; cierre exige pipeline de firma (fases
I10/I11). Updater deshabilitado por defecto en desarrollo (ADR-0009).

## 5. Hallazgo

`crates/archmaker-core/src/limits.rs` ya incluye los derives serde requeridos por
`dto.rs`; el workspace compila y los 5 tests negativos ejecutan en verde.
