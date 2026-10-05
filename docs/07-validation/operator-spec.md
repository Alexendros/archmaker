---
id: DOC-VAL-OPS-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - quality
reviewers:
  - independent-reviewer
---

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
| `required` | `ValueRef` | bool | required |
| `selected` | `Id` | bool | proposed |
| `count` | `ValueRef` | integer | proposed |
| `count_gt` | `(ValueRef,integer)` | bool | legacy (solo migración) |
| `provides` | `Capability` | bool | proposed |
| `target_is` | `TargetId` | bool | proposed (diferido a v1) |

`required` queda **ratificado como operador estable** el 2026-10-05.

`count_gt` es un operador **legacy**: la migración v5.1 lo reescribe a `count` con umbral. **No se ejecuta en el runtime nuevo**; se conserva solo para migrar documentos heredados.

`target_is` queda **diferido a v1**; no se usa en el MVP.

Los operadores desconocidos producen diagnóstico bloqueante; nunca se ignoran ni evalúan como false silenciosamente.
