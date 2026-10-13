---
id: DOC-DEL-MVP6-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: not-started
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0010
  - id: DOC-DEL-PKG-001
  - id: DOC-GOV-GNG-001
---

# Alcance MVP-6 — empaquetado del configurador

- Documento: DOC-DEL-MVP6-001 · Estado: in-review · Propietario: Release · Fecha: 2026-10-14
- Entrada: política de empaquetado en `docs/10-delivery/packaging-release.md` y residual de
  DEC-006 (firma definida, pipeline no aplicado). Cero releases en GitHub a 2026-10-14.
- Este borrador no autoriza subir paquetes ni abrir una ficha de tienda.

## Objetivo

Hacer instalable el configurador en las familias que el operador pidió, por el canal que la
ley de cada tienda permite a tarifa de alta cero, sin presentar el producto como Arch Linux
oficial.

La tienda instala la aplicación. La imagen de instalación de un sistema es otro alcance
(`docs/10-delivery/iso-scope.md`).

## Canales de la primera release verificable

| Familia               | Gestor                                  | Canal de esta fase                       | Tarifa de alta                                                     |
| --------------------- | --------------------------------------- | ---------------------------------------- | ------------------------------------------------------------------ |
| Debian, Ubuntu        | apt; Software o Discover si ven el repo | `.deb` construido en OBS                 | 0 € (<https://en.opensuse.org/openSUSE:Build_Service_FAQ>)         |
| Fedora                | dnf; Software o Discover                | `.rpm` en el mismo origen OBS            | 0 €                                                                |
| openSUSE              | zypper; Discover                        | `.rpm` en OBS                            | 0 €                                                                |
| Arch                  | pacman                                  | PKGBUILD en el AUR, solo x86_64          | 0 € (<https://wiki.archlinux.org/title/AUR_submission_guidelines>) |
| Las cinco, sin tienda | —                                       | AppImage firmado, además de los paquetes | hosting del binario                                                |

GNOME Software y Discover no son un formato. Muestran lo que apt, dnf, zypper, PackageKit o
Flatpak ya publican.

Quedan fuera del Done de este alcance:

- Archivos oficiales (Debian, Ubuntu archive, Fedora, Factory, `[extra]`). El review de Fedora
  no cobra y sí pide sponsor (<https://docs.fedoraproject.org/en-US/package-maintainers/Package_Review_Process/>).
- Flathub. Su política rechaza utilidades de host y apps atadas a una distro
  (<https://docs.flathub.org/docs/for-app-authors/requirements>). Un Flatpak que solo exporta
  un perfil puede pedirse después, y lo abre una persona: la política de octubre de 2026
  prohíbe que un agente abra ese PR.

## NFR-MARK-001 — Nombre público congelado

- Prioridad: P0
- Fase: MVP
- Estado: draft
- Journey/UC: —

### Enunciado

Ningún artefacto público (paquete, AppImage, metainfo, nombre de imagen) usa un signo que
empiece por ARCH ni el logo de Arch hasta que ADR-0010 esté accepted con permiso escrito o
con un nombre nuevo.

### Motivo

La política de 2021-04-18 pide permiso para las marcas que empiezan por ARCH. Publicar antes
arriesga una retirada.

### Criterios Given/When/Then

- **Positivo:** Given ADR-0010 accepted con vía de nombre cerrada, When se publica un paquete, Then el nombre visible es el que el ADR fijó y el texto dice «derived from Arch Linux» si habla de la base.
- **Negativo:** Given ADR-0010 todavía proposed, When un workflow de release corre, Then no sube ficha de tienda ni imagen.
- **Negativo:** Given un metainfo con el logo o el lema oficial de Arch, When se revisa el artefacto, Then se rechaza.

### Datos e interfaces

ADR-0010. Licencia de código Apache-2.0 (DEC-005), que no sustituye a la marca.

### Validación y seguridad

Revisión humana del diff de empaquetado. Sin capability `shell:*` (NFR-SEC-001, ADR-0002).

### Pruebas

Inspección del artefacto de CI: checksum SHA-256, SBOM y firma (cierre del residual DEC-006). Smoke de que el paquete se construye. La instalación en cada distro queda anotada como hecha o como no hecha.

### Exclusiones

No renombra crates. No envía correo a trademarks@ en nombre del operador.

## NFR-COST-001 — Coste medible del empaquetado

- Prioridad: P1
- Fase: MVP
- Estado: draft
- Journey/UC: —

### Enunciado

La release anota minutos de CI y tarifa aplicable. No afirma ser más barata que otro
constructor sin esa anotación.

### Motivo

GitHub no cobra los runners estándar en un repositorio público
(<https://docs.github.com/billing/concepts/product-billing/github-actions>). OBS, AUR y el
review de Fedora tampoco cobran alta. El coste real de esta fase es horas de persona y, si
algún día se usa un runner grande, la tarifa de ese runner (0,006 USD por minuto el Linux
2-core en repo privado, referencia tras el 2026-01-01; los runners grandes se cobran también
en repos públicos).

### Criterios Given/When/Then

- **Positivo:** Given un workflow de empaquetado en este repo público con runner estándar, When termina, Then el informe dice 0 USD de tarifa de Actions y los minutos medidos.
- **Negativo:** Given un job en runner grande, When termina, Then el informe anota minutos y tarifa vigente, y no los presenta como cero.
- **Negativo:** Given un texto de tienda que dice «más barato que archiso» sin la tabla de partidas, When se revisa, Then se rechaza.

### Datos e interfaces

Log del workflow. No hay nómina del proyecto: las horas se cuentan como horas, no se convierten a euros.

### Validación y seguridad

El informe no incluye secretos.

### Pruebas

Lectura del log de la primera release. Hasta que esa release exista, el requisito no está verificado.

### Exclusiones

No compara el precio de `mkarchiso`: esa corrida pertenece al alcance de ISO y no ahorra CPU.

## Issues

| #        | Issue                                     | Depende de | Gate   | Owner   | Criterio Done                                    |
| -------- | ----------------------------------------- | ---------- | ------ | ------- | ------------------------------------------------ |
| MVP-6-01 | Aceptar este alcance y ADR-0010           | —          | G2, G8 | product | Ambos `accepted`, con la vía de nombre cerrada   |
| MVP-6-02 | Pipeline real de SBOM, provenance y firma | 01         | G9     | release | DEC-006 deja de estar solo en política           |
| MVP-6-03 | `.deb`, `.rpm`, AppImage y PKGBUILD       | 02         | G10    | release | Checksums publicados; smoke de build en CI       |
| MVP-6-04 | Repo OBS y ficha AUR                      | 03         | G10    | release | Origen que apt, dnf, zypper y pacman pueden usar |

## Excluido

Runner, ISO, Flathub automático, archivos oficiales, telemetría, aarch64.
