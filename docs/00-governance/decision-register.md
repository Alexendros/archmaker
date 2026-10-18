---
id: DOC-GOV-DEC-001
phase: planning
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - governance
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-GOV-STATUS-001
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
---

# Registro de decisiones

Autoridad única de las decisiones de gobierno (fase R2, issue `AUD-004`). Los documentos
consumidores referencian los identificadores `DEC-*` y **no** reafirman su estado (regla R2 de
`remediation-plan-v1.1.md`). El vocabulario de estados y roles se rige por
`status-model.md` (DOC-GOV-STATUS-001).

## Convenciones del registro

- Estado de decisión: `accepted` (vigente) · `superseded` (reemplazada por una sucesora) ·
  `rejected` · `proposed` (pendiente de autoridad).
- Autoridad: rol de la tabla ROLES de `docs/00-governance/remediation-plan-v1.1.md`.
- Toda decisión `accepted` declara fecha y autoridad; una decisión `superseded` apunta a su
  sucesora.
- Cada decisión enlaza los ADR, requisitos, riesgos y gates afectados.
- Cada consecuencia P0 tiene un issue de seguimiento rastreado (`AUD-*` o equivalente).
- Los consumidores citan la decisión, nunca su estado ni su fecha.

## Resumen del registro

| ID      | Decisión aceptada                                                          | Fecha      | Autoridad    | Prioridad | Estado   | Desviación |
| ------- | -------------------------------------------------------------------------- | ---------- | ------------ | --------- | -------- | ---------- |
| DEC-001 | Perfil ArchMaker + reporte; `archinstall` experimental                     | 2026-10-05 | product      | P0        | accepted | no         |
| DEC-002 | IDs globales con namespace `vendor.kind.id`                                | 2026-10-05 | architecture | P0        | accepted | no         |
| DEC-003 | Adapter versionado de `archinstall`, dominio desacoplado                   | 2026-10-05 | architecture | P0        | accepted | no         |
| DEC-004 | Runner por Unix socket + autenticación de sesión; elevar fuera del WebView | 2026-10-05 | security     | P0        | accepted | no         |
| DEC-005 | Licencia **Apache-2.0 únicamente**                                         | 2026-10-05 | product      | P0        | accepted | no         |
| DEC-006 | Firma Sigstore (keyless) de releases; verificación offline                 | 2026-10-05 | security     | P1        | accepted | no         |
| DEC-007 | **Tauri updater en MVP**                                                   | 2026-10-05 | release      | P1        | accepted | sí         |
| DEC-008 | **Solo Arch x86_64**                                                       | 2026-10-05 | product      | P0        | accepted | sí         |
| DEC-009 | Sin CDN en runtime; recursos empaquetados o del sistema                    | 2026-10-06 | ux           | P1        | accepted | no         |
| DEC-010 | Sin telemetría en MVP                                                      | 2026-10-06 | product      | P1        | accepted | no         |
| DEC-011 | Nombre público Neubat + canal inicial AppImage Linux x86_64                | 2026-10-16 | product      | P0        | accepted | no         |
| DEC-012 | Nombre visible Forge (GUI) + motor ArchMaker; ids técnicos intactos        | 2026-10-17 | product      | P0        | accepted | no         |
| DEC-013 | Target `archmaker-live-profile` y proceso de constitución, sin A6          | 2026-10-18 | architecture | P0        | accepted | no         |

## Registro estructurado

### DEC-001 — Alcance de exportación del MVP

- Pregunta: ¿qué exporta el MVP?
- Decisión: perfil ArchMaker + reporte; exporter `archinstall` experimental.
- Fecha y autoridad: 2026-10-05 · `product` · `accepted`.
- ADR: ADR-0003. Requisitos: FR-EXPORT-001, FR-MANIFEST-001. Riesgos: —. Gates: G2, G10.
- Consecuencia: conjunto de `ExportTarget` fijado (`archmaker-profile`, `archinstall-profile`,
  `report`). DEC-013 añade un cuarto id; no borra estos tres.
- Issue de seguimiento: AUD-016.

### DEC-013 — Perfil vivo y constitución de la imagen

- Pregunta: ¿cómo se amplía el conjunto de export para la imagen viva sin abrir discos?
- Decisión: target técnico `archmaker-live-profile` y un proceso separado de constitución
  (descarga tipada y una corrida `mkarchiso`). No es el runner de instalación de ADR-0004.
- Fecha y autoridad: 2026-10-18 · `architecture` · `accepted`.
- ADR: ADR-0019. Requisitos: FR-ART-ISO-001, FR-FETCH-001, FR-ISO-001, NFR-MARK-001.
  Riesgos: RSK-008 (no reabierto: este proceso no eleva). Gates: G2, G4, G8, G10.
- Consecuencia: el fichero público es `Forge-<prefijo-de-digest>-x86_64.iso` y se describe
  como derivada de Arch Linux. El contrato de código cambia en ISO-02.
- Issue de seguimiento: ISO-02.

### DEC-002 — Ámbito de los identificadores

- Pregunta: ¿ámbito global, por catálogo o por sección?
- Decisión: identificadores globales con namespace `vendor.kind.id`.
- Fecha y autoridad: 2026-10-05 · `architecture` · `accepted`.
- ADR: ADR-0005. Requisitos: FR-CAT-001, FR-PRESET-001, FR-DRAFT-001, NFR-DET-001.
  Riesgos: RSK-005. Gates: G4.
- Consecuencia: habilita la detección de duplicados (`CON-004`) y el namespace de `CatalogRef`.
- Issue de seguimiento: AUD-011.

### DEC-003 — Motor de instalación v1

- Pregunta: ¿`archinstall`, motor propio o scripts?
- Decisión: adapter versionado de `archinstall`, sin acoplar el dominio.
- Fecha y autoridad: 2026-10-05 · `architecture` · `accepted`.
- ADR: ADR-0003, ADR-0004. Requisitos: FR-EXPORT-001, NFR-PORT-001. Riesgos: RSK-002.
  Gates: G3, G5.
- Consecuencia: el dominio permanece agnóstico del target; sin `pacstrap` ni shell en runtime.
- Issue de seguimiento: AUD-016.

### DEC-004 — Transporte del runner y elevación

- Pregunta: ¿Unix socket, sidecar stdio o D-Bus?
- Decisión: Unix socket + autenticación de sesión; elevación fuera del WebView.
- Fecha y autoridad: 2026-10-05 · `security` · `accepted`.
- ADR: ADR-0004. Requisitos: FR-RUN-001, NFR-SEC-001. Riesgos: RSK-001, RSK-008.
  Amenazas: THR-RUN-001, THR-RUN-002. Gates: G8.
- Consecuencia: transporte y modelo de elevación fijados; v1 diferido hasta su gate.
- Issue de seguimiento: AUD-018.

### DEC-005 — Licencia del repositorio

- Pregunta: ¿Apache-2.0, dual MIT/Apache o GPLv3?
- Decisión (**clarificación**): **Apache-2.0 únicamente**. Queda descartada la alternativa dual
  MIT/Apache. El fichero `LICENSE` ya es Apache-2.0 y no se modifica.
- Fecha y autoridad: 2026-10-05 (clarificada 2026-10-06) · `product` · `accepted`.
- ADR: —. Requisitos: —. Riesgos: RSK-007. Gates: G9.
- Consecuencia: licencia única de repositorio, recursos visuales y artefactos derivados.
- Issue de seguimiento: AUD-022.

### DEC-006 — Firma de catálogos y releases

- Pregunta: ¿Minisign, Sigstore o ambos?
- Decisión: **Sigstore** para releases + **verificación offline**. Clarificación exigida:
  1. **Qué se firma:** release files, binarios y SBOM (además de imágenes de release y artefactos
     de actualización).
  2. **Quién firma:** Release owner, mediante la identidad del workflow de CI (OIDC); no hay
     clave privada de larga duración.
  3. **Custodia y gestión de claves:** Sigstore **Fulcio** emite el certificado efímero ligado a la
     identidad OIDC y **Rekor** registra el evento de firma (transparencia). No se custodia clave
     privada persistente; rotación/revocación dependen de la identidad y del registro.
  4. **Verificación offline:** `cosign verify` con las raíces de **Fulcio y Rekor fijadas**
     (`trusted_root.json` embebido), de modo que la verificación no dependa de red.
- Fecha y autoridad: 2026-10-05 (clarificada 2026-10-06) · `security` · `accepted`.
- ADR: ADR-0008, ADR-0009. Requisitos: FR-CAT-001, NFR-SEC-001, NFR-OBS-001.
  Riesgos: RSK-007. Amenazas: THR-CAT-001, THR-SUP-001. Gates: G9.
- Consecuencia: firma keyless ligada a OIDC; verificación offline con raíces fijadas; coste
  operativo de firma y de fijación de raíces.
- Issue de seguimiento: AUD-019, AUD-022.

### DEC-007 — Actualización de la aplicación

- Pregunta: ¿actualización manual, Tauri updater o repos de distro?
- Decisión (**desviación aceptada**): **Tauri updater en el MVP**. Se confirma como desviación
  respecto a la recomendación inicial («manual/repo en MVP, updater firmado tras threat model»).
  Consecuencias obligatorias:
  1. `THR-UPD-001` deja de ser diferible y debe cerrarse **antes de MVP-0**.
  2. El canal de actualización debe estar **firmado** (arrastra DEC-006).
  3. **Fail-open offline** (`NFR-OFF-001`): la aplicación es plenamente usable sin red; solo
     comprueba actualizaciones cuando hay conectividad.
  4. Requiere `ADR-0009` y política de **canal, versión mínima y rollback**.
- Fecha y autoridad: 2026-10-05 · `release` · `accepted`.
- ADR: ADR-0009. Requisitos: NFR-SEC-001, NFR-OFF-001, NFR-OBS-001. Riesgos: RSK-007.
  Amenazas: THR-UPD-001. Gates: G8, G9.
- Consecuencia: superficie de red adicional en el MVP, sujeta a firma y verificación.
- Issue de seguimiento: AUD-019.

### DEC-008 — Soporte inicial de plataforma

- Pregunta: ¿Arch, Arch-based o live ISO propia?
- Decisión (**desviación aceptada**): **Arch x86_64 únicamente**. `aarch64` queda **fuera de
  alcance**. Se confirma como desviación respecto a la recomendación inicial («Arch x86_64 +
  aarch64 experimental separado»). Consecuencias:
  1. `reference/v5.1/arch-info.json` (aarch64) queda solo como evidencia histórica.
  2. `docs/11-operations/support-policy.md` y `docs/09-quality/test-matrix.md` se reducen a
     x86_64.
  3. Un futuro soporte `aarch64` exige nueva decisión, hardware/CI y catálogo verificado.
- Fecha y autoridad: 2026-10-05 · `product` · `accepted`.
- ADR: ADR-0003. Requisitos: FR-EXPORT-001, NFR-PORT-001. Riesgos: —. Gates: G2, G10.
- Consecuencia: matriz de soporte y pruebas limitada a x86_64.
- Issue de seguimiento: AUD-020, AUD-021.

### DEC-009 — Fuentes de recursos en runtime

- Pregunta: ¿CDN, recursos empaquetados o del sistema?
- Decisión (**resolución**): **sin CDN en runtime**; los recursos se **empaquetan** con la
  aplicación o se toman **del sistema**.
- Fecha y autoridad: 2026-10-06 · `ux` (Diseño/Legal) · `accepted`.
- ADR: ADR-0002. Requisitos: NFR-OFF-001, FR-CAT-001. Riesgos: RSK-006.
  Contradicciones: CON-010. Gates: G6, G8.
- Consecuencia: CSP restrictiva `self`; eliminación de fuentes remotas y dependencias CDN.
- Issue de seguimiento: AUD-020.

### DEC-010 — Telemetría

- Pregunta: ¿ninguna, opt-in o enterprise?
- Decisión (**resolución**): **sin telemetría en MVP**. Un opt-in futuro exige ADR propio antes
  de diseñar observabilidad externa.
- Fecha y autoridad: 2026-10-06 · `product` · `accepted`.
- ADR: —. Requisitos: NFR-OBS-001. Riesgos: —. Gates: G8.
- Consecuencia: el MVP no envía telemetría; solo errores y eventos locales redactados.
- Issue de seguimiento: — (diferido a ADR futuro).

### DEC-011 — Nombre público y canal inicial de distribución

- Pregunta: ¿qué nombre público y qué plataforma única abren la primera release verificable
  sin permiso escrito de Arch Linux?
- Decisión: nombre público **Neubat** (ADR-0010 opción 3; ids técnicos `archmaker*` intactos);
  plataforma inicial **AppImage Linux x86_64** publicada en GitHub Release (`release.yml`);
  MVP-6 `accepted` solo para esa plataforma; `.deb`/`.rpm`/OBS/AUR/Flatpak deferred o blocked;
  AUR nunca como repo binario en este acto.
- Fecha y autoridad: 2026-10-16 · `product` · `accepted`.
- ADR: ADR-0010. Requisitos: NFR-MARK-001, NFR-COST-001, FR-EXPORT-001. Riesgos: —. Gates: G2, G8, G10.
- Consecuencia: F8 passed / F9 authorized; metainfo público sin signo ARCH; F9 construye desde
  `RC_SHA` de `main` sin alterar bytes tras el digest.
- Issue de seguimiento: MVP-6-02, MVP-6-03 (F9).

### DEC-012 — Nombre visible Forge (GUI de ArchMaker)

- Pregunta: ¿qué nombre visible usa la GUI no publicada tras `v0.1.0`, sin alterar la identidad
  técnica ni reescribir la release ya publicada?
- Decisión: nombre visible de la GUI **Forge**; proyecto y motor **ArchMaker**; fórmula
  «Forge, la interfaz gráfica de ArchMaker». Ids técnicos (`dev.archmaker.app`,
  IndexedDB `archmaker-profiles`, `archmaker.lastDraftId`, crates `archmaker*`,
  `@archmaker/web`) intactos. ADR-0010 / DEC-011 permanecen como historia de `v0.1.0` (Neubat).
- Fecha y autoridad: 2026-10-17 · `product` · `accepted`.
- ADR: ADR-0012. Requisitos: NFR-MARK-001. Riesgos: —. Gates: G2.
- Consecuencia: superficies activas y empaquetado futuro usan Forge; no se mueven tags ni
  assets de `v0.1.0`.
- Issue de seguimiento: product-naming P1.

## Desviaciones respecto a la recomendación

Las únicas desviaciones aceptadas son `DEC-007` y `DEC-008`; ambas se detallan en su registro. El
resto de decisiones coinciden con la recomendación o la precisan (DEC-005, DEC-006, DEC-009,
DEC-010, DEC-011, DEC-012, DEC-013).

## Consecuencias aplicadas y no aplicadas

| ID      | Consecuencia                                                         | Estado                                                                  | Issue            |
| ------- | -------------------------------------------------------------------- | ----------------------------------------------------------------------- | ---------------- |
| DEC-001 | Conjunto de `ExportTarget` fijado en contratos y DTO.                | aplicada                                                                | AUD-016          |
| DEC-002 | Namespace global `vendor.kind.id`; habilita detección de duplicados. | aplicada                                                                | AUD-011          |
| DEC-003 | Dominio agnóstico del target; sin `pacstrap`/shell en runtime.       | aplicada                                                                | AUD-016          |
| DEC-004 | Transporte y elevación fijados; runner v1 diferido a su gate.        | aplicada (diseño)                                                       | AUD-018          |
| DEC-005 | Licencia Apache-2.0 única en `LICENSE`.                              | aplicada                                                                | AUD-022          |
| DEC-006 | Firma keyless y verificación offline definidas.                      | parcial (política)                                                      | AUD-019, AUD-022 |
| DEC-006 | Pipeline real de build/attest/SBOM y firmas.                         | parcial (F9 `v0.1.0`: SBOM/provenance/checksums; cosign local deferred) | AUD-022          |
| DEC-007 | `THR-UPD-001` cerrado antes de MVP-0.                                | no aplicada                                                             | AUD-019          |
| DEC-007 | Updater deshabilitado por defecto en builds de desarrollo.           | aplicada                                                                | AUD-019          |
| DEC-008 | Matriz de soporte y pruebas reducida a x86_64.                       | aplicada                                                                | AUD-020          |
| DEC-009 | Sin CDN en runtime; CSP `self` y recursos empaquetados.              | aplicada                                                                | AUD-020          |
| DEC-010 | MVP sin telemetría; solo eventos locales redactados.                 | aplicada                                                                | —                |
| DEC-011 | Nombre público Neubat; AppImage x86_64 como único canal inicial.     | aplicada (gobierno + release `v0.1.0`)                                  | MVP-6-02/03 (F9) |
| DEC-012 | Nombre visible Forge; motor ArchMaker; sin renombrar ids técnicos.   | aplicada                                                                | ADR-0012         |
| DEC-013 | Target `archmaker-live-profile`; proceso de constitución sin A6.     | aplicada al target; la cara no ejecuta `mkarchiso`                      | ISO-02           |

## Reglas de validación (R2)

El validador del plan R2 comprueba:

```text
DEC reference exists
→ consumer does not restate status
→ accepted decision has date and authority
→ consequences have tracked issues
→ superseded decision points to successor
```

Equivalencias aplicadas:

1. Toda referencia `DEC-00N` en un consumidor resuelve a una decisión de este registro.
2. Ningún consumidor reafirma el estado, la fecha ni la opción de la decisión; solo la cita.
3. Toda decisión `accepted` declara fecha y autoridad.
4. Toda consecuencia P0 tiene issue de seguimiento rastreado en la tabla anterior.
5. Toda decisión `superseded` apuntaría a su sucesora; hoy no existe ninguna.

## Historia

Se conservan `DEC-001`..`DEC-013` sin eliminar ninguna. Todas están en estado `accepted`; ninguna
está `superseded`, `rejected` ni `proposed`, por lo que no se declara sucesora. `DEC-005` y
`DEC-006` fueron aceptadas el 2026-10-05 y **clarificadas** el 2026-10-06. `DEC-009` y `DEC-010`
pasan de `proposed` a `accepted` el 2026-10-06. `DEC-011` se acepta el 2026-10-16 con ADR-0010
opción 3 y MVP-6 AppImage. `DEC-012` se acepta el 2026-10-17 con ADR-0012 (Forge). `DEC-013` se
acepta el 2026-10-18 con ADR-0019 (perfil vivo; el contrato de código queda para ISO-02).
