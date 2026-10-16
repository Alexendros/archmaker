---
id: DOC-DEL-REM-ACC-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: in-progress
verificationStatus: partial
releaseStatus: ineligible
owners:
  - architecture
---

# Notas de aceptación — remediación operativa

## Puertas

| Puerta | Estado                                                      |
| ------ | ----------------------------------------------------------- |
| OP-0   | Hallazgos en `findings-r0.md`                               |
| OP-1   | `create-core-port.ts`: Tauri / WASM / mock explícito        |
| OP-2   | `AppManifest::commands` + `main`/`restricted` + test ACL    |
| OP-3   | `prepareExport` + invalidación por generación               |
| OP-4   | Descarga de bytes + digest en toast; export sin `store_dir` |
| OP-5   | `open_draft` + seed / IndexedDB                             |
| OP-6   | «Pendiente de validar»; banner mock; import UI              |
| OP-7   | Job CI `web-wasm-prod`                                      |
| OP-8   | README / plan-status / gap / tauri-policy alineados         |
| OP-9   | Fuera (MVP-6)                                               |

## Comandos locales

```bash
cargo test --workspace --locked
bash tools/build-wasm.sh
cd apps/web && pnpm check && pnpm build
cd apps/web && pnpm test-a11y:flow   # ?mock=1
cd apps/web && pnpm exec playwright test wasm-flow.playwright.test.ts
```

## Exclusiones

Sin ISO, runner, root, discos, `pacstrap`, canales MVP-6.
