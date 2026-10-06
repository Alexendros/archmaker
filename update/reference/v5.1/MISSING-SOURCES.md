# Disposición de fuentes declaradas

ID `REF-V5.1-DISPOSITION-001` · Estado `accepted` · Fecha 2026-10-06.

Este registro distingue entre evidencia recibida, alias materializado y fuentes históricas no recuperadas. La ausencia de una fuente declarada no se resuelve fabricando su contenido.

## Estado

| Fuente declarada             | Disposición                  | Sustituto permitido                                                                                                           | Efecto G0                                                                                       |
| ---------------------------- | ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `archmaker-v10.4-p3.yaml`    | `unavailable-declared-only`  | Ninguno dentro de `reference/`; usar los contratos actuales únicamente como diseño nuevo                                      | No bloquea G0 si la ausencia, impacto y prohibición de reconstrucción quedan registrados        |
| `i18n.es.json`               | `unavailable-declared-only`  | Ninguno dentro de `reference/`; crear un catálogo i18n nuevo fuera de la colección histórica cuando exista requisito aprobado | No bloquea G0; sí impide afirmar que se preservó el catálogo histórico                          |
| `neubat_forge_v4_final.html` | `probable-rename-not-proven` | `forge_v4_final.html`; alias byte a byte en `aliases/neubat_forge_v4_final.html`                                              | No bloquea G0 como referencia visual, pero no permite afirmar recuperación del nombre histórico |

## Evidencia HTML

- `forge_v4_final.html` fue recibido y preservado sin modificar.
- Su elemento `title` contiene `NEUBAT FORGE v4.1 - Kernel 7 - Enterprise`.
- El README histórico declara `neubat_forge_v4_final.html` como diseño original base.
- Ambos indicios hacen probable un cambio de nombre, pero no prueban identidad con un archivo histórico ausente.
- El alias materializado es una copia exacta de `forge_v4_final.html`; ambos deben producir el mismo SHA-256 registrado en `ALIASES.json`.

## Reglas

- No crear archivos con los nombres históricos ausentes simulando que son originales.
- No añadir las fuentes ausentes a `SHA256SUMS`, que cubre exclusivamente originales recibidos.
- Mantener los derivados y alias en subdirectorios explícitos.
- No usar valores técnicos del HTML como verdad actual de producto.
- Si aparece un original posterior, conservarlo con su hash, registrar procedencia y comparar antes de reemplazar esta disposición.

## Criterio G0

G0 evalúa control, integridad, procedencia y disposición explícita de fuentes; no exige recuperar material que nunca se recibió. Puede cerrarse con `SRC-001` en estado `controlled-incomplete` siempre que no exista una dependencia P0 que requiera semántica exclusiva de las dos fuentes no recuperadas.
