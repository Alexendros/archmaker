---
id: DOC-OPS-MAN-LIMITS
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

# Límites

Texto canónico: [`docs/11-operations/manual/Limites.md`](https://github.com/Alexendros/archmaker/blob/main/docs/11-operations/manual/Limites.md).

## Sin red de contenidos

El MVP es usable sin CDN ni servidor (NFR-OFF-001). El catálogo y los drafts viven en local. Una comprobación de actualización firmada, cuando exista canal, no bloquea el uso offline (fail-open).

## Sin instalación del sistema

Forge / ArchMaker (ids técnicos `archmaker*`) no particiona discos, no ejecuta `pacstrap`, no corre hooks ni scripts heredados y no escribe el disco del operador ni el del destino. La imagen viva está autorizada como alcance (`docs/10-delivery/build-scope.md`, ISO-01) y todavía no se constituye: el SSD lo instalará archinstall en la sesión viva.

## Sin ejecución del plan

El runner privilegiado (JNY-003 / UC-006) está diferido (ADR-0004 / DEC-004). No hay confirmación de elevación ni journal de ejecución en esta baseline.

## Empaquetado

La **release de plan** publica notas y el código del tag. **No hay release de paquete
publicada**: los tags `v0.1.0` y `v0.2.0` existen sin GitHub Release ni AppImage
descargable (corregido el 2026-10-25; `/releases` = `[]`). `v0.1.0` se declara no
publicado; la primera release real será `v0.3.0`. Limitaciones: cosign local y 3× install
GUI de aceptación pendientes; sin `RC_SHA_FUNCIONAL` F7; sin imagen Arch. El tip de `main` puede avanzar con docs y harness
sin mover el tag. `.deb`, `.rpm`, OBS y AUR quedan deferred; AUR no es repositorio binario.
Detalle: `docs/10-delivery/packaging-release.md`, `docs/10-delivery/closure/f8-governance.md` y
`docs/10-delivery/closure/f9-release.md`.

## Veredicto vigente

CONDITIONAL-CLEAR hasta MVP-5 (sin runner). MVP-6 AppImage está `accepted` para el canal
único y `v0.1.0` está publicado; ISO y runner siguen sin `accepted`.
