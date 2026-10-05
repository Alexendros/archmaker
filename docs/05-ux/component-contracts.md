---
id: DOC-UX-COMP-001
phase: MVP
priority: P1
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - ux
reviewers:
  - independent-reviewer
---

# Contratos de componentes

Cada componente interactivo debe documentar: propósito, props, slots, variantes, estados, teclado, ARIA, focus, tokens, responsive behavior, reduced motion, empty/loading/error y tests.

## OptionCard

- Estados: idle, hover, focus-visible, selected-manual, selected-derived, required, locked-policy, incompatible, disabled, warning.
- Semántica: radio si cardinalidad uno; checkbox si múltiple; nunca simular ambos con `div` sin rol.
- Acción secundaria: mostrar evidencia, dependencias y motivo de derivación.

## StepRail

- Estados: incomplete, current, valid, warning, blocked, complete.
- Teclado: navegación natural de links; no roving tabindex salvo patrón demostrado.
- Responsive: rail lateral en ancho amplio; stepper compacto en móvil.

## ValidationPanel

- Agrupa por severidad y step.
- Enlace de cada diagnóstico al control afectado.
- Región viva solo para resúmenes; no anunciar en masa cada revalidación.
