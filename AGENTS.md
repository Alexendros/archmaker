# AGENTS.md

## Autoridad

- `docs/00-governance/`: estados, decisiones, riesgos, contradicciones y trazabilidad.
- `docs/01-product/`: problema, personas, journeys, casos de uso y requisitos.
- `docs/02-architecture/`: C4, módulos, dependencias y ADR.
- `docs/03-data/`: modelo canónico, versionado, hashing y migraciones.
- `docs/04-interfaces/`: CorePort, IPC, protocolo, errores y eventos.
- `docs/05-ux/`: IA, flujos, componentes, estados y accesibilidad.
- `docs/06-design-system/`: tokens y estilos; no rediseñar sin decisión.
- `docs/07-validation/`: reglas, operadores y pipeline.
- `docs/08-security/`: amenazas y privilegios.
- `docs/09-quality/`: pruebas y automatización.
- `docs/10-delivery/`: gates, roadmap, backlog y releases.
- `reference/`: evidencia inmutable, nunca runtime.

## Restricciones

- No desarrollar funcionalidades mientras `planning-v1` sea No-Go.
- No ejecutar strings heredados `cmd`, hooks, `pacstrap` ni shell.
- Rust es la única autoridad semántica; TypeScript no duplica reglas.
- MVP no usa root, discos, shell, sidecars ni instalación real.
- Tauri opera deny-by-default mediante capabilities y scopes mínimos.
- El runner v1 es proceso separado, acepta operaciones tipadas y nunca scripts arbitrarios.
- No modificar el diseño visual heredado salvo corrección trazable de accesibilidad, consistencia o mantenibilidad.
- No añadir dependencias o contratos sin FR/NFR y ADR cuando proceda.
- Cada PR declara IDs de requisitos, riesgos, amenazas, validadores y pruebas afectados.

## Cursor Cloud specific instructions

Rust 1.90 llega por `rust-toolchain.toml`. Node y pnpm ya están en la imagen. `install` añade las librerías de Tauri 2 (GTK 3, WebKitGTK 4.1, ayatana-appindicator, librsvg, patchelf), `pnpm install --frozen-lockfile` en la raíz, PyYAML 6.0.2 y jsonschema 4.23.0, y un stub `apps/web/dist/index.html`.

- Ese stub es obligatorio para compilar `src-tauri`: `tauri_build` falla si `frontendDist` no existe. No es la UI. La UI de desarrollo es Vite.
- UI y Playwright: desde `apps/web`, `pnpm exec vite --host 0.0.0.0 --port 5173`. `start` la deja en la sesión tmux `vite`.
- `apps/web/index.html` enlaza `../../packages/tokens/tokens.css`. En `http://localhost:5173/` el navegador pide `/packages/tokens/tokens.css` y Vite (raíz `apps/web`) no lo sirve. Carga `src/styles.css`; los tokens de color no. El botón de tema sí cambia de texto.
- El escritorio Tauri usa `devUrl` `http://localhost:1420` y `beforeDevCommand` vacío. `pnpm tauri dev` necesita Vite en el puerto 1420, aparte del 5173.
- Sin `window.__TAURI__`, «Crear draft» en el navegador responde `AM-PROTO-002`. El flujo crear → guardar → resolver → validar → manifest → exportar se demuestra con `cargo test --workspace --locked` (incluye `end_to_end_create_save_resolve_validate_build_export`).
- Mismos checks que `.github/workflows/ci.yml`: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `pnpm check`, y los validadores Python del job `docs-python`.
- No ejecutar strings heredados `cmd`, hooks, `pacstrap` ni shell. `reference/v5.1/` es evidencia inmutable.
