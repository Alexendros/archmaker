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
