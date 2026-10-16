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

- Funcionalidad nueva solo dentro del veredicto vigente (CONDITIONAL-CLEAR hasta MVP-4). Runner, imagen real, root, discos y shell siguen cerrados hasta un alcance `accepted` y una reevaluación del veredicto.
- No ejecutar strings heredados `cmd`, hooks, `pacstrap` ni shell.
- Rust es la única autoridad semántica; TypeScript no duplica reglas.
- MVP no usa root, discos, shell, sidecars ni instalación real.
- Tauri opera deny-by-default mediante capabilities y scopes mínimos.
- El runner v1 es proceso separado, acepta operaciones tipadas y nunca scripts arbitrarios.
- No modificar el diseño visual heredado salvo corrección trazable de accesibilidad, consistencia o mantenibilidad.
- No añadir dependencias o contratos sin FR/NFR y ADR cuando proceda.
- Cada PR declara IDs de requisitos, riesgos, amenazas, validadores y pruebas afectados.
- No fusionar a `main` mientras exista algún check de CI en fallo o pendiente: la integración solo es válida con la suite completa en verde.

## Learned User Preferences

- Responder y preguntar en español; si hay opciones, explicar beneficios y consecuencias de cada una.
- La siguiente fase se especifica desde el último plan implementado, el roadmap de `docs/10-delivery/` y el trabajo pendiente, sin ampliar el veredicto vigente.
- No incluir en un commit cambios de `.gitignore` ni artefactos de fugas salvo petición explícita.
- La cara del configurador es el HTML NEUBAT que pasó el usuario: no inventar pantallas; conectar los botones y controles que ya trae.
- En la cara UEFI el arranque preseleccionado es systemd-boot; GRUB y Limine siguen seleccionables.
- Protección de `main` sin checks bloqueantes para el dueño y sí ante colaboradores externos.

## Learned Workspace Facts

- Remoto canónico: `https://github.com/Alexendros/archmaker`. Licencia Apache-2.0.
- Baseline vigente: `implementation-baseline-mvp4`. En el árbol están el walking skeleton MVP-0 y los slices MVP-1 (dominio, catálogo y migración v5.1), MVP-2 (reglas, resolución y validación), MVP-3 (adapters de exportación) y MVP-4 (React + MockCorePort).
- MVP-5, MVP-6 (empaquetado) e `iso-scope.md` son borradores sin alcance `accepted` y no autorizan implementación.
- `reference/v5.1/` es evidencia inmutable verificada por SHA-256. La UI React vive en `apps/web/`; el dominio está en `crates/archmaker-core` y se expone por CorePort.
- ADR-0011 (propuesto, `in-review`): la selección del configurador produce un contrato JSON de aplicación; cada cláusula tiene schema en `contracts/json-schema/`, issue `kind:clause` y tests verificados en Rust. No añade target de imagen.
- La referencia visual NEUBAT vive fuera del repo, en `/home/alexendros/Aplicaciones/Repertorio/archmaker_neubat.html` (no en Descargas).
