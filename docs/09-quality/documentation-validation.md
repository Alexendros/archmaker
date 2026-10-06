---
id: DOC-QLT-DOCVAL-001
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

# Validación documental

## Gates automatizables

- Markdown lint y enlaces internos.
- Mermaid parse/render.
- JSON/YAML parse.
- JSON Schema meta-validation.
- Fixtures válidos e inválidos.
- IDs únicos por namespace.
- Referencias de trazabilidad existentes.
- `sha256sum --check reference/v5.1/SHA256SUMS`.
- Prohibición de URLs CDN en código de producción.
- Prohibición de `cmd`, hooks y shell en contracts runtime.

## Resultado

Los validadores deben devolver código distinto de cero y diagnóstico localizado; no corregir silenciosamente documentación aceptada.
