# Errores, diagnósticos y eventos

## CoreError

```json
{
  "code": "AM-CAT-003",
  "category": "catalog",
  "messageKey": "error.catalog.signatureInvalid",
  "params": {},
  "retryable": false,
  "correlationId": "uuid",
  "causeCode": null
}
```

## Diagnostic

```json
{
  "code": "AM-RES-004",
  "severity": "error",
  "blocking": true,
  "path": "/selections/environment.desktop",
  "messageKey": "diagnostic.capabilityMissing",
  "params": {},
  "source": "archmaker-resolver",
  "ruleId": "rule-id",
  "suggestions": []
}
```

## Eventos

`core.operation.started`, `core.operation.progress`, `core.operation.completed`, `catalog.loaded`, `draft.changed`, `resolution.changed`, `validation.completed`, `artifact.created`; v1 añade `runner.preflight.*`, `plan.*`, `execution.*`, `journal.*`.

## Familias

`AM-DOC`, `AM-SCHEMA`, `AM-CAT`, `AM-MIG`, `AM-RULE`, `AM-RES`, `AM-TGT`, `AM-IO`, `AM-PROTO`, `AM-RUN`, `AM-POL`, `AM-AUTH`.

## Catálogo canónico de códigos (fuente única)

Este documento es la **fuente única** de los códigos `AM-*`. Cualquier regla, validador o documento (`rule-inventory.md`, `operator-corpus.md`, `pipeline.md`) referencia este catálogo y no define códigos por su cuenta.

Convención `AM-<FAMILIA>-<NNN>`. La asignación numérica es estable y aditiva: un código no se reutiliza ni se renombra; un código retirado queda marcado `deprecated`. Los códigos listados están en estado `draft` (no son contrato `accepted`).

| Código | Familia | Mensaje | Severidad | Bloqueante | Origen |
|---|---|---|---|---|---|
| `AM-CAT-003` | AM-CAT | firma de catálogo inválida | error | sí | `CoreError` (ejemplo) |
| `AM-RES-004` | AM-RES | capability ausente | error | sí | `Diagnostic` (ejemplo) |
| `AM-RULE-001` | AM-RULE | cardinalidad de compositor excedida | error | sí | RULE-COMP-001 |
| `AM-RULE-002` | AM-RULE | derivación de `dm` aplicada | info | no | RULE-DM-001 |
| `AM-RULE-003` | AM-RULE | conflicto GPU/compositor sin resolución | error | sí (provisional) | RULE-GPU-001 |
| `AM-RULE-004` | AM-RULE | cardinalidad de kernel inválida | error | sí | RULE-KERNEL-001 |
| `AM-RULE-005` | AM-RULE | ausencia de navegador | warning | no | RULE-BROWSER-001 |
| `AM-RULE-006` | AM-RULE | disponibilidad de paquete no verificada | warning | no | RULE-PKG-001 |
| `AM-RULE-007` | AM-RULE | conflicto kernel/filesystem | error | sí | RULE-KERNEL-002 |
| `AM-RULE-008` | AM-RULE | identidad de paquete duplicada | error | sí | RULE-DUP-001 |
| `AM-RULE-009` | AM-RULE | operador de regla no soportado | error | sí | operator-corpus |

Las familias sin códigos asignados (`AM-DOC`, `AM-SCHEMA`, `AM-MIG`, `AM-TGT`, `AM-IO`, `AM-PROTO`, `AM-RUN`, `AM-POL`, `AM-AUTH`) se poblarán al implementar cada etapa de `pipeline.md`; hasta entonces, cualquier código de esas familias es `reserved`.
