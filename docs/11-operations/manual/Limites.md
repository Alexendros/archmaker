# Límites

Texto canónico: [`docs/11-operations/manual/Limites.md`](https://github.com/Alexendros/archmaker/blob/main/docs/11-operations/manual/Limites.md).

## Sin red de contenidos

El MVP es usable sin CDN ni servidor (NFR-OFF-001). El catálogo y los drafts viven en local. Una comprobación de actualización firmada, cuando exista canal, no bloquea el uso offline (fail-open).

## Sin instalación del sistema

ArchMaker no particiona discos, no ejecuta `pacstrap`, no corre hooks ni scripts heredados y no escribe fuera del espacio de la aplicación. La imagen de instalación es otro alcance (`docs/10-delivery/iso-scope.md`), todavía no autorizado.

## Sin ejecución del plan

El runner privilegiado (JNY-003 / UC-006) está diferido (ADR-0004 / DEC-004). No hay confirmación de elevación ni journal de ejecución en esta baseline.

## Empaquetado

La **release de plan** publica notas y el código del tag. La **release de paquete** (AppImage, `.deb`, `.rpm`, AUR) está documentada en `docs/10-delivery/packaging-release.md` y bloqueada hasta que MVP-6 y el nombre público (ADR-0010) estén aceptados.

## Veredicto vigente

CONDITIONAL-CLEAR hasta MVP-4. MVP-5, MVP-6 e imagen no autorizan implementación mientras sus alcances no estén `accepted`.
