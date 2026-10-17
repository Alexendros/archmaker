---
id: ADR-0013
phase: planning
priority: P0
documentStatus: proposed
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0001
---

# ADR-0013: UKI en ESP (`uki-on-esp`)

- Estado: proposed
- Fecha: 2026-10-17
- Propietario: Arquitectura
- Requisitos: FR-MANIFEST-001, NFR-DET-001

## Contexto

El plan de disco declarativo necesita fijar dónde vive el artefacto de arranque sin introducir
XBOOTLDR ni asumir Secure Boot firmado.

## Drivers

- Separar artefacto de arranque de la política Secure Boot.
- Mantener ESP FAT32 montada en `/efi` fuera de LUKS2.
- No inventar particiones BIOS boot.

## Opciones

1. UKI en ESP (`bootArtifactStrategy = uki-on-esp`).
2. Entradas tradicionales de cargador sin UKI.
3. XBOOTLDR / segunda partición de arranque.

## Decisión

Opción 1. `bootArtifactStrategy` es `uki-on-esp`. Secure Boot se observa o declara por separado y
no certifica por sí solo la firma del cargador futuro. XBOOTLDR queda fuera.

## Consecuencias

- El `storagePlan` v2 exige esta estrategia cuando hay plan.
- La ayuda de systemd-boot/UKI explica límites UEFI; BIOS no certifica.

## Riesgos

- Confundir Secure Boot enabled con UKI firmado: mitigado por estados ortogonales (ADR-0014).

## Verificación

- Corpus draft/manifest v2 y tests de dominio.

## Sustituye/Sustituido por

Ninguno.
