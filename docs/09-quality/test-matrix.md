---
id: DOC-QLT-TEST-001
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

# Matriz de pruebas

| Área | Unit | Property | Golden | Contract | E2E | Fuzz/Fault |
|---|---:|---:|---:|---:|---:|---:|
| Domain | ✓ | ✓ |  |  |  |  |
| Schema | ✓ |  | ✓ | ✓ |  | ✓ |
| Rules/resolver | ✓ | ✓ | ✓ | ✓ |  | ✓ |
| Migrations | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Exporters | ✓ |  | ✓ | ✓ | ✓ |  |
| Tauri/WASM | ✓ |  | ✓ | ✓ | ✓ |  |
| UI/a11y | ✓ |  | visual |  | ✓ |  |
| Runner | ✓ | ✓ | ✓ | ✓ | VM | ✓ |
| Enterprise | ✓ |  |  | ✓ | ✓ | fault |

## Golden profiles

Minimal UEFI/ext4; GNOME/Btrfs; Hyprland/AMD; Sway/NVIDIA conflict; gaming/multilib; dev/Podman; v5.1 migration; aliases; catálogo manipulado; target incompatible.

## Alcance de plataforma

Todos los perfiles y pruebas se ejecutan sobre **Arch Linux x86_64** (DEC-008, 2026-10-05). No se define matriz aarch64; un futuro soporte requeriría decisión, hardware/CI y catálogo verificado.
