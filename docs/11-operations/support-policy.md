---
id: DOC-OPS-SUP-001
phase: MVP
priority: P1
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
---

# Política de soporte

## Propuesta

- MVP: Arch Linux **x86_64** como único host soportado; navegador moderno soportado.
- aarch64: **fuera de alcance** (DEC-008). La evidencia aarch64 de `reference/v5.1/arch-info.json` es histórica, no un target soportado. Un futuro soporte exige nueva decisión, hardware/CI y catálogo verificado.
- v1: matriz explícita de live environment, archinstall/adapter, firmware y filesystems, limitada a x86_64.
- N-1 para documentos dentro del mismo major; majors antiguos mediante migradores publicados.
- Protocol runner: compatibilidad negociada; rechazar majors incompatibles.

## Fin de soporte

Aviso en release notes, migrador disponible y error accionable. Nunca abrir silenciosamente un documento con semántica desconocida.
