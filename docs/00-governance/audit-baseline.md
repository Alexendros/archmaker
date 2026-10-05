# Baseline de auditoría — planning-v1

ID `DOC-GOV-AUD-001` · Estado `accepted` · Propietario Gobierno · Fecha 2026-10-06.

## Propósito

Registrar de forma inmutable el estado de partida de la remediación `planning-v1 → planning-v1.1`, los hallazgos que la motivan y su resolución esperada. Este documento es la autoridad de la baseline congelada; los estados de avance se leen de sus fuentes canónicas, no de este texto.

## Baseline congelada

| Dato | Valor |
|---|---|
| Etiqueta de baseline | `planning-v1-audit-baseline` |
| Commit | `cf2fa70` |
| Rama de trabajo | `remediation/planning-v1.1` |
| Hito | `planning-v1.1-remediation` (#1) |
| Perfil de arranque | P1 (`init-work.sh`) |
| Semáforo de arranque | rojo (por falso positivo de secreto; ver AUD-F-05) |

## Declaración de parada (semáforo rojo)

`init-work.sh` marcó **P0: posible secreto** en `reference/v5.1/App.tsx:180` e `reference/v5.1/index.html:203`. Inspección directa: ambas líneas contienen el token de diseño CSS `{ token: "--rh-red", hex: "#ee0000", oklch: "oklch(0.62 0.24 29)", usage: "primary action" }`. **Clasificación: falso positivo** — el escáner interpreta `token:` como asignación de credencial. No hay secreto y no se requiere rotación. Acción derivada: allowlist del patrón `token:` de tokens de diseño en el gate de escaneo (AUD-022).

## Hallazgos

| ID | Severidad | Hallazgo | Resolución |
|---|---|---|---|
| AUD-F-01 | P0 | Contradicción de estado: `ADR-0007` en `proposed` mientras G3/G4 se declaran `complete`. | R3 (AUD-006) + R5 (AUD-009) |
| AUD-F-02 | P0 | Estados unidimensionales en gates/decisiones: no separan documento-completo, diseño-listo, implementación-lista, verificado y liberación-lista. | R1 (AUD-002/003) |
| AUD-F-03 | P0 | Backlog con filas 1–14 sin esquema `AUD-*`, sin owners ni dependencias explícitas. | R0 (AUD-001) |
| AUD-F-04 | P0 | Integridad de fuentes: 3 fuentes declaradas no recibidas (`archmaker-v10.4-p3.yaml`, `i18n.es.json`, `neubat_forge_v4_final.html`). | Disposición G0 (AUD-007) |
| AUD-F-05 | P1 | Falso positivo de escaneo de secretos sobre `token:` de diseño en `reference/`. | R11 (AUD-022) |
| AUD-F-06 | P0 | CI en estado `draft` con guards silenciosos (`PENDIENTE … guard`) y TODOs; sin ejecución verde registrada. | R11 (AUD-022) |
| AUD-F-07 | P0 | Registro de decisiones: DEC-005 (licencia) y DEC-006 (firma) ambiguas; DEC-009/010 en `proposed`. | R2 (AUD-004) |
| AUD-F-08 | P0 | Vista C4 muestra Rust Core como contenedor, contradiciendo su propio deployment. | R4 (AUD-007/008) |
| AUD-F-09 | P0 | Canonicalización sin las 12 decisiones normativas; RFC 8785 (JCS) sin fuente primaria verificada. | R5 (AUD-009/010) |
| AUD-F-10 | P0 | `RULE-GPU-001` no implementable (depende de SRC-002/003). | R8 (AUD-017) |
| AUD-F-11 | P0 | Faltan schemas comunes (`common`, `core-error`, `changeset`) y el directorio `contracts/governance/`. | R6 (AUD-011…016) |
| AUD-F-12 | P1 | `neubat_forge_v4_final.html` con identidad `probable-rename-not-proven`; `PROVENANCE.json` no reflejaba la disposición. | Disposición G0 (AUD-007) |

## Disposición G0 de fuentes (resumen)

Aplicada la actualización `archmaker-g0-reference-update`: `SRC-001` pasa de `received-partial` a `controlled-incomplete`; los once originales recibidos permanecen inmutables bajo `SHA256SUMS`; dos fuentes quedan `unavailable-declared-only` (sin reconstrucción); el HTML NEUBAT se materializa como alias byte a byte en `reference/v5.1/aliases/`. Autoridad: `docs/00-governance/G0-SOURCE-DISPOSITION.md`. Verificación: `sha256sum --check reference/v5.1/SHA256SUMS` (11/11 OK), `cmp` alias (idéntico), validación JSON de `ALIASES.json` y `UNAVAILABLE-SOURCES.json`.

## Fuera de alcance de esta remediación

El tablero P1 detecta carencias de estándar de repositorio (p. ej. `CHANGELOG.md`, `.github/workflows/ci.yml`, `.github/renovate.json`, `Makefile`, `docs/README.md`, meta-sección «Propósito de este documento», dependabot→Renovate). Quedan registradas como observaciones de estándar de repositorio; su corrección es independiente de los gates G0–G10 de esta remediación.
