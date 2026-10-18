---
id: ADR-0019
phase: v1
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: not-started
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0003
    expectedDocumentStatus: accepted
  - id: ADR-0004
    expectedDocumentStatus: accepted
  - id: ADR-0010
    expectedDocumentStatus: accepted
  - id: ADR-0012
    expectedDocumentStatus: accepted
---

# ADR-0019: Target de perfil vivo y proceso de constitución

- Estado: accepted
- Fecha: 2026-10-18
- Aceptación: 2026-10-18
- Propietario: Arquitectura
- Requisitos: FR-ART-ISO-001, FR-FETCH-001, FR-ISO-001, NFR-MARK-001, NFR-SEC-001, NFR-OFF-001
- Decisiones: DEC-001, DEC-003, DEC-004, DEC-008, DEC-012, DEC-013

## Contexto

DEC-001 fija tres targets de export: `archmaker-profile`, `archinstall-profile` y `report`.
`docs/10-delivery/iso-scope.md` (ISO-01, aceptado el 2026-10-18) pide un cuarto target para el
perfil archiso de una imagen viva x86_64 UEFI, y prohíbe ampliar ese conjunto en silencio.

ADR-0004 describe el runner de instalación: proceso separado, operaciones allowlisted, elevación
fuera del WebView, capaz de un plan destructivo. Ese runner sigue cerrado (A6, root, discos,
shell arbitraria). La constitución de la imagen no es ese runner.

ADR-0010 y ADR-0012 fijan la marca. El fichero que descarga una persona no puede presentarse
como Arch Linux ni empezar por el signo ARCH. El nombre visible vigente es Forge.

## Drivers

- FR-ART-ISO-001: el manifiesto sellado produce `packages.x86_64` y `profiledef.sh`.
- FR-ISO-001: una corrida de `mkarchiso` y un digest ligado a ese manifiesto.
- NFR-SEC-001: el WebView no ejecuta shell ni root.
- NFR-OFF-001: sin red, el configurador sigue usable.
- DEC-008: el host de la imagen es Arch x86_64. aarch64 queda fuera.
- No reimplementar archiso ni importarlo desde el dominio Rust (ADR-0003).

## Opciones

1. Añadir el target dentro de `archmaker-core` llamando a `mkarchiso` desde el export.
2. Reabrir el runner de instalación de ADR-0004 para que también construya la imagen.
3. Añadir un target de perfil y un proceso de constitución distinto, con argv fijo y sin
   privilegio de disco.

## Decisión

Se adopta la opción 3 (2026-10-18, autoridad `architecture`, DEC-013).

| Ámbito                 | Valor                                                                                         |
| ---------------------- | --------------------------------------------------------------------------------------------- |
| Id técnico del target  | `archmaker-live-profile`                                                                      |
| Targets previos        | Siguen válidos. DEC-001 no se borra.                                                          |
| Fichero público        | `Forge-<prefijo-de-digest>-x86_64.iso`                                                        |
| Descripción de la base | «derivada de Arch Linux»                                                                      |
| Contenido del target   | `packages.x86_64` (un nombre por línea) y `profiledef.sh` con `arch=x86_64`                   |
| Prohibido en el perfil | `pacstrap`, strings `cmd`, copia de binarios no redistribuibles a `airootfs/`                 |
| Proceso                | Binario separado de constitución. No es el runner de instalación.                             |
| Operaciones            | Descarga tipada del conjunto del perfil, y una corrida de `mkarchiso`                         |
| Argv de `mkarchiso`    | Fijo: el binario externo y el directorio del perfil sellado. Sin script del operador.         |
| Botón de Revisión      | Pide `constitute_image`. No abre un shell ni ejecuta `mkarchiso`.                             |
| Sin red                | La cara sigue usable. La constitución falla con error tipado.                                 |
| Discos                 | Este proceso no particiona, no formatea y no escribe el disco del operador ni el del destino. |

El id técnico no es el nombre del fichero. El dominio Rust no enlaza archiso. El contrato de
código (`EXPORT_TARGETS`, schemas y `dto.md`) cambia en ISO-02, no en este corte documental.

La instalación en un SSD de prueba la hace archinstall dentro de la sesión viva. Forge no la
ejecuta.

## Consecuencias

- ISO-02 puede añadir `archmaker-live-profile` al contrato. Hasta ese cambio, exportar ese id
  sigue rechazado.
- ISO-03 y ISO-04 implementan el proceso. Si el host no tiene `mkarchiso`, la corrida queda
  `blocked` o `not_run`. No se fabrica una imagen de sustitución.
- ADR-0004 no se enmienda para autorizar escritura en discos.
- `executionStatus` de los planes de disco permanece `not-executed`. A6 no se reabre.

## Riesgos

- Confundir este proceso con el runner de instalación. Se mitiga con el id de operación y con
  la ausencia de comandos de disco en su allowlist.
- Un botón que construya una string de shell. Se mitiga con argv fijo y prueba negativa.
- Nombre de fichero con marca Arch. Se mitiga con el patrón `Forge-…-x86_64.iso`.

## Verificación

- Golden de ISO-02: mismos bytes, sin `pacstrap` ni `cmd`. Manifiesto bloqueante sin perfil.
- ISO-03: fixture local; nombre fuera del perfil rechazado; sin red, error tipado.
- ISO-04: `SHA256SUMS` ligado al manifiesto y arranque UEFI en VM, o `blocked` / `not_run`.
- El WebView no recibe una string `cmd`.

## Sustituye

—

## Sustituido por

—
