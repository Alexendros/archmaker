# Fixture de migración v5.1 → vNext

Material de corpus para el pipeline de `docs/03-data/versioning-migrations.md`. Estado `draft`.

| Archivo | Rol |
|---|---|
| `v5.1-instance.sample.json` | Recorte fiel de `reference/v5.1/archmaker.instance.v5.1.json`, sin campos ejecutables (`pacstrap`, `cmd`, hooks ni shell). Evidencia de entrada. |
| `vnext-draft.expected.json` | Destino esperado al validar contra `../draft.schema.json` (draft vNext con IDs globales DEC-002). |

## Casos obligatorios

| Caso | Dónde aparece en el sample | Resolución esperada |
|---|---|---|
| `mode: single` | `configurator.steps[1].mode = "single"` y `sections[].single = true` en `environment` | Cardinalidad única en la definición de dominio y `SelectionValue.kind = "single"` en el draft. |
| `count_gt` desconocido | `validation.rules[0].when.count_gt` | Se conserva como operador legacy acotado en `../rule.schema.json` (`count_gt`); el resto de operadores no soportados produce `AM-RULE-009` bloqueante. |
| Arrays en presets | `presets[1].selections.apps = ["steam","vlc"]` | Se transforma en `SelectionValue.kind = "multiple"` con `optionIds`; nunca se conserva un array donde el schema espera un escalar (CON-003). |
| `base-devel` duplicado | Sección `base-devel` y opción `base-devel` (CON-004) | Detección por identidad global `vendor.kind.id`; el draft resuelto contiene una única selección `archmaker.pkg.base-devel`. |
| `cmd` y hooks | No presentes (recorte saneado) | Rechazados como ejecución (CON-005, CON-006); no se migran a contratos runtime. |

## Notas

- El sample conserva los nombres de paquete (`pkg`) del legado únicamente como evidencia; los contratos nuevos no declaran paquetes ni versiones.
- `vnext-draft.expected.json` no incluye selecciones derivadas (p. ej. la derivación GNOME→GDM): el Draft almacena solo intención manual y referencias (DM-DRAFT).
- El original se conserva intacto; la migración produce un documento nuevo (NFR-MIG-001).
