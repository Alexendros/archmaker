---
id: DOC-DEL-REM-ACC-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Notas de aceptación — remediación operativa

## Puertas

| Puerta | Estado                                                                                                |
| ------ | ----------------------------------------------------------------------------------------------------- |
| OP-0   | Hallazgos en `findings-r0.md`                                                                         |
| OP-1   | `create-core-port.ts`: Tauri / WASM / mock explícito                                                  |
| OP-2   | `AppManifest::commands` + `main`/`restricted` + test ACL                                              |
| OP-3   | `prepareExport` + invalidación por generación                                                         |
| OP-4   | Descarga de bytes + digest en toast; export sin `store_dir`                                           |
| OP-5   | `open_draft` + seed / IndexedDB                                                                       |
| OP-6   | «Pendiente de validar»; banner mock; import UI                                                        |
| OP-7   | Job CI `web-wasm-prod`                                                                                |
| OP-8   | README / plan-status / gap / tauri-policy alineados; Prove-It (UUID wasm-safe, face-only, generación) |
| OP-9   | Fuera (MVP-6)                                                                                         |

## Prove-It (cierre de deuda de remediación)

| Comportamiento                       | Evidencia                                                                                         |
| ------------------------------------ | ------------------------------------------------------------------------------------------------- |
| UUID wasm-safe sin panic             | `new_uuid_v4_wasm_safe_is_valid_without_host_entropy` (`core_contract`) + E2E `wasm-flow`         |
| Intent face-only fuera de resolve    | `resolve_draft_rejects_face_intent_option` + `draftForCore` en `forge-helpers.playwright.test.ts` |
| Invalidación de generación al editar | `export-generation` Prove-It en `forge-helpers.playwright.test.ts` + ciclo en `App.tsx`           |

## Comandos locales

```bash
cargo test --workspace --locked
bash tools/build-wasm.sh
cd apps/web && pnpm check && pnpm build
cd apps/web && pnpm test-a11y:flow   # ?mock=1
cd apps/web && pnpm exec playwright test wasm-flow.playwright.test.ts forge-helpers.playwright.test.ts
```

## Exclusiones

Sin ISO, runner, root, discos, `pacstrap`, canales MVP-6. Tag `implementation-baseline-mvp5` (MVP-5-05) emitido en `b13c200`.
