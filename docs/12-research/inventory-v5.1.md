# Inventario consolidado v5.1

| Campo | Valor |
|---|---|
| ID | DOC-RES-INV-001 |
| Título | Inventario consolidado de artefactos v5.1 |
| Estado | draft |
| Propietario | Arquitectura/Investigación |
| Última revisión | 2026-10-05 |
| Fuente | `reference/v5.1/*` (SRC-001, `received-partial`) |
| Sustituye / sustituido por | — |

> Este informe no altera `reference/`. Todo valor entrecomillado o citado es copia fiel de los bytes recibidos. Los datos no presentes en la evidencia se marcan **no verificado — fuente primaria pendiente**.

## 1. Inventario de artefactos

Verificación de integridad ejecutada sobre el contenido real del directorio (SHA-256 recalculado) y contrastada con `PROVENANCE.json`, `SHA256SUMS` y `SHA256-VERIFY.txt`.

**Conteo real:** `reference/v5.1/` contiene **16 archivos** (`readdirSync`, solo ficheros). El enunciado de la tarea declara "17 archivos"; esa cifra **no se corresponde con el contenido en disco y queda como no verificada**. El manifest versionado (`PROVENANCE.json:5-83`) cubre 11; los 5 restantes son metadatos del propio bundle.

### 1.1 Artefactos presentes (16)

| # | Archivo | Bytes | SHA-256 | mediaType | Estado |
|---:|---|---:|---|---|---|
| 1 | `App.tsx` | 81977 | `1c649703dc67d2c4c977d6140061d3fdf175f8ee06b1a1cadf97839a42014fd4` | `application/octet-stream` | received-unmodified (manifest, MATCH) |
| 2 | `arch-info.json` | 582 | `bd9e3be80848742a308e33849c4e885c890ec918e51cb0a9df99a5064445b0a7` | `application/json` | received-unmodified (manifest, MATCH) |
| 3 | `archmaker.instance.v5.1.json` | 15954 | `7e24779787479fc7235969b61114a6d9ab518712a8e6031ea979436a5d4e25ef` | `application/json` | received-unmodified (manifest, MATCH) |
| 4 | `archmaker.schema.original.json` | 16443 | `e4542b88dc57afe362fca8fc73e245fdd0a7010df4157682e220b46bdc217cdd` | `application/json` | received-unmodified (manifest, MATCH) |
| 5 | `archmaker.schema.v5.1.json` | 6975 | `6d40c7d19ea8d2988e6e5b747b1f97cadda9d74d0492071b58ffd3a862b8af0e` | `application/json` | received-unmodified (manifest, MATCH) |
| 6 | `forge_v4_final.html` | 35288 | `02e37d4be7dbb894c7751cc0a2da5f946b755a41a1e58b74363f854661bb1426` | `text/html` | received-unmodified (manifest, MATCH) |
| 7 | `index.html` | 84253 | `6e08055891c89f3480032cb4ab443220be6a8aee538666b13aca93f39e39f8cf` | `text/html` | received-unmodified (manifest, MATCH) |
| 8 | `libpacks.json` | 3448 | `01725c753f4cf1d2903db8a80621d3c56b785bfb48ec30bfe5cb5eef32ea7c09` | `application/json` | received-unmodified (manifest, MATCH) |
| 9 | `presets.json` | 708 | `b810dc64add4c23d5d6f30529ba9c6de2c5d7351f8b875ec23d31cf82c3522f5` | `application/json` | received-unmodified (manifest, MATCH) |
| 10 | `README.md` | 2480 | `3ec9801be46f10e9d514d549f9b47c3220e545d9b8938f732323de8cc17e50e3` | `text/markdown` | received-unmodified (manifest, MATCH) |
| 11 | `validation-rules.json` | 986 | `f013dbef741bec2c8a342411ce5fcc5165e7ce639e207fbdf751dbd53d73331d` | `application/json` | received-unmodified (manifest, MATCH) |
| 12 | `INVENTORY.md` | 471 | `eb4cec3aaa583a1699cc2a22025e49964044b6de742d68ec7e9e3a780a52ffed` | `text/markdown` *(inferido)* | presente; fuera de manifest |
| 13 | `MISSING-SOURCES.md` | 203 | `c3e9a56f1d8deb28bb859ef2c1bcaf80ba0c2629d20f6fdba3ad1e1812c73821` | `text/markdown` *(inferido)* | presente; fuera de manifest |
| 14 | `PROVENANCE.json` | 2756 | `071e801eff0446f07d9b4fb2228732c9e24dcf3d170479b74f85e2a350a2f26a` | `application/json` *(inferido)* | presente; fuera de manifest |
| 15 | `SHA256-VERIFY.txt` | 244 | `21986a622ee9ed0d37a134b42231bee05deaf1eb8ccc7d0bb104d09b5b34b48c` | `text/plain` *(inferido)* | presente; fuera de manifest |
| 16 | `SHA256SUMS` | 926 | `a4422d72ed7408630375cad1d42243bc3c00b81a4f817f0f4f6d989ea6435178` | `text/plain` *(inferido)* | presente; fuera de manifest |

Los 11 hashes del manifest coinciden exactamente con los recalculados; `SHA256-VERIFY.txt:1-11` declara `OK` para esos 11. `INVENTORY.md:3` afirma "Los siete JSON parsean": hay 8 ficheros `.json` en disco, de los cuales 7 son de catálogo (los cubiertos por el manifest) y `PROVENANCE.json` es metadato; la afirmación es consistente si se refiere a los 7 de catálogo.

### 1.2 Declarados y ausentes (3)

Fuente: `PROVENANCE.json:84-88` (`declaredButMissing`) y `MISSING-SOURCES.md:1-5`.

| Archivo declarado | Estado | Observación |
|---|---|---|
| `archmaker-v10.4-p3.yaml` | ausente | "no recibido." (`MISSING-SOURCES.md:3`) |
| `i18n.es.json` | ausente | "no recibido." (`MISSING-SOURCES.md:4`) |
| `neubat_forge_v4_final.html` | ausente | "nombre no recibido; no asumir que sea `forge_v4_final.html`." (`MISSING-SOURCES.md:5`) |

`README.md:8` declara `neubat_forge_v4_final.html` como "Diseño original base (referencia)" y `PROVENANCE.json` recibió `forge_v4_final.html` (35288 bytes, `text/html`). No se asume equivalencia (CON-008).

## 2. Duplicación detectada

### 2.1 Catálogo

El catálogo del configurador está triplicado:

- `archmaker.instance.v5.1.json:51-502` — `configurator.steps`, con secciones que usan la clave `options`.
- `App.tsx:4-195` — constante embebida `INSTANCE` (`App.tsx:4`), copia del catálogo, pero las secciones usan la clave `packages`.
- `index.html:27-218` — copia literal del `INSTANCE` de `App.tsx`, también con `packages`.

Evidencia estructural (`instance.section` keys = `id,name,single,options,auto_logic,essential,recommended`; claves de sección en `App.tsx` = `packages`):

- El schema v5.1 solo define `packages` en `$defs/section` (`archmaker.schema.v5.1.json:149-173`, propiedades `id,name,single,packages`).
- El schema original define `options` (`archmaker.schema.original.json:482-544`, propiedades `id,name,multi,options,fields,groups,hooks`).
- La instancia usa `options` (formato del schema **original**), no `packages` (formato del schema v5.1). Ambos schemas aceptan la instancia porque carecen de `additionalProperties:false` (ver §3.4).

Duplicación adicional de la capa de librerías: `libpacks.json:1-155` (objeto raíz `{id,name,icon,description,sections}`) reproduce las mismas secciones `office-pack,dev-pack,gaming-pack` ya embebidas en `archmaker.instance.v5.1.json:236-389` (step `libpacks`) y en el `INSTANCE` de `App.tsx`.

### 2.2 Reglas

- `validation-rules.json:1-56` y `archmaker.instance.v5.1.json:504-559` son **idénticos** (verificado por comparación JSON): mismas 3 reglas `single-compositor`, `gdm-gnome`, `nvidia-sway`.
- `App.tsx:169-176` define un conjunto **divergente**: `single-compositor` (con `target:"compositor"` y nota "selección única activada"), `kernel-required`, `browser-required`, `gdm-auto`.
- `index.html` reproduce el mismo conjunto divergente de `App.tsx`.
- Además, `App.tsx` reimplementa la validación en React en `App.tsx:318-344` (`useMemo`), y la regla `single-compositor` queda **hardcoded `active: false`** en `App.tsx:331` — nunca puede dispararse. `validation-rules.json`/instancia usan `when.count_gt` (`:9-17`), operador ausente de ambos schemas.

### 2.3 Presets

- `presets.json:1-42` y `archmaker.instance.v5.1.json:577-618` son **idénticos** (verificado): `minimal`, `desktop`, `dev`.
- `App.tsx:190-194` (y `index.html:213-217`) declara otros tres: `minimal-dev`, `full-workstation`, `gaming`, con selecciones mediante la clave `pkgs` (arrays) en lugar de `selections` escalares.
- Colisión nominal explícita: `minimal` (instancia/presets.json) vs `minimal-dev` (`App.tsx:191`).
- CON-003: los presets usan arrays (`apps`, `dev-pack`, `pkgs`) donde el schema original solo admite escalares (`archmaker.schema.original.json:773-782`, `selections.additionalProperties` tipo `string|boolean|number`).

## 3. Inconsistencias de IDs y estructura

### 3.1 `arch`: longitud 1 en instancia vs 2 en UI

- Instancia: `"arch": ["x86_64"]` (`archmaker.instance.v5.1.json:6-8`).
- `App.tsx`: `arch: ["x86_64", "aarch64"]` (`App.tsx:5`; idéntico en `index.html:28`).
- El schema v5.1 admite ambos valores (`archmaker.schema.v5.1.json:31-41`), sin cardinalidad.
- La UI expone selector y modal de dos arquitecturas (`App.tsx:414`, `App.tsx:910-969`).
- `forge_v4_final.html` no menciona `x86_64` ni `aarch64` (0 coincidencias).

### 3.2 Presets `minimal` vs `minimal-dev`

Ver §2.3. `minimal` no existe en los datos embebidos de UI; `minimal-dev` no existe en `presets.json` ni en la instancia.

### 3.3 `base-devel` duplicado

Colisión de identificador en la instancia: `base-devel` aparece **2 veces** en el espacio de IDs de sección/opción:

- como `id` de sección: `archmaker.instance.v5.1.json:123` (`"id": "base-devel"`, nombre "Base Devel").
- como `id` de opción dentro de esa sección: `archmaker.instance.v5.1.json:127` (`"id": "base-devel"`, `pkg: "base-devel"`).

En `App.tsx` la sección se renombra a `base-tools` (`App.tsx:29`), eliminando esa colisión, pero `base-devel` reaparece en `FIXED_IDS` (`App.tsx:201`) y en el estado inicial `selectedOthers` (`App.tsx:211`). Corresponde a CON-004 y exige el detector de duplicados de DEC-002.

Discrepancias `id` ≠ `pkg` detectadas (mismo patrón en instancia y `libpacks.json`):

| Ubicación | `id` | `pkg` |
|---|---|---|
| `instance/compositor` | `cosmic` | `cosmic-epoch` |
| `instance/office-pack/essential` y `libpacks/office-pack/essential` | `hunspell-es` | `hunspell-es_es` |
| `instance/gaming-pack/essential` y `libpacks/gaming-pack/essential` | `lib32-vulkan` | `lib32-vulkan-icd-loader` |

`App.tsx` usa `lib32-vulkan-icd-loader` en `FIXED_IDS` (`App.tsx:201`) en lugar del `id` `lib32-vulkan` de los JSON.

### 3.4 Schemas abiertos vs cerrados

Hecho por bytes (propiedad `additionalProperties` explícita o ausente; ausente = abierto por defecto en JSON Schema):

- Schema v5.1: **raíz sin `additionalProperties`** (`archmaker.schema.v5.1.json:5-10`), `$defs/option` sin `additionalProperties` (`:109-148`), igual que `section`, `step`, `libpack`, `rule`, `logic` y `preset`. El único nodo cerrado explícito es `iso` con `additionalProperties:true` (abierto).
- Schema original: **raíz sí declara `additionalProperties: true`** (`archmaker.schema.original.json:10`); `$defs/option` también `true` (`:432`). Sin embargo, el original sí cierra explícitamente `$defs/logic` (sus `oneOf` con `additionalProperties:false`: `:638, :653, :665, :680, :695, :710`) y `$defs/hooks` (`additionalProperties:false`: `:753`).
- **Corrección de premisa:** "v5.1 abierto vs original cerrado" es exacto para `logic` y `hooks`, pero **no** para la raíz ni para `option`, donde el original también es abierto. Se registra la lectura precisa; la caracterización global del original como "cerrado" queda **no verificada / matizada**.
- Diferencia de operadores: el original incluye `ne` (`:682-711`); v5.1 no (`archmaker.schema.v5.1.json:263-330`, solo `all,any,not,eq,in`). `count_gt` no existe en ninguno de los dos (CON-002).

## 4. Propiedades abiertas / ejecutables peligrosas

| Hallazgo | Evidencia (bytes) | Clasificación |
|---|---|---|
| Campo `cmd` | `archmaker.schema.v5.1.json:144-146` (`"cmd": {"type":"string"}`). Presente en el schema v5.1; **ausente** en `$defs/option` del original (verificado). Uso real: `App.tsx:19` `linux-hardened ... cmd: "pacman -S linux-hardened"` y `index.html:42`. | **CONSERVAR-EVIDENCIA / NO-MIGRAR-A-RUNTIME** |
| Strings `pacstrap` | `arch-info.json:6` y `:12` (`"pacstrap": "pacstrap /mnt base linux ..."`, `"... linux-aarch64 ..."`); instancia `:15` y `:21`; `App.tsx:963`; comando generado `App.tsx:313-315` (`pacstrap /mnt base ${pkgs} --needed`). | **CONSERVAR-EVIDENCIA / NO-MIGRAR-A-RUNTIME** |
| `build.pipeline` con `pacstrap` | Instancia `:568-575` (`["validate","resolve","derive_palette","pacstrap","assemble","checksum"]`); `App.tsx:189` e `index.html:212` idénticos; el enum del schema original incluye `pacstrap` (`archmaker.schema.original.json:219-231`). | **NO-MIGRAR-A-RUNTIME** (convertir a plan tipado) |
| Hooks `pre`/`post` como arrays de strings | `archmaker.schema.original.json:737-754` (`$defs/hooks`); referenciados por `option` (`:428-430`), `section` (`:539-541`) y `step` (`:571-573`). No aparecen en el schema v5.1 ni en `App.tsx`/`index.html` (0 coincidencias). | **CONSERVAR-EVIDENCIA / NO-MIGRAR-A-RUNTIME** |

Referencias de gobierno: CON-005 (`cmd`), CON-006 (hooks), CON-012 (`pacstrap`), y la prohibición de `AGENTS.md:21` ("No ejecutar strings heredados `cmd`, hooks, `pacstrap` ni shell").

## 5. Lógica de dominio en React, datos embebidos y estilos

- **Reglas implementadas en React en vez del core:** `App.tsx:318-344` recalcula validaciones (`kernel-required`, `single-compositor` con `active:false` en `:331`, `browser-required`, `gdm-auto`). El auto-bloqueo GDM también vive en el cliente: `useEffect` en `App.tsx:229-234` y `handleDMSelect` en `:308-311`. Contradice `AGENTS.md:22` ("Rust es la única autoridad semántica; TypeScript no duplica reglas"). Ver CON-009.
- **Datos embebidos en `App.tsx`:** catálogo, paleta, reglas, presets y build dentro de `INSTANCE` (`App.tsx:4-195`), replicado en `index.html:27-218`. CON-009.
- **Estilos inline / tokens no normalizados:** bloque `<style>` embebido en `App.tsx:389-401`; `:root`/`.dark` con variables `--rh-*` (`App.tsx:391-392`, `index.html:11-12`). Tokens en datos (`App.tsx:180-187`, `index.html:203-209`) con `hex` + `oklch`. Uso masivo de utilidades Tailwind arbitrarias (`text-[10px]`, `rounded-[6px]`, `bg-[var(--rh-bg)]`): 224 coincidencias de `--rh-` en `App.tsx` y 227 en `index.html`; `forge_v4_final.html` usa valores hex literales en clases arbitrarias (`bg-[#ee0000]`, `:115, :126, :144, ...`) 41 coincidencias.
- **Fuentes remotas / CDN:** `App.tsx:390` (`@import url('https://fonts.googleapis.com/...')`); `index.html:7` (`https://cdn.tailwindcss.com`), `:8` (Google Fonts), `:20-22` (`unpkg.com` React, ReactDOM, Babel); `forge_v4_final.html:7-8, :21-23` idénticos. Ver CON-010, RSK-006 y DEC-009.

## 6. Afirmaciones técnicas que requieren verificación oficial

Todas las siguientes se registran con marca **no verificado — fuente primaria pendiente** (SRC-002 ArchWiki y SRC-003 archinstall están `pending-capture`; `INVENTORY.md:9` y CON-011 lo confirman).

| Afirmación | Evidencia |
|---|---|
| Kernel "7.1.2 estable" | `archmaker.instance.v5.1.json:73`; `App.tsx:17`; `index.html:40` |
| "7.1-zen baja latencia" | `archmaker.instance.v5.1.json:83`; `App.tsx:18` |
| Hardened "+0 MB extra, ASLR, stack protector" | `App.tsx:19` |
| Tamaños 180/185/182 (MB) | `archmaker.instance.v5.1.json:69,79,89`; `App.tsx:17-19` |
| Hyprland 0.47 | `App.tsx:50` |
| GNOME 50 | `App.tsx:51` |
| Sway 1.10 | `App.tsx:52` |
| COSMIC Epoch 1.0 | `App.tsx:53` |
| AUR aarch64 "~85% (box64)" | `App.tsx:943` |
| Batería "4-6h" vs "10-18h" | `App.tsx:946` |
| Apple Silicon / Raspberry Pi 5 / Snapdragon X Elite "soportado" | `App.tsx:947-949` |
| Kernel 7.1 / "6.12.25 LTS" en prototipo previo | `forge_v4_final.html:239-241` |
| Formas `pacstrap` por arquitectura | `arch-info.json:6,12` |

## 7. Clasificación final por campo

| Campo/artefacto | Clasificación | Nota |
|---|---|---|
| `archmaker` metadatos (`name,version,date,deploy,iso`) | migrable | verificar versión/fecha |
| `archmaker.arch` `["x86_64"]` | conservable | ampliar requiere DEC-008 |
| `arch_info.pacstrap` | conservar-evidencia / no-migrar-a-runtime | ejecución prohibida |
| `palette.tokens` (`--rh-*`) | migrable | normalizar a tokens canónicos |
| `configurator` con clave `options` | deprecado | schema v5.1 usa `packages` |
| `configurator` con clave `packages` (UI) | migrable | reconciliar con instancia |
| `cmd` | rechazado (runtime) / conservable (evidencia) | CON-005 |
| hooks `pre`/`post` | rechazado (runtime) / conservable (evidencia) | CON-006 |
| `build.pipeline` con `pacstrap` | rechazado (runtime) | CON-012 |
| `validation.rules` (`count_gt`) | migrable con bloqueo | formalizar cardinalidad (CON-002) |
| `gdm-gnome` / `gdm-auto` | migrable | derivación reversible |
| `nvidia-sway` | evidence-required | acción sin `target` |
| `presets` de `presets.json`/instancia | migrable | definir `SelectionValue` (CON-003) |
| presets `minimal-dev`/`full-workstation`/`gaming` (UI) | deprecado | IDs divergentes |
| `archmaker.schema.v5.1.json` | deprecado como contrato | abierto; incorpora `cmd` |
| `archmaker.schema.original.json` | conservable como evidencia | histórico v10.4 |
| `libpacks.json` | migrable | colisiona con instancia |
| `App.tsx` / `index.html` | rechazado como runtime / conservable como evidencia UX | prototipo CDN |
| `forge_v4_final.html` | conservable como evidencia | distinto de `neubat_forge_v4_final.html` |
| CDN y fuentes remotas | rechazado | DEC-009, RSK-006 |
| Reglas reimplementadas en React (`active:false`) | rechazado | duplican el core |

## 8. Trazabilidad a gobierno y reglas

| Hallazgo | Contradicción | Decisión | Regla afectada |
|---|---|---|---|
| `mode: single` fuera del schema original | CON-001 | — | — |
| `count_gt` no definido | CON-002 | — | RULE-COMP-001 |
| Arrays en presets | CON-003 | — | — |
| `base-devel` duplicado | CON-004 | DEC-002 | — |
| Campo `cmd` | CON-005 | — | — |
| Hooks `pre`/`post` string | CON-006 | — | — |
| Fuentes declaradas y no recibidas | CON-007 | — | — |
| `neubat` vs `forge_v4_final` | CON-008 | — | — |
| Catálogo/reglas/UI embebidos | CON-009 | DEC-003 | RULE-KERNEL-001, RULE-BROWSER-001 |
| Dependencia de CDN | CON-010 | DEC-009 | — |
| Afirmaciones sin fuente | CON-011 | — | — |
| `pacstrap` en pipeline | CON-012 | DEC-003 | — |
| IDs globales/duplicados | CON-004 | DEC-002 | — |
| `gdm-gnome` / `gdm-auto` | — | — | RULE-DM-001 |
| `nvidia-sway` sin target | — | — | RULE-GPU-001 |
| Cardinalidad de compositor | CON-002 | — | RULE-COMP-001 |

Reglas citadas: inventario en `docs/07-validation/rule-inventory.md:5-9`. Decisiones: `docs/00-governance/decision-register.md`. Contradicciones: `docs/00-governance/contradiction-register.md`.
