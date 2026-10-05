# Especificación de operadores

## Tipos

`ValueRef`, `Literal`, `String`, `Boolean`, `Integer`, `Id`, `List<T>`.

| Operador | Firma | Resultado | Estado |
|---|---|---|---|
| `all` | `List<Expr>` | bool | required |
| `any` | `List<Expr>` | bool | required |
| `not` | `Expr` | bool | required |
| `eq` | `(T,T)` | bool | required |
| `ne` | `(T,T)` | bool | required |
| `in` | `(T,List<T>)` | bool | required |
| `required` | `ValueRef` | bool | proposed |
| `selected` | `Id` | bool | proposed |
| `count` | `ValueRef` | integer | proposed |
| `count_gt` | `(ValueRef,integer)` | bool | legacy-pending |
| `provides` | `Capability` | bool | proposed |
| `target_is` | `TargetId` | bool | proposed |

Los operadores desconocidos producen diagnóstico bloqueante; nunca se ignoran ni evalúan como false silenciosamente.
