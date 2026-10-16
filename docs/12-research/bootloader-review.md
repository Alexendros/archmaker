---
id: DOC-RES-BOOT-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: not-started
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-RES-HTML-001
    expectedDocumentStatus: in-review
  - id: ADR-0011
    expectedDocumentStatus: in-review
---

# Revisión auditada del arranque

- ID: DOC-RES-BOOT-001
- Estado: in-review
- Propietario: Arquitectura
- Captura: 2026-10-15
- Requisitos relacionados: FR-CAT-001, FR-EXPORT-001, NFR-OFF-001, CON-011
- Sustituye / sustituido por: —

Esta revisión responde qué gestor puede ofrecer el configurador en UEFI x86_64 y
cuál queda fuera de la cara del paso. No instala, no genera shell y no admite
formato de imagen. `CLAUSE-BOOT-001` sigue con `status` distinto de `satisfied`
(`contracts/json-schema/clause-boot.schema.json`).

## Cierre

En firmware UEFI, el instalador guiado de Arch (`archinstall`) preselecciona
**systemd-boot**. GRUB es el valor al que cae ese mismo instalador cuando la
máquina es BIOS. El canon ya guardado en este repo coincide con el caso UEFI:
los presets de `reference/v5.1/` y la primera opción de
`archmaker.base.boot` son `systemd-boot`.

La cara UEFI del configurador enseña solo `archmaker.boot.systemd-boot`.
GRUB sigue en el fixture local porque el id ya existe y porque archinstall lo
usa en BIOS; no se dibuja en el paso cuya cláusula solo admite `firmware: uefi`.
Quien necesite BIOS, temas o el lenguaje de `grub.cfg` no lo encuentra en esta
cara. Limine, rEFInd y EFISTUB tampoco se dibujan.

Ningún gestor ejecuta bash en el firmware. El criterio pedido — quedarse en
shell y no añadir el lenguaje de GRUB — lo cumple systemd-boot: la entrada es
clave-valor y el plugin que la escribe, en el tag leído, es `/bin/sh`.
GRUB mantiene además su propio lenguaje en `grub.cfg`.

No hay alcance con los nombres NOLANG ni NOIDIOTMATIC. Esta revisión es el tope
anterior a cualquier trabajo con esos nombres. Las cifras del HTML v5.1
(megabytes, «arranque en menos de un segundo», paquete `systemd-boot`) siguen
sin verificar y no entran en la cara.

## Hechos capturados

| claimId    | Hecho                                                                                                                                                       | Fuente                                                                                                      | Pin                                                                             | Estado del hecho |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | ---------------- |
| BOOT-C-001 | `Bootloader.get_default`: con UEFI devuelve `Systemd-boot`; sin UEFI devuelve `Grub`; con `skip_boot` devuelve `No bootloader`.                             | `archinstall/lib/models/bootloader.py`                                                                      | `archlinux/archinstall` `2c25540ad56b23922b9bbf44320b3154c0ecfd15` (2026-10-03) | verificado       |
| BOOT-C-002 | `BootloaderConfiguration.get_default` activa UKI cuando hay UEFI y el gestor declara soporte. En ese commit, todo gestor salvo «sin bootloader» lo declara. | mismo fichero                                                                                               | mismo pin                                                                       | verificado       |
| BOOT-C-003 | systemd-boot solo arranca ejecutables EFI desde la ESP en la que está instalado o desde XBOOTLDR del mismo disco.                                           | [ArchWiki: systemd-boot](https://wiki.archlinux.org/title/Systemd-boot)                                     | página viva, leída 2026-10-15                                                   | verificado       |
| BOOT-C-004 | La tabla del proceso de arranque marca GRUB y Limine con BIOS y UEFI; systemd-boot sin BIOS.                                                                | [ArchWiki: Arch boot process](https://wiki.archlinux.org/title/Arch_boot_process)                           | página viva, leída 2026-10-15                                                   | verificado       |
| BOOT-C-005 | El catálogo local carga `archmaker.boot.systemd-boot`, `archmaker.boot.grub` y `archmaker.boot.limine` (MVP-5-02).                                          | `crates/archmaker-core/fixtures/local-catalog-mvp1.json`, `docs/12-research/html-catalog-matrix.md`         | árbol de trabajo 2026-10-16                                                     | verificado       |
| BOOT-C-006 | Los presets embebidos en la evidencia v5.1 usan `boot: systemd-boot` en minimal-dev y full-workstation. El preset gaming usa `limine`.                      | `reference/v5.1/presets.json`, `reference/v5.1/App.tsx`                                                     | evidencia inmutable                                                             | verificado       |
| BOOT-C-007 | La cláusula de arranque del contrato solo admite `arch: x86_64` y `firmware: uefi`, y prohíbe `satisfied` mientras el formato de imagen no esté admitido.   | `contracts/json-schema/clause-boot.schema.json`, ADR-0011                                                   | árbol de trabajo                                                                | verificado       |
| BOOT-C-008 | systemd-boot viaja dentro del paquete `systemd`, dependencia de `base`. La wiki dice que no hace falta instalar otro paquete a mano.                        | [ArchWiki: systemd-boot](https://wiki.archlinux.org/title/Systemd-boot), sección Installation               | página viva, releída 2026-10-15                                                 | verificado       |
| BOOT-C-009 | `esp/loader/loader.conf` y cada `esp/loader/entries/*.conf` son pares clave-valor. No hay lenguaje de programa en el cargador.                              | misma página, sección Configuration                                                                         | página viva, releída 2026-10-15                                                 | verificado       |
| BOOT-C-010 | `90-loaderentry.install.in` empieza por `#!/bin/sh` y escribe la entrada tipo 1 en `loader/entries/`.                                                       | `systemd` tag `v259`, `src/kernel-install/90-loaderentry.install.in`                                        | releído 2026-10-15                                                              | verificado       |
| BOOT-C-011 | `kernel-install` ejecuta plugins `*.install`. `install.conf` es asignaciones `KEY=VALUE` compatibles con la sintaxis de shell.                              | [kernel-install(8)](https://man.archlinux.org/man/core/systemd/kernel-install.8.en)                         | manual vivo de Arch, leído 2026-10-15                                           | verificado       |
| BOOT-C-012 | Tabla de gestores: systemd-boot sin BIOS y con UEFI; GRUB y Limine con ambos; rEFInd solo UEFI; EFI stub y UKI no son un menú.                              | [ArchWiki: Arch boot process](https://wiki.archlinux.org/title/Arch_boot_process), tabla Feature comparison | página viva, releída 2026-10-15                                                 | verificado       |

Relectura de esta captura, 2026-10-15: `Bootloader.get_default` y
`BootloaderConfiguration.get_default` del pin `2c25540` coinciden con
BOOT-C-001 y BOOT-C-002. En UEFI el segundo deja `uki` activo porque
`has_uki_support` es verdadero para todo valor distinto de «No bootloader».
El ejemplo de `guided.rst` con `"uki": false` sigue sin ser esa función.
La guía manual de instalación enumera vías y no sustituye a `get_default`.

## Características

Lo que cada opción sabe hacer, según BOOT-C-003, BOOT-C-009 y BOOT-C-012.
Sin segundos de arranque y sin tamaños.

| Opción         | Menú | BIOS | UEFI | Dónde lee el kernel                                         | Configuración que ve quien la mantiene                                                                   |
| -------------- | ---- | ---- | ---- | ----------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| systemd-boot   | sí   | no   | sí   | ESP donde está instalado, o XBOOTLDR del mismo disco        | `loader.conf` y entradas clave-valor                                                                     |
| GRUB           | sí   | sí   | sí   | Según la tabla de la wiki, con sistemas de ficheros propios | Lenguaje de `grub.cfg`, además de los scripts que lo generan. Esta captura no reabrió la página de GRUB. |
| Limine         | sí   | sí   | sí   | Sistemas de ficheros limitados, según la tabla de la wiki   | `limine.conf`                                                                                            |
| rEFInd         | sí   | no   | sí   | Hereda del firmware; puede cargar drivers UEFI              | `refind.conf`                                                                                            |
| EFI stub / UKI | no   | no   | sí   | El firmware lanza el ejecutable EFI                         | Línea de órdenes embebida o pasada por otro gestor                                                       |

systemd-boot detecta UKI colocadas en `esp/EFI/Linux/` (BOOT-C-012). En el pin
de archinstall, UKI se enciende junto al gestor UEFI (BOOT-C-002). Eso no crea
un id `archmaker.boot.*` nuevo.

## Estabilidad

Aquí «estable» es quién lo mantiene y si el predeterminado del instalador
guiado sigue en el mismo sitio. No hay tasa de fallos medida en esta captura;
no se inventa.

- systemd-boot viaja dentro de `systemd`, y `base` ya depende de ese paquete
  (BOOT-C-008). El predeterminado UEFI de archinstall sigue siendo ese valor
  en el commit del 2026-10-03 (BOOT-C-001). La configuración clave-valor no
  ha cambiado de forma en la página releída (BOOT-C-009).
- GRUB sigue en el enum del mismo pin y es el predeterminado solo en BIOS.
  Cubre más firmware. Mantenerlo exige dos lenguajes. Para la cara UEFI de
  esta cláusula, eso lo deja fuera del paso.
- Limine está en el mismo enum: el modelo lo nombra en ese commit. No se
  releyó su instalador. El id `archmaker.boot.limine` ya está en el catálogo
  local (MVP-5-02).
- rEFInd está en el enum y en la tabla de la wiki. No está en la matriz HTML
  v5.1 ni en el catálogo. Fuera de esta selección.
- El paquete llamado `systemd-boot` en la matriz HTML no se toma como hecho:
  la wiki dice que no hace falta otro paquete (BOOT-C-008, CON-011).

## Posibilidades

Lo que esta revisión autoriza a dibujar cuando MVP-5-01 pase a `accepted`, y
lo que deja fuera:

- Cara UEFI x86_64: una sola tarjeta, `archmaker.boot.systemd-boot`.
- GRUB permanece en `local-catalog-mvp1.json`. No se borra el id. No se pisa
  en el paso UEFI. Una máquina BIOS no cabe en `CLAUSE-BOOT-001`.
- Limine ya está como `archmaker.boot.limine` en el catálogo local (MVP-5-02).
- UKI no se ofrece como tercer gestor. Si más adelante hace falta el flag,
  sale de Rust, no de un booleano inventado en TypeScript.
- El configurador no genera `loader.conf`, no llama a `bootctl` y no escribe
  shell. Eso sigue cerrado con el veredicto vigente.

## Bash

El firmware no tiene bash. La diferencia está en el lenguaje que queda
después de instalar:

- systemd-boot no interpreta un programa. `kernel-install` ejecuta plugins
  (BOOT-C-011). El plugin que escribe la entrada, en systemd `v259`, es
  `#!/bin/sh` (BOOT-C-010). Un gancho local en `/etc/kernel/install.d/` puede
  ser el mismo shell. No hace falta un segundo lenguaje en el arranque.
- GRUB genera configuración desde scripts y el cargador lee `grub.cfg`.
  Mantenerlo exige los dos. Esta captura no reabrió la página de GRUB; el
  doble lenguaje queda como hecho de la revisión anterior, no como cita nueva.
- Limine no habla bash: su configuración es `limine.conf`.
- rEFInd no habla bash: su configuración es `refind.conf`.

Por eso la opción que se queda en shell mejor que GRUB es systemd-boot. No es
un gestor que ejecute bash antes del kernel.

## Presentación en el configurador

Sin paleta nueva y sin tocar `reference/`. En la ventana web, los tres botones
del bloque Bootloader de ese HTML usan la `.option-card` que ya existía.
systemd-boot llega marcado, igual que el `useState` del HTML. GRUB y Limine
se eligen con el mismo botón. No hay caja nueva ni texto de megabytes.

```text
(●) systemd-boot     ● Seleccionada
( ) GRUB             ○ Sin seleccionar
( ) Limine           ○ Sin seleccionar
```

## Qué queda cerrado con esto

- La cara NEUBAT en `apps/web` conecta los tres radios de Bootloader:
  `systemd-boot` marcado al abrir el draft, `GRUB` y `Limine` al pulsarlos
  en modo Avanzado. El resto de huecos de catálogo sigue fuera del fixture
  Rust hasta MVP-5 (`docs/10-delivery/mvp5-scope.md`).
- Alta de Limine, presets y cualquier target de imagen esperan su alcance
  `accepted` y, para la imagen, un veredicto nuevo. ADR-0011 no se amplía aquí.
- NOLANG y NOIDIOTMATIC no son alcances. No se abre trabajo con esos nombres.

## Verificación

Hecho en esta captura: relectura del pin `2c25540`, de las páginas
systemd-boot y Arch boot process, del manual `kernel-install(8)` y del plugin
`90-loaderentry.install.in` en systemd `v259`.

Sigue abierto:

- Firma de un revisor independiente. El documento permanece `in-review`.
- `apps/web/mock-flow.playwright.test.ts` comprueba que la tarjeta visible
  es systemd-boot, que llega marcada, que GRUB no está en la cara y que el
  draft creado ya lleva `archmaker.boot.systemd-boot`.
