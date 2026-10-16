---
id: DOC-DEL-REM-ACC-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Notas de aceptación — remediación operativa

## Puertas OP (remediación)

| Puerta | Estado         | Nota                                                                      |
| ------ | -------------- | ------------------------------------------------------------------------- |
| OP-0   | passed         | Hallazgos en `findings-r0.md`                                             |
| OP-1   | passed         | `create-core-port.ts`: Tauri / WASM / mock explícito                      |
| OP-2   | passed         | `AppManifest::commands` + `main`/`restricted` + test ACL                  |
| OP-3   | passed         | `prepareExport` + invalidación por generación                             |
| OP-4   | passed         | Descarga de bytes + digest; export sin `store_dir` (ampliado en F2)       |
| OP-5   | passed         | `open_draft` + seed / IndexedDB                                           |
| OP-6   | passed         | «Pendiente de validar»; banner mock; import UI (ampliado en F4)           |
| OP-7   | passed         | Job CI `web-wasm-prod`                                                    |
| OP-8   | passed         | README / plan-status / gap / tauri-policy + Prove-It (re-alineados en F8) |
| OP-9   | not-applicable | Fuera de remediación (MVP-6 / empaquetado); no es fallo                   |

Estados: `passed` | `failed` | `pending` | `blocked` | `not-applicable`. OP-9 no se marca
`failed`: el empaquetado pertenece a otro alcance (`mvp6-scope.md` / F9), no a esta remediación.

## Capas posteriores a la remediación (F8 / F9)

No confundir cierre OP con aceptación funcional ni con release:

| Capa                                                                  | Estado    | Ancla                                                                 |
| --------------------------------------------------------------------- | --------- | --------------------------------------------------------------------- |
| Baseline de código (`implementation-baseline-mvp5` + F0–F8 en `main`) | passed    | Tag `b13c200…`; cierre en `docs/10-delivery/closure/`                 |
| Aprobación funcional (matriz F7 / revisor)                            | pending   | `closure/f7-candidate.md` (sin `RC_SHA_FUNCIONAL`)                    |
| Release binaria (F9)                                                  | published | Forge AppImage `v0.1.0`; verificación parcial (`closure/f9-release.md`) |

Gobierno: [`../closure/f8-governance.md`](../closure/f8-governance.md).

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

## Exclusiones (de esta remediación OP)

Sin ISO, runner, root, discos ni `pacstrap` en el cierre OP. El canal AppImage Forge
`v0.1.0` se publica en F9 (capa distinta); no cierra aceptación funcional F7 ni imagen Arch.
Tag `implementation-baseline-mvp5` (MVP-5-05) emitido en `b13c200a8747290e03383c6b76cbe31752741a31`
(objeto de tag anotado distinto del commit; el commit apuntado es el citado).
