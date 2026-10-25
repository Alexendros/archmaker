---
id: ADR-0014
phase: planning
priority: P0
documentStatus: superseded
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0013
---

# ADR-0014: Estados ortogonales de verificación y ejecución

- Estado: superseded
- Superseded by: ADR-0020

- Estado: proposed
- Fecha: 2026-10-17
- Propietario: Arquitectura

## Contexto

Mezclar «plan válido», «hardware visto», «arranque compatible», «capacidad suficiente» y
«ejecutado» produce falsos verdes.

## Decisión

Campos separados en el draft/manifiesto v2:

| Campo                        | Rol                                           |
| ---------------------------- | --------------------------------------------- |
| `planValidationStatus`       | Validación semántica del plan                 |
| `hardwareVerificationStatus` | Observación/confirmación de hardware          |
| `bootCompatibilityStatus`    | Compatibilidad de arranque                    |
| `capacityVerificationStatus` | Capacidad conocida y suficiente               |
| `executionStatus`            | Siempre `not-executed` mientras A6 esté fuera |

Ninguno se deduce solo de cambiar de pantalla.

## Consecuencias

- Un plan UEFI solo declarado puede exportarse con verificación pendiente.
- A6 no se marca cumplida al aceptar un plan.

## Verificación

- Tests de defaults fundacionales y walking skeleton.
