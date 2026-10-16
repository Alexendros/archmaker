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
- No rediseña la cara NEUBAT. No toca capabilities de Tauri (dueño: F5).

## Seis pasos visibles (`RAIL_STEPS`)

El flujo del configurador ya tiene seis pasos en `apps/web/src/neubat-face.ts` → `RAIL_STEPS`, renderizados por `ForgeRail` sin cambio de diseño:

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

### Producto (congelado; pendiente de medición)

Umbrales de **producto** (web-ready / web-prepare; desktop-ready / desktop-prepare si hay GUI):

| Métrica | Umbral P95 | n | Estado |
| --- | --- | --- | --- |
| ready (Forge interactiva) | ≤ 3.000 ms | ≥ 30 | **pending** — sin medición de producto |
| prepare (manifiesto válido) | ≤ 1.000 ms | ≥ 30 | **pending** — sin medición de producto |

Máquina de referencia de producto: **no fijada** (pendiente de registro al medir). Sin
informe de producto con n≥30 no se declara rendimiento `passed`.

### Harness nativo (no certifica producto)

`python3 tools/run_f7_p95_harness.py` (caso `F7-P95`) es un **microbenchmark nativo**
(init/prepare del core sobre fixture embebido). Comparte los mismos números de umbral
como presupuesto de laboratorio, pero **no certifica** el presupuesto de producto
anterior. Las cifras del harness viven fuera del árbol
(`~/archmaker-closure/evidence/<source-sha>/…/p95-report.json`). Un verde local del
harness en una máquina concreta no sustituye la aceptación independiente F7 ni cierra
web-ready / web-prepare / desktop-*.
