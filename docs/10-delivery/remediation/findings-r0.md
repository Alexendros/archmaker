---
id: DOC-DEL-REM-R0-001
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

# R0 — Línea base y hallazgos (remediación operativa)

- SHA base: `af56a8912a33e22715a015188fde5a08adfcafa2` (`main`)
- Fecha: 2026-10-16
- Toolchain observada: rustc/cargo 1.90.0, Node v22.22.1, pnpm 11.21.0
- Veredicto: CONDITIONAL-CLEAR (sin runner/ISO/privilegios). MVP-6 sigue `in-review`.

## Hallazgos

### REM-H01 — Fallback silencioso a MockCorePort

```text
ID: REM-H01
SHA: af56a891…
Clasificación: confirmado
Pasos: abrir apps/web con Vite sin shell Tauri
Esperado: motor real (WASM) o error visible
Obtenido: MockCorePort sin aviso (main.tsx resolvePort)
Impacto: demostración tomada por producto
Prioridad: P0
Archivos: apps/web/src/main.tsx
Prueba: create-core-port no elige mock en producción
```

### REM-H02 — IPC vía global **TAURI**

```text
ID: REM-H02
SHA: af56a891…
Clasificación: confirmado
Pasos: inspeccionar tauri-core-port.ts; tauri.conf sin withGlobalTauri
Esperado: invoke desde @tauri-apps/api/core
Obtenido: window.__TAURI__.core.invoke
Impacto: escritorio puede caer al mock
Prioridad: P0
Archivos: apps/web/src/tauri-core-port.ts, src-tauri/tauri.conf.json
```

### REM-H03 — ACL incompleta / prueba ambigua

```text
ID: REM-H03
SHA: af56a891…
Clasificación: confirmado
Pasos: main.json permissions []; build.rs solo tauri_build::build();
       test security_runtime_deny_ungranted_command acepta "command"
Esperado: comandos propios en AppManifest + allow mínimo + denegación ACL clara
Obtenido: TOML core:* sin commands.allow; ACL generada sin create_draft
Impacto: denegación no demostrada de forma representativa
Prioridad: P0
Archivos: src-tauri/build.rs, capabilities/, permissions/, src/lib.rs
```

### REM-H04 — WASM sin sesión ni WasmCorePort

```text
ID: REM-H04
SHA: af56a891…
Clasificación: confirmado
Pasos: revisar archmaker-wasm lib.rs; búsqueda WasmCorePort
Esperado: instancia de sesión + adaptador UI
Obtenido: RealCore nuevo por llamada; sin WasmCorePort; sin wasm-pack en CI
Impacto: saveDraft/revisión no sobreviven entre llamadas; web de prod usa mock
Prioridad: P0
```

### REM-H05 — Manifiesto obsoleto / doble preparación

```text
ID: REM-H05
SHA: af56a891…
Clasificación: confirmado
Pasos: generar → editar → forjar sin regenerar
Esperado: entrega B o regeneración obligatoria
Obtenido: onForge reutiliza manifest de estado si existe
Impacto: artefacto A presentado como configuración B
Prioridad: P0
Archivos: apps/web/src/App.tsx
```

### REM-H06 — Exportación con token fijo

```text
ID: REM-H06
SHA: af56a891…
Clasificación: confirmado
Pasos: Forjar en UI
Esperado: descarga o destino real
Obtenido: destination.token = "opaque-handle.json"; toast de éxito sin bytes al usuario
Impacto: no hay entrega efectiva comprobada
Prioridad: P0
```

### REM-H07 — Catálogo embebido ≠ cara NEUBAT

```text
ID: REM-H07
SHA: af56a891…
Clasificación: confirmado
Pasos: comparar embedded-catalog.json vs STEP_IDS / local-catalog-mvp1
Esperado: misma identidad de catálogo
Obtenido: embedded minimal (archmaker.sys.*) vs cara (archmaker.base.*)
Impacto: validación real falla o mock oculta el desajuste
Prioridad: P0
```

### REM-H08 — Sin persistencia/reapertura en UI

```text
ID: REM-H08
SHA: af56a891…
Clasificación: confirmado
Pasos: buscar saveDraft/importDraft en App.tsx
Esperado: guardar y recuperar
Obtenido: createDraft al montar; sin save/import/open
Prioridad: P1 (ciclo esencial de perfiles)
```

### REM-H09 — «Sin incidencias» sin validar

```text
ID: REM-H09
SHA: af56a891…
Clasificación: confirmado
Pasos: abrir Revisión antes de Generar
Esperado: Pendiente de validar
Obtenido: «Sin incidencias.»
Prioridad: P1
```

## Criterio de salida R0

Todos los hallazgos anteriores clasificados. Ninguno se da por corregido en este documento; el cierre vive en los PR REM-*.
