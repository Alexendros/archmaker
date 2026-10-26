---
id: DOC-OPS-MAN-GUIDE
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
---

# Guía de usuario

Texto canónico: [`docs/11-operations/manual/Guia-de-usuario.md`](https://github.com/Soluciones-Alexendros/archmaker/blob/main/docs/11-operations/manual/Guia-de-usuario.md).

## Para quién

| Persona                    | Objetivo                                                                    |
| -------------------------- | --------------------------------------------------------------------------- |
| PER-001 — Usuario guiado   | Obtener una configuración segura sin aprender comandos.                     |
| PER-002 — Usuario avanzado | Controlar cada selección, comparar resultados y conservar reproducibilidad. |

Las personas PER-003 (maintainer de catálogo) y PER-004 (operador Enterprise) están documentadas en producto; no son el público del flujo MVP actual.

## Qué resultado obtiene

Al completar el flujo principal (JNY-001):

1. Un **draft** versionado con selecciones.
2. Una **resolución** reproducible de capacidades y conflictos.
3. Un **manifest** canónico (solo si no hay errores bloqueantes).
4. Un **artefacto** exportado con digests, versiones y target.

Al importar un documento heredado (JNY-002): un draft nuevo más un informe de migración, sin mutar el archivo original.

## Cómo empezar

Sigue el [Manual de uso](Manual-de-uso.md). Respeta los [Límites](Limites.md).
