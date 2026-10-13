---
id: ADR-0010
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: not-started
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - product
reviewers:
  - independent-reviewer
---

# ADR-0010: Marca y nombre de artefacto público

- Estado: proposed
- Fecha: 2026-10-14
- Propietario: Producto
- Requisitos: NFR-MARK-001, FR-EXPORT-001, DEC-005, DEC-008

## Contexto

La política de marcas de Arch Linux, versión 2021-04-18
(<https://terms.archlinux.org/docs/trademark-policy/>), dice: «Any mark beginning with the
letters ARCH is sufficiently similar to one or more of the trademarks that permission will be
needed in order to use it.» Desaconseja nombres del estilo ArchMan o ArchTools para una
herramienta hecha para Arch. Un derivado con cambios sustanciales —otro kernel, paquetes fuera
de los repos oficiales, otro escritorio— puede decir «based on Arch Linux» o «derived from
Arch Linux», y no puede llamar al producto Arch Linux ni usar el logo. El uso comercial y
cualquier uso no listado piden licencia a `trademarks@archlinux.org`.

El id técnico del repo y de los crates ya usa ese prefijo. El HTML de `reference/v5.1/` mete
kernels alternativos y paquetes `-bin`. Publicar una tienda o una imagen con ese nombre, o con
el logo de Arch, antes de una respuesta escrita deja el artefacto en el supuesto que la propia
política marca como pendiente de permiso.

## Drivers

- NFR-MARK-001: ningún artefacto público usa un signo que empiece por ARCH, ni el logo de Arch,
  hasta permiso escrito o un nombre nuevo aceptado en este ADR.
- DEC-008: el host de la imagen es Arch x86_64; eso no concede la marca.
- DEC-005: la licencia del código es Apache-2.0 y no sustituye a la política de marcas.
- Coste: retirar una release ya anunciada sale más caro que congelar el nombre visible.

## Opciones

1. Seguir publicando el nombre actual en tiendas e imágenes, sin preguntar.
2. Congelar el nombre visible y el de la imagen hasta respuesta de `trademarks@archlinux.org`,
   y mientras tanto describir la imagen como derivada de Arch Linux, sin logo de Arch.
3. Elegir ya otro nombre público y dejar el id técnico de los crates como está.

## Decisión

Se adopta la opción 2 como propuesta, no como permiso de publicación. El id técnico en crates,
contratos y catálogo no se renombra en este ADR. La cadena visible en una ficha de tienda, en
el nombre de la imagen y en cualquier logo queda congelada hasta que ocurra una de estas dos
cosas: respuesta escrita del Project Lead, o una revisión accepted de este ADR que fije la
opción 3. No se abre un `DEC-*` accepted hasta esa aceptación: el registro de decisiones no
gana una fila nueva en este documento.

## Consecuencias

- MVP-6 y el alcance de ISO citan NFR-MARK-001 y no dan por hecho el nombre de la imagen.
- Los documentos internos pueden nombrar el repositorio. Un artefacto que el usuario descarga
  no puede presentarse como Arch Linux oficial ni usar el lema «A simple, lightweight Linux
  distribution».
- Los paquetes `-bin` no se incrustan en la imagen. Incluirlos aleja el resultado de un remix
  y acerca el conflicto de marca y de licencia.

## Riesgos

- Publicar antes de la respuesta: el Project Lead puede exigir el cambio con la release ya fuera.
- Renombrar crates a la vez que la marca: coste de migración que este ADR deja fuera.

## Verificación

- Ningún workflow de release escribe un nombre de imagen ni un metainfo de tienda hasta que
  este ADR pase a accepted con la opción 2 cerrada por correo o con la opción 3.
- Revisión del diff de empaquetado: ausencia de logo de Arch y del lema oficial.

## Sustituye

—

## Sustituido por

—
