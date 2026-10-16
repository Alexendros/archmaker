---
id: DOC-DEL-MVP6-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0010
    expectedDocumentStatus: accepted
  - id: DOC-DEL-PKG-001
  - id: DOC-GOV-GNG-001
  - id: DOC-GOV-DEC-001
---

# Alcance MVP-6 — empaquetado del configurador

- Documento: DOC-DEL-MVP6-001 · Estado: **accepted** · Propietario: Release · Fecha: 2026-10-14
- Aceptación de canal: 2026-10-16 (DEC-011, ADR-0010 opción 3).
- Entrada: política de empaquetado en `docs/10-delivery/packaging-release.md` y residual de
  DEC-006 (firma definida; Etapa C parcial en F9 `v0.1.0`: SBOM/provenance/checksums;
  cosign local deferred).
- Primera publicación: tag [`v0.1.0`](https://github.com/Alexendros/archmaker/releases/tag/v0.1.0)
  (`RC_SHA` `9297ed7…`). Detalle: `closure/f9-release.md`.
- Este alcance autoriza **una sola plataforma inicial**. No anuncia cinco familias como Done.

## Objetivo

Hacer instalable el configurador en la plataforma autorizada, por el canal que la política
permite a tarifa de alta cero, sin presentar el producto como Arch Linux oficial. Nombre
visible: **Forge** (ADR-0012 / DEC-012; `v0.1.0` publicó Neubat bajo ADR-0010 / DEC-011).

La tienda instala la aplicación. La imagen de instalación de un sistema es otro alcance
(`docs/10-delivery/iso-scope.md`), sigue sin `accepted`.

## Plataforma autorizada (única)

| Campo                | Valor                                                                  |
| -------------------- | ---------------------------------------------------------------------- |
| Plataforma           | **AppImage Linux x86_64**                                              |
| Canal de publicación | GitHub Release del tag `v*` (workflow `.github/workflows/release.yml`) |
| Nombre visible       | Forge (histórico `v0.1.0`: Neubat)                                    |
| Arquitectura         | x86_64 (DEC-008)                                                       |
| Motivo de elección   | Ver «Por qué AppImage» más abajo                                       |

### Por qué AppImage (y no las otras filas del borrador)

1. `packaging-release.md` lista AppImage como primer formato MVP Linux portátil.
2. El borrador MVP-6 ya contemplaba «AppImage firmado, además de los paquetes» como vía sin
   tienda; es la única fila que no exige OBS, review de Fedora ni ficha AUR.
3. `release.yml` ya publica en GitHub Release con SBOM CycloneDX, checksums, cosign keyless y
   provenance SLSA v1 — el mismo acto de release que aloja el AppImage.
4. Autorizar `.deb`/`.rpm`/OBS/AUR ahora anunciaría varias familias; F8 exige **una**.
5. AUR queda **excluido** como repositorio binario de pacman (solo PKGBUILD fuente si se
   reabre en el futuro). Runner, ISO, root, discos, shell y Enterprise siguen fuera.

## Familias deferred / blocked

| Familia                      | Estado   | Motivo                                             |
| ---------------------------- | -------- | -------------------------------------------------- |
| `.deb` (Debian/Ubuntu / OBS) | deferred | Fuera de la plataforma única inicial               |
| `.rpm` (Fedora / openSUSE)   | deferred | Fuera de la plataforma única inicial               |
| PKGBUILD AUR (solo fuente)   | deferred | No autorizado en este accepted; nunca repo binario |
| Flatpak / Flathub            | blocked  | Política Flathub + fuera de alcance                |
| Archivos oficiales distro    | blocked  | Fuera de alcance                                   |
| ISO / runner / root / discos | blocked  | Veredicto CONDITIONAL-CLEAR hasta MVP-5            |

## NFR-MARK-001 — Nombre visible Forge

- Prioridad: P0
- Fase: MVP
- Estado: accepted (vía ADR-0010 opción 3 / DEC-011)
- Journey/UC: —

### Enunciado

Ningún artefacto público (AppImage, metainfo, notas de release) usa un signo que empiece por
ARCH ni el logo de Arch. El nombre visible es **Forge**. Los ids técnicos `archmaker*` no se
renombran en este alcance.

### Criterios Given/When/Then

- **Positivo:** Given ADR-0010 accepted con opción 3, When se publica el AppImage, Then el
  nombre visible es Forge y, si habla de la base OS, dice «derived from Arch Linux».
- **Negativo:** Given un metainfo con logo o lema oficial de Arch, When se revisa el artefacto,
  Then se rechaza.
- **Negativo:** Given un anuncio de cinco familias como Done de MVP-6, When se revisa el
  alcance, Then se rechaza.

### Exclusiones

No renombra crates. No envía correo a trademarks@ en nombre del operador.

## NFR-COST-001 — Coste medible del empaquetado

Sin cambio de enunciado respecto al borrador: la release anota minutos de CI y tarifa
aplicable (0 USD en runner estándar de repo público). Verificación en el log de la primera
release F9.

## Issues

| #        | Issue                                               | Depende de | Gate   | Owner   | Criterio Done                                           | Estado    |
| -------- | --------------------------------------------------- | ---------- | ------ | ------- | ------------------------------------------------------- | --------- |
| MVP-6-01 | Aceptar este alcance y ADR-0010 (opción 3)          | —          | G2, G8 | product | Ambos `accepted`; DEC-011; plataforma AppImage única    | **done**  |
| MVP-6-02 | Pipeline real de SBOM, provenance y firma (Etapa C) | 01         | G9     | release | DEC-006 deja de estar solo en política; F9              | **partial** (SBOM/provenance OK; cosign local deferred) |
| MVP-6-03 | AppImage Linux x86_64 + checksums                   | 02         | G10    | release | Artefacto publicado; smoke/checksums                    | **done** (`v0.1.0`; GUI ×3 not-verified)                |
| MVP-6-04 | Repo OBS / ficha AUR / `.deb` / `.rpm`              | —          | G10    | release | **deferred** — no parte del accepted de esta plataforma | deferred  |

## Excluido

Runner, ISO, Flathub automático, archivos oficiales, telemetría, aarch64, Enterprise, AUR como
repo binario, anuncio de cinco familias como Done.
