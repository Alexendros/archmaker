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

- Funcionalidad nueva solo dentro del veredicto vigente (CONDITIONAL-CLEAR hasta MVP-5: catálogo de matriz, UI de cinco pasos, paridad Tauri/WASM). Runner, imagen real, root, discos y shell siguen cerrados hasta un alcance `accepted` adicional y una reevaluación del veredicto.
- No ejecutar strings heredados `cmd`, hooks, `pacstrap` ni shell.
- Rust es la única autoridad semántica; TypeScript no duplica reglas.
- MVP no usa root, discos, shell, sidecars ni instalación real.
- Tauri opera deny-by-default mediante capabilities y scopes mínimos.
- El runner v1 es proceso separado, acepta operaciones tipadas y nunca scripts arbitrarios.
- No modificar el diseño visual heredado salvo corrección trazable de accesibilidad, consistencia o mantenibilidad.
- No añadir dependencias o contratos sin FR/NFR y ADR cuando proceda.
- Cada PR declara IDs de requisitos, riesgos, amenazas, validadores y pruebas afectados.
- No fusionar a `main` mientras exista algún check de CI en fallo o pendiente: la integración solo es válida con la suite completa en verde.
- Nombre visible: GUI **Forge**; proyecto/motor **ArchMaker**. Neubat solo en historia/`v0.1.0`/allowlist (`tools/check_product_naming.py`).
- Evidencia de cierre: sin autoaprobación; `technicalResult=passed` para aprobar; desktop/offline **blocked** sin binario de producción del tip (`p6-desktop-offline.md`); MockCorePort no cierra gates.

## Learned User Preferences

- Responder y preguntar en español; si hay opciones, explicar beneficios y consecuencias de cada una.
- Mensajes de commit y PR en español, estilo Conventional Commits.
- La siguiente fase se especifica desde el último plan implementado, el roadmap de `docs/10-delivery/` y el trabajo pendiente, sin ampliar el veredicto vigente.
- No incluir en un commit cambios de `.gitignore` ni artefactos de fugas salvo petición explícita.
- La GUI se llama Forge (la interfaz gráfica de ArchMaker); el proyecto y el motor se llaman ArchMaker. Neubat queda fuera de la nomenclatura activa y se conserva solo como historia. No inventar pantallas: conectar los botones y controles que ya trae la cara.
- El flujo de Forge es Intención → Sistema base → Entorno → Apps → Identidad → Revisión.
- En la cara UEFI el arranque preseleccionado es systemd-boot; GRUB y Limine siguen seleccionables en modo Avanzado.
- Modo Sencillo filtra WM avanzados y capas gaming/dev; modo Avanzado muestra el catálogo completo y referencias pacman.
- Protección de `main` sin checks bloqueantes para el dueño y sí ante colaboradores externos.

## Learned Workspace Facts

- Remoto canónico: `https://github.com/Alexendros/archmaker`. Licencia Apache-2.0.
- Baseline vigente: `implementation-baseline-mvp5`. En el árbol están el walking skeleton MVP-0 y los slices MVP-1 (dominio, catálogo y migración v5.1), MVP-2 (reglas, resolución y validación), MVP-3 (adapters de exportación), MVP-4 (React + MockCorePort + cara Forge en `apps/web`) y MVP-5 (catálogo de matriz, UI cinco pasos, paridad Tauri/WASM, remediación operativa + Prove-It).
- Alcance MVP-5 cerrado (`mvp5-scope.md`, MVP-5-01..05). Hueco residual: `docs/10-delivery/mvp5-gap-spec.md` (draft). Remediación operativa: `docs/10-delivery/remediation/`. MVP-6 **accepted** solo AppImage (histórico Neubat `v0.1.0`; nombre visible vigente Forge); release [`v0.1.0`](https://github.com/Alexendros/archmaker/releases/tag/v0.1.0) publicada (verificación parcial: sin cosign local, sin 3× GUI, F7 sin `RC_SHA_FUNCIONAL`). `iso-scope.md` sigue sin `accepted`.
- `reference/v5.1/` es evidencia inmutable verificada por SHA-256. La UI React vive en `apps/web/` (`forge-face.ts`); el dominio está en `crates/archmaker-core` y se expone por CorePort.
- Arranque de adaptadores vía `apps/web/src/create-core-port.ts`: Desktop → TauriCorePort (`invoke` de `@tauri-apps/api/core`); navegador → WasmCorePort (`tools/build-wasm.sh`); MockCorePort solo con `?mock=1` u opt-in explícito, nunca por fallo silencioso de Tauri/WASM.
- ADR-0011 (propuesto, `in-review`): la selección del configurador produce un contrato JSON de aplicación; cada cláusula tiene schema en `contracts/json-schema/`, issue `kind:clause` y tests verificados en Rust. No añade target de imagen.
- La revisión auditada de arranque (UEFI / systemd-boot) vive en `docs/12-research/bootloader-review.md`.
- La referencia visual histórica de la cara vive fuera del repo, en `/home/alexendros/Aplicaciones/Repertorio/archmaker_neubat.html` (no en Descargas); el nombre de ese archivo se conserva.
