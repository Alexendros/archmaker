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

- Funcionalidad nueva solo dentro del veredicto vigente (ADR-0020): GUI fina sin root que valida, genera un perfil archiso completo y un `user_configuration.json` real de archinstall, y construye la ISO con `mkarchiso` en Docker (ADR-0021). El recorrido es Sistema base → Entorno → Apps → Identidad y seguridad → Revisión. Quedan cerrados el runner de instalación, root, escritura de discos (A6), shell arbitraria, aarch64 y Enterprise.
- `mkarchiso` solo dentro del contenedor Docker descrito en ADR-0021 (nunca en host, `sudo` ni `pkexec`); perfil montado en solo lectura y salida explícita.
- No ejecutar strings heredados `cmd`, hooks, `pacstrap` ni shell.
- Rust es la única autoridad semántica; TypeScript no duplica reglas.
- Tauri opera deny-by-default mediante capabilities y scopes mínimos.
- No modificar el diseño visual heredado salvo corrección trazable de accesibilidad, consistencia o mantenibilidad.
- No añadir dependencias o contratos sin FR/NFR y ADR cuando proceda.
- No fusionar a `main` mientras exista algún check de CI en fallo o pendiente: la integración solo es válida con la suite completa en verde.
- Nombre visible: GUI **Forge**; proyecto/motor **ArchMaker**. Neubat solo en historia/`v0.1.0`/allowlist (`tools/check_product_naming.py`).
- `reference/` es evidencia histórica, no normativa tras la reorientación.

## Learned User Preferences

- Responder y preguntar en español; si hay opciones, explicar beneficios y consecuencias de cada una.
- Mensajes de commit y PR en español, estilo Conventional Commits.
- La siguiente fase se especifica desde el último plan implementado, el roadmap de `docs/10-delivery/` y el trabajo pendiente, sin ampliar el veredicto vigente.
- No incluir en un commit cambios de `.gitignore` ni artefactos de fugas salvo petición explícita.
- La GUI se llama Forge (la interfaz gráfica de ArchMaker); el proyecto y el motor se llaman ArchMaker. Neubat queda fuera de la nomenclatura activa y se conserva solo como historia. No inventar pantallas: conectar los botones y controles que ya trae la cara.
- El flujo de Forge es Sistema base → Entorno → Apps → Identidad y seguridad → Revisión (experiencia única; sin Sencillo/Avanzado). En Revisión se pide la imagen. No hay paso de disco, particiones, GPT, ESP, LUKS ni formatear. El SSD lo instala archinstall en la sesión viva.
- En la cara UEFI el arranque preseleccionado es systemd-boot; GRUB y Limine siguen seleccionables.
- Título visible de ventana/documento/cabecera: ArchMaker Forge; productName Forge e identifier intactos.
- Protección de `main` sin checks bloqueantes para el dueño y sí ante colaboradores externos.

## Learned Workspace Facts

- Remoto canónico: `https://github.com/Alexendros/archmaker`. Licencia Apache-2.0.
- Baseline vigente: `implementation-baseline-mvp5`. En el árbol están el walking skeleton MVP-0 y los slices MVP-1 (dominio, catálogo y migración v5.1), MVP-2 (reglas, resolución y validación), MVP-3 (adapters de exportación), MVP-4 (React + MockCorePort + cara Forge en `apps/web`) y MVP-5 (catálogo de matriz, UI cinco pasos, paridad Tauri/WASM, remediación operativa + Prove-It).
- Alcance MVP-5 cerrado (`mvp5-scope.md`, MVP-5-01..05). Hueco residual histórico: `docs/10-delivery/mvp5-gap-spec.md` (draft; cara supersedida por incremento Forge 2026-10-17). Remediación operativa: `docs/10-delivery/remediation/`. MVP-6 **accepted** solo AppImage (histórico Neubat `v0.1.0`; nombre visible vigente Forge); **ninguna release publicada** (tags `v0.1.0`/`v0.2.0` sin GitHub Release ni AppImage, corregido el 2026-10-25; próxima `v0.3.0`). Reorientación aprobada el 2026-10-25: GUI fina sin root → perfil `archiso` completo + `user_configuration.json` real + ISO con `mkarchiso` en Docker (ADR-0020/0021). `build-scope.md` **accepted** el 2026-10-18 solo para imagen viva x86_64 UEFI (ISO-01). ISO-01..ISO-04 ya están en `main`: target `archmaker-live-profile`, crate `archmaker-constitute`, `scripts/build-iso.sh` + `docker/archiso.Dockerfile` y arranque UEFI en CI (G9). Incremento vigente **ISO-05**: cierre y verificación de la reorientación (ADR-0020/0021) y publicación de `v0.3.0`.
- `reference/v5.1/` es evidencia inmutable verificada por SHA-256. La UI React vive en `apps/web/` (`forge-face.ts`); el dominio está en `crates/archmaker-core` y se expone por CorePort.
- Arranque de adaptadores vía `apps/web/src/create-core-port.ts`: Desktop → TauriCorePort (`invoke` de `@tauri-apps/api/core`); navegador → WasmCorePort (`tools/build-wasm.sh`); MockCorePort solo con `?mock=1` u opt-in explícito, nunca por fallo silencioso de Tauri/WASM.
- ADR-0011 (propuesto, `in-review`): la selección del configurador produce un contrato JSON de aplicación; cada cláusula tiene schema en `contracts/json-schema/`, issue `kind:clause` y tests verificados en Rust. No añade target de imagen.
- Alcance declarativo Forge (draft): `docs/10-delivery/forge-declarative-scope.md`. ADR-0013…0018 (UKI ESP, estados ortogonales, handles, digests v2, ONLYOFFICE/navegadores, A6 excluida). No reescribe `mvp5-scope.md`. Mantener `executionStatus=not-executed` en ese alcance.
- Cadena de integridad Forge (selección → documento persistido → motor → resolución/validación → UI efectiva → export → reimportación). Registro: `docs/10-delivery/closure/forge-integrity-findings-2026-10-18.md`; matriz: `docs/10-delivery/closure/forge-acceptance-matrix.md`. Desde el 2026-10-18 la cara no autora `storagePlan`, filesystem ni cifrado de disco; un borrador importado con `storagePlan` no reabre el paso de disco. `executionStatus` de planes de disco permanece `not-executed`. TypeScript no inventa paquetes ni el perfil.
- En `apps/web` con Vite, la CSP (`style-src 'self'`) exige CSS por `<link>` a `/forge-face.css` (plugin Vite); inyectar `<style>` inline deja la cara sin estilos.
- La revisión auditada de arranque (UEFI / systemd-boot) vive en `docs/12-research/bootloader-review.md`.
- La referencia visual histórica de la cara vive fuera del repo, en `/home/alexendros/Aplicaciones/Repertorio/archmaker_neubat.html` (no en Descargas); el nombre de ese archivo se conserva.
- Archify es la herramienta externa de diagramación; no es el producto, el motor ni la GUI.
