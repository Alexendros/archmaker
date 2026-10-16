---
id: ADR-0010
phase: planning
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: partial
verificationStatus: partial
releaseStatus: eligible
owners:
  - product
reviewers:
  - independent-reviewer
---

# ADR-0010: Marca y nombre de artefacto público

- Estado: accepted
- Fecha: 2026-10-14
- Aceptación: 2026-10-16
- Propietario: Producto
- Requisitos: NFR-MARK-001, FR-EXPORT-001, DEC-005, DEC-008, DEC-011

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

No hay permiso escrito del Project Lead en el expediente. Inventar uno incumple NFR-MARK-001.
Por tanto la vía viable sin correo a trademarks@ es la opción 3.

## Drivers

- NFR-MARK-001: ningún artefacto público usa un signo que empiece por ARCH, ni el logo de Arch,
  hasta permiso escrito o un nombre nuevo aceptado en este ADR.
- DEC-008: el host de la imagen es Arch x86_64; eso no concede la marca.
- DEC-005: la licencia del código es Apache-2.0 y no sustituye a la política de marcas.
- Coste: retirar una release ya anunciada sale más caro que fijar el nombre visible ahora.

## Opciones

1. Seguir publicando el nombre actual en tiendas e imágenes, sin preguntar.
2. Congelar el nombre visible y el de la imagen hasta respuesta de `trademarks@archlinux.org`,
   y mientras tanto describir la imagen como derivada de Arch Linux, sin logo de Arch.
3. Elegir ya otro nombre público y dejar el id técnico de los crates como está.

## Decisión

Se adopta la **opción 3** (aceptada 2026-10-16, autoridad `product`, DEC-011):

| Ámbito                                                            | Valor                                                                       |
| ----------------------------------------------------------------- | --------------------------------------------------------------------------- |
| Nombre público (metainfo, UI visible, AppImage, notas de release) | **Neubat**                                                                  |
| Ids técnicos (crates, repo, contratos, catálogo)                  | Sin renombrar (`archmaker*`, rutas del repositorio)                         |
| Descripción de base OS cuando aplique                             | «derived from Arch Linux» / «based on Arch Linux»; sin logo ni lema oficial |

La opción 2 queda descartada para la primera release verificable porque exige permiso humano
escrito que no existe en el expediente y no puede inventarse. Una revisión futura puede
añadir un alias comercial distinto sin reabrir los ids técnicos, mediante nuevo ADR o
enmienda accepted de este.

## Consecuencias

- MVP-6 y el alcance de ISO citan NFR-MARK-001 con el nombre público **Neubat**.
- Los documentos internos pueden nombrar el repositorio. Un artefacto que el usuario descarga
  no se presenta como Arch Linux oficial ni usa el lema «A simple, lightweight Linux
  distribution».
- Los paquetes `-bin` no se incrustan en la imagen. Incluirlos aleja el resultado de un remix
  y acerca el conflicto de marca y de licencia.
- `productName` / título visibles del empaquetado deben decir Neubat; el id de crate y el
  identificador técnico pueden conservar `archmaker` hasta una migración voluntaria.

## Riesgos

- Confusión repo (`archmaker`) vs nombre público (`Neubat`): se mitiga en README y notas de
  release.
- Renombrar crates a la vez que la marca: coste de migración que este ADR deja fuera.

## Verificación

- Diff de empaquetado y metainfo público: ausencia de signo que empiece por ARCH, del logo de
  Arch y del lema oficial.
- Nombre visible del artefacto de la plataforma autorizada = Neubat.
- DEC-011 aceptada en el registro de decisiones.

## Sustituye

—

## Sustituido por

—
