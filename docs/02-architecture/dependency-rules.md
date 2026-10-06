---
id: DOC-ARCH-DEP-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Reglas de dependencia

- ID: DOC-ARCH-DEP-001
- Estado: accepted
- Propietario: Arquitectura
- Última revisión: 2026-10-05
- Requisitos relacionados: NFR-PORT-001, NFR-DET-001
- Sustituye / sustituido por: —

- El grafo debe ser acíclico.
- Dominio no conoce serialización externa salvo value types necesarios.
- Los DTO públicos no exponen tipos de infraestructura.
- UI solo conoce `CorePort` y view models.
- WASM y Tauri deben superar pruebas de paridad.
- El catálogo contiene intención y metadatos, nunca comandos ejecutables.
- Los exporters traducen manifest canónico a targets; no alteran el draft.
- El runner recibe un plan inmutable y operations enum.
- Enterprise implementa puertos opcionales; el core local no depende del control plane.
