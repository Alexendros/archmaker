# Inventario de reglas

| ID | Fuente | Condición | Efecto heredado | Estado nuevo | Pruebas |
|---|---|---|---|---|---|
| RULE-COMP-001 | `validation-rules.json` | `count_gt(compositor,1)` | block | Formalizar cardinalidad en dominio; conservar diagnóstico | 0,1,2 selecciones |
| RULE-DM-001 | v5.1 | compositor=gnome | change dm=gdm | Derivación explicable/reversible según policy | GNOME/no GNOME/manual conflict |
| RULE-GPU-001 | v5.1 | gpu=nvidia AND compositor=sway | change sin target | Incompleta; evidence-required | NVIDIA/Sway permutations |
| RULE-KERNEL-001 | App.tsx | ningún kernel | block | Requisito de cardinalidad | none/one/multiple |
| RULE-BROWSER-001 | App.tsx | ningún browser | suggest | No bloqueante | none/one |

## Ficha obligatoria

Cada regla futura declara ID, fuente, tipos, condición, prioridad, efecto, severidad, blocking, diagnóstico, remedy, interacciones, test cases y versión de introducción/deprecación.
