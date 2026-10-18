---
id: DOC-DEL-ISO-001
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
  - id: ADR-0010
  - id: ADR-0003
    expectedDocumentStatus: accepted
  - id: ADR-0004
    expectedDocumentStatus: accepted
  - id: DOC-RES-HTML-001
  - id: DOC-GOV-GNG-001
  - id: ADR-0019
    expectedDocumentStatus: accepted
---

# Alcance de imagen — perfil, descarga y constitución

- Documento: DOC-DEL-ISO-001 · Estado: accepted · Propietario: Arquitectura · Fecha: 2026-10-14
- Aceptación: 2026-10-18 (ISO-01). Solo imagen viva x86_64 UEFI.
- Fase de producto: v1. Este alcance no autoriza root, discos (A6) ni shell arbitraria.
- La línea de `docs/00-governance/go-no-go.md` (ISO-01) es la compuerta del exporter y de
  `mkarchiso`. El contrato de código del target llega en ISO-02 (ADR-0019 / DEC-013).

## Objetivo

Un manifiesto sellado produce un perfil archiso declarativo. Un proceso de constitución,
distinto del runner de instalación (ADR-0004), ejecuta una sola corrida de `mkarchiso` y
publica el digest. archiso no se reimplementa. `pacstrap` no es el constructor. El dominio
Rust no importa archiso (ADR-0003).

La imagen aceptada es la viva x86_64 UEFI. El criterio de arranque se llama LiveISO: si la imagen arranca en una VM UEFI, el criterio queda cubierto. Instalarla en un SSD es prueba
del archinstall que ya va en esa sesión, no una escritura de Forge. aarch64 sigue fuera.

Estructura de perfil, según <https://wiki.archlinux.org/title/Archiso> y
`README.profile.rst` del proyecto archiso: `airootfs/`, arranque, `packages.x86_64` (un
nombre por línea), `pacman.conf`, `profiledef.sh` con `arch=x86_64`. El nombre de la imagen
no dice Arch Linux (ADR-0010, NFR-MARK-001). aarch64 queda fuera (DEC-008) aunque el HTML lo liste.

## FR-ART-ISO-001 — Perfil declarativo desde el manifiesto

- Prioridad: P0
- Fase: v1
- Estado: draft
- Journey/UC: JNY-001 / UC-001

### Enunciado

Dado un manifiesto sellado, el exporter escribe un perfil archiso: `packages.x86_64` con los
nombres de paquete redistribuibles, y `profiledef.sh` como configuración generada, no como
script que el usuario pegue.

### Motivo

Hoy una persona edita el perfil a mano por cada variante. El manifiesto ya es el digest común.
El ahorro está en esa autoría, no en la CPU de la corrida.

### Criterios Given/When/Then

- **Positivo:** Given un manifiesto sin errores bloqueantes, When se exporta el perfil, Then `packages.x86_64` lista esos nombres, uno por línea, y el fichero no contiene `pacstrap` ni `cmd`.
- **Positivo:** Given el mismo manifiesto, When se exporta dos veces, Then los bytes del perfil coinciden.
- **Negativo:** Given una opción de clase `no-redistribuible-candidato`, When se exporta, Then su binario no se copia a `airootfs/` y el perfil solo puede nombrar la obtención diferida.
- **Negativo:** Given un manifiesto con diagnóstico bloqueante, When se exporta, Then no hay perfil.

### Datos e interfaces

`DM-MANIFEST`, `DM-ARTIFACT`, target `archmaker-live-profile` (ADR-0019 / DEC-013), distinto de `archmaker-profile`, `archinstall-profile` y `report`. El id técnico no es el nombre del fichero público (`Forge-<prefijo-de-digest>-x86_64.iso`). El match en `EXPORT_TARGETS` es ISO-02; este corte no lo añade.

### Validación y seguridad

VAL del exporter: rechazo de líneas con shell. THR-IMP-001. El dominio no enlaza archiso.

### Pruebas

Golden del perfil a partir de un manifiesto fixture. Inspección de que `pacstrap` no aparece.

### Exclusiones

No ejecuta `mkarchiso`. No llama a archinstall. No instala en disco.

## FR-FETCH-001 — Descarga tipada

- Prioridad: P0
- Fase: v1
- Estado: draft
- Journey/UC: JNY-001

### Enunciado

La obtención de paquetes del espejo es una operación tipada del runner: conjunto cerrado de
nombres salidos del manifiesto, digest cuando el espejo lo da, error tipado si no hay red.

### Motivo

El configurador tiene que seguir usable sin red (NFR-OFF-001). La constitución de la imagen
no puede disfrazarse de string `cmd` ni de `pacstrap` en la UI (CON-005, CON-012).

### Criterios Given/When/Then

- **Positivo:** Given un perfil sellado y red disponible, When el runner descarga, Then cada paquete pedido está en el conjunto del perfil y el journal cita el digest.
- **Negativo:** Given el mismo perfil sin red, When el configurador arranca, Then la UI opera y la constitución falla con error tipado, sin colgar el proceso.
- **Negativo:** Given un nombre que no está en el perfil, When se pide descargarlo, Then el runner lo rechaza.
- **Negativo:** Given un paquete `-bin` no redistribuible, When se constituye la imagen, Then el blob no viaja dentro de `airootfs/`.

### Datos e interfaces

`DM-PLAN`, `DM-SESSION`, transporte ya decidido en DEC-004 y todavía no implementado. Operación nueva del runner, no un shell libre.

### Validación y seguridad

THR-RUN y NFR-SEC-001. Pruebas con fixture local. Ningún test llama a `pacstrap` en el puesto del operador.

### Pruebas

Fixture de espejo local. Caso sin red. Caso de nombre fuera de conjunto.

### Exclusiones

No abre el disco del host. No eleva privilegios dentro del WebView (ADR-0004).

## FR-ISO-001 — Una corrida y un digest

- Prioridad: P0
- Fase: v1
- Estado: draft
- Journey/UC: JNY-001

### Enunciado

El runner, en proceso separado, ejecuta una corrida de `mkarchiso` sobre el perfil sellado y
publica la imagen x86_64 UEFI más `SHA256SUMS`.

### Motivo

archiso ya construye la imagen. Volver a escribir ese constructor no baja el coste de CPU.
Lo que este requisito añade es el digest ligado al manifiesto y la prueba de arranque en VM.

### Criterios Given/When/Then

- **Positivo:** Given un perfil sellado, When la corrida termina, Then existe un fichero de imagen cuyo SHA-256 está en `SHA256SUMS` y el nombre no usa la marca de Arch (NFR-MARK-001).
- **Positivo:** Given la imagen, When una VM UEFI x86_64 arranca, Then llega al entorno vivo previsto por el perfil.
- **Negativo:** Given ADR-0010 sin aceptar, When alguien pide la corrida, Then no se publica imagen.
- **Negativo:** Given el pipeline del HTML, When se genera el perfil, Then la etapa `pacstrap` no se copia.

### Datos e interfaces

Perfil de FR-ART-ISO-001. Herramienta externa `mkarchiso`, citada y no vendada. Host de build: Arch x86_64.

### Validación y seguridad

El botón de Revisión no ejecuta un shell ni llama a `mkarchiso`. Pide la imagen con la operación tipada `constitute_image`, que no recibe ruta ni script. La corrida de `mkarchiso` del 2026-10-18 fue una vez, en una VM, sobre el perfil sellado. El proceso no particiona. Sin telemetría (DEC-010).

### Pruebas

Job de CI o de runner desechable, más evidencia de arranque UEFI. Minutos y tarifa anotados según NFR-COST-001. En repo público y runner estándar la tarifa de Actions es 0 USD; un runner grande se anota aparte.

### Exclusiones

aarch64. Instalación en el disco del operador. Reimplementar archinstall. Llamar a la imagen Arch Linux.

## Coste de este lado

La corrida de `mkarchiso` es la misma que la de un perfil a mano: no hay ahorro de CPU ni de
espejo. El ahorro es el número de ficheros que una persona edita: cero, porque `packages.x86_64`
sale del manifiesto. archinstall sigue siendo el instalador; el sobre experimental ya existente
no se convierte en otro instalador.

## Issues

| #      | Issue                                                                    | Depende de | Gate        | Owner        | Criterio Done                               |
| ------ | ------------------------------------------------------------------------ | ---------- | ----------- | ------------ | ------------------------------------------- |
| ISO-01 | Aceptar este alcance, ADR-0019 y una reevaluación del veredicto          | —          | G2, G8, G10 | product      | `go-no-go.md` nombra el slice (2026-10-18)  |
| ISO-02 | Target de export del perfil, con corpus, sin ampliar DEC-001 en silencio | 01         | G4, G5      | architecture | Golden sin `pacstrap` ni `cmd`              |
| ISO-03 | Operación de descarga tipada con fixture                                 | 02         | G8          | security     | Rechazo sin red y rechazo de nombres ajenos |
| ISO-04 | Corrida `mkarchiso` en runner separado y prueba de VM                    | 03         | G10         | release      | Imagen + SHA-256 + arranque UEFI anotado    |

ISO-01 documental está aceptado. ISO-02 admite el target `archmaker-live-profile` (perfil, no imagen). Siguen los tres targets de DEC-001. ISO-03 descarga ese conjunto desde un fixture local en el proceso `archmaker-constitute`; sin fixture responde `constitution-offline` y no abre red. ISO-04 se ejecutó el 2026-10-18 dentro de una VM Arch x86_64: `Forge-aaa81a961b0a-x86_64.iso`, SHA-256 en `SHA256SUMS`, arranque UEFI hasta el shell de la sesión viva. Ese arranque cubre el criterio LiveISO (orden del operador, 2026-10-18). SSD sigue `not_run`. Registro: [`closure/iso04-mkarchiso-not-run.md`](closure/iso04-mkarchiso-not-run.md). P6 y F7 no se marcan passed.
