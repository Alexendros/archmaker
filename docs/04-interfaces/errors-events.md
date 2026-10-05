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
