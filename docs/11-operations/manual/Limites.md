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

Neubat (ids técnicos `archmaker*`) no particiona discos, no ejecuta `pacstrap`, no corre hooks ni scripts heredados y no escribe fuera del espacio de la aplicación. La imagen de instalación es otro alcance (`docs/10-delivery/iso-scope.md`), todavía no autorizado.

## Sin ejecución del plan

El runner privilegiado (JNY-003 / UC-006) está diferido (ADR-0004 / DEC-004). No hay confirmación de elevación ni journal de ejecución en esta baseline.

## Empaquetado

La **release de plan** publica notas y el código del tag. La **release de paquete** publicada
es el **AppImage Linux x86_64** con nombre público **Neubat** tag
[`v0.1.0`](https://github.com/Alexendros/archmaker/releases/tag/v0.1.0) (ADR-0010 opción 3,
DEC-011, MVP-6 accepted solo para esa plataforma). Comprueba el archivo con
`ARTIFACT_SHA256=6567464f5e2f21398fd3eef35592f0e9d377fd2cc7887c1e46df092b5a8e6d81` antes de
confiar en el binario. Limitaciones: cosign local y 3× install GUI de aceptación pendientes;
sin `RC_SHA_FUNCIONAL` F7; sin imagen Arch. El tip de `main` puede avanzar con docs y harness
sin mover el tag. `.deb`, `.rpm`, OBS y AUR quedan deferred; AUR no es repositorio binario.
Detalle: `docs/10-delivery/packaging-release.md`, `docs/10-delivery/closure/f8-governance.md` y
`docs/10-delivery/closure/f9-release.md`.

## Veredicto vigente

CONDITIONAL-CLEAR hasta MVP-5 (sin runner). MVP-6 AppImage está `accepted` para el canal
único y `v0.1.0` está publicado; ISO y runner siguen sin `accepted`.
