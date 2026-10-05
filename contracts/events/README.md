---
id: DOC-CON-EVENTS-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Catálogo de eventos

Los eventos públicos deben tener `eventVersion`, `eventId`, `correlationId`, `occurredAt`, `name`, `payload` y clasificación de datos. Los cambios breaking incrementan major. No incluir secretos ni paths innecesarios.
