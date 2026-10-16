---
id: DOC-DEL-CLO-UX-001
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-CLO-INV-001
    expectedDocumentStatus: draft
---

# F6 — Cara honesta (mensajes, rail, rendimiento)

- Documento: DOC-DEL-CLO-UX-001 · Estado: draft · Propietario: Arquitectura
- No rediseña la cara Forge. No toca capabilities de Tauri (dueño: F5).

## Seis pasos visibles (`RAIL_STEPS`)

El flujo del configurador ya tiene seis pasos en `apps/web/src/forge-face.ts` → `RAIL_STEPS`, renderizados por `ForgeRail` sin cambio de diseño:

| #   | id            | Título       |
| --- | ------------- | ------------ |
| 1   | `intent`      | Intención    |
| 2   | `base`        | Sistema base |
| 3   | `environment` | Entorno      |
| 4   | `apps`        | Apps         |
| 5   | `identity`    | Identidad    |
| 6   | `review`      | Revisión     |

Denominador: 6/6. F6 solo documenta este hecho.

## C-07 — tamaño y avisos

Corregido en F6:

- La suma de tamaño parte de `0` y solo acumula `size` de paquetes de catálogo seleccionados. No hay base fija de 650 MB.
- No se emite la advertencia UI «Sway con este kernel…». RULE-GPU-001 sigue no implementable en el motor; la cara no inventa ese diagnóstico.

## Mensajes de exportación

Coordinados con la entrega de bytes de F2 (`downloadArtifact` / `prepareDelivery`):

| Canal                       | Condición                                                     | Mensaje             |
| --------------------------- | ------------------------------------------------------------- | ------------------- |
| Web / mock / descarga `<a>` | Tras iniciar la descarga                                      | «Descarga iniciada» |
| Escritorio                  | Solo si hay confirmación de guardado (`confirmedSaveOutcome`) | «Archivo guardado»  |

`downloadArtifact` nunca devuelve `file-saved`. El camino nativo confirmado lo aportará F5; hasta entonces el escritorio que reutilice la descarga del webview también debe decir «Descarga iniciada».

## Teclado, foco y viewports

Ampliación de `a11y-playwright.test.ts`: viewports 360 / 768 / 1440 px y zoom 200 % con regiones live. axe sigue siendo complemento, no conformidad WCAG.

## Presupuesto de rendimiento (P95)

### Producto (congelado)

Umbrales de **producto** (web-ready / web-prepare; desktop-ready / desktop-prepare si hay GUI):

| Métrica | Umbral P95 | n | Estado |
| --- | --- | --- | --- |
| ready (Forge interactiva) | ≤ 3.000 ms | ≥ 30 | **web passed** — P95 123 ms (`product-p95-report.json`) |
| prepare (manifiesto válido) | ≤ 1.000 ms | ≥ 30 | **web passed** — P95 58 ms (mismo informe) |
| desktop-ready / desktop-prepare | mismos umbrales | ≥ 30 | **blocked** — faltan `desktop-ready-p95-samples`, `desktop-prepare-p95-samples`, AppImage Forge del SHA |

Máquina de la medición web: Ubuntu 26.04.1 LTS, Linux 7.0.0-38-generic x86_64,
AMD Ryzen AI 7 350 (16 CPUs lógicas), MemTotal 31421388 kB, Node v26.6.0,
perfil `vite-preview` + pkg WASM (sin `?mock=1`). Informe versionable:
`docs/10-delivery/closure/product-p95-report.json` (schema
`product-p95-report.schema.json`). Caso matriz: `F7-P95`. `--allow-pending` es
diagnóstico y **no** cierra aceptación. Una muestra inválida o ausente cuenta en el
denominador. La aceptación global de rendimiento exige web **y** desktop; desktop
`blocked` deja `acceptanceClosed=false` / `result=blocked`.

### Harness nativo (microbenchmark; no certifica producto)

`python3 tools/run_f7_p95_harness.py` es un **microbenchmark nativo**
(init/prepare del core sobre fixture embebido; `kind=microbenchmark`,
`certifiesProduct=false`). Comparte los mismos números de umbral como presupuesto de
laboratorio, pero **no certifica** el presupuesto de producto anterior. Las cifras del
microbenchmark viven fuera del árbol
(`~/archmaker-closure/evidence/<source-sha>/…/p95-report.json`). Un verde local del
microbenchmark no sustituye la aceptación independiente F7 ni cierra
web-ready / web-prepare / desktop-*.
