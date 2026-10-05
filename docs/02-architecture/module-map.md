---
id: DOC-ARCH-MOD-001
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

# Mapa de módulos

- ID: DOC-ARCH-MOD-001
- Estado: accepted
- Propietario: Arquitectura
- Última revisión: 2026-10-05
- Requisitos relacionados: NFR-PORT-001, NFR-DET-001
- Sustituye / sustituido por: —

| Crate/paquete | Responsabilidad | No puede depender de |
|---|---|---|
| `archmaker-domain` | Entidades, value objects e invariantes. | UI, Tauri, WASM, FS, red. |
| `archmaker-schema` | Schemas y meta-validación. | UI/infra. |
| `archmaker-catalog` | Carga, índice, procedencia. | Tauri/UI. |
| `archmaker-rules` | AST tipado y evaluación pura. | Infra. |
| `archmaker-resolver` | Capabilities, conflictos, ChangeSet. | UI. |
| `archmaker-validation` | Pipeline y diagnósticos. | UI. |
| `archmaker-migrations` | Transformaciones puras e informes. | Infra mutable. |
| `archmaker-export` | SPI de targets y artifacts. | Tauri. |
| `archmaker-api` | Casos de uso y puertos. | Implementaciones concretas. |
| `archmaker-wasm` | Adapter browser. | Tauri. |
| `archmaker-tauri` | Adapter desktop. | Runner internals. |
| `archmaker-plan` | Modelo de plan tipado. | Ejecución. |
| `archmaker-runner-protocol` | Mensajes/state machine. | UI. |
| `archmaker-runner` | Preflight/ejecución/journal. | WebView. |
