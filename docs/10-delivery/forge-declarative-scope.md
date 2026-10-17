---
id: DOC-DEL-FORGE-DECL-001
phase: planning
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-MVP5-001
    expectedDocumentStatus: accepted
---

# Alcance Forge — planificación declarativa (A6 fuera)

- Documento: DOC-DEL-FORGE-DECL-001 · Estado: draft · Propietario: Arquitectura · Fecha: 2026-10-17
- Base GitHub: `b6c0b50a6dcb5b42410d5e067019cf323494ccc4` (`origin/main`).
- **No reescribe** [`mvp5-scope.md`](mvp5-scope.md): MVP-5 sigue cerrado; este documento registra un
  alcance *adicional* pendiente de `accepted` para sondas de solo lectura y UI posteriores.

## Objetivo

Autorizar el trabajo fundacional de contratos `schemaVersion` `"2"` (draft/manifiesto) para que
Forge exporte un perfil con plan de disco **declarativo**, estados de verificación ortogonales y
política de cifrado/navegadores **sin** ejecutar escrituras en discos.

## Incluido (fase fundacional)

| Área | Contenido | Notas |
| ---- | --------- | ----- |
| Contratos | Draft/manifiesto v2, migrador v1→v2, digests `archmaker:draft:v2` / `archmaker:manifest:v2` | Rust autoridad |
| ADR | UKI en ESP, estados ortogonales, handles, digests, ONLYOFFICE/navegadores, A6 excluida | `docs/02-architecture/adr/` |
| Walking skeleton | Draft v2 manual → CorePort → buildManifest/export → reimport/migración | Sin sonda Linux fingida |
| Sondas futuras | Solo lectura; IPC tipado; sin root | Fuera de este PR; documentadas como futuras |

## Excluido (explícito)

- **A6**: helper privilegiado, polkit, escritura de tabla GPT/MBR, LUKS2 real, filesystem real,
  formateo, `BootOrder`, Secure Boot write.
- Runner, root, shell, `pacstrap`, ISO, sidecars de instalación.
- UI completa de modos/layout, catálogo ApexShot completo, aceptación T01–T44, AppImage, publicación.
- Reescritura de `mvp5-scope.md` como si la ejecución en discos estuviera permitida.

## Inventario baseline (A0)

Confirmado al arrancar la rama `feat/forge-fundacional-v2` sobre `b6c0b50`:

| Elemento | Estado en main |
| -------- | -------------- |
| Contratos JSON | `contracts/json-schema/` v0; draft/manifest `schemaVersion` `"1"` |
| Catálogo | `crates/archmaker-core/fixtures/local-catalog-mvp1.json` (ONLYOFFICE id canónico presente) |
| CorePort | `create_draft` … `import_draft` + `save_export_bytes` (Tauri) |
| Capabilities Tauri | `src-tauri/capabilities/main.json` deny-by-default; sin comandos de disco |
| Dominio hash | `archmaker:draft:v1` / `archmaker:manifest:v1` (ADR-0007) |

## Criterio de salida de la fase fundacional

- ADRs 0013–0018 registrados.
- Corpus JSON + tests Rust de contratos/migración v2 en verde.
- Walking skeleton demostrable por test de integración CorePort.
- `executionStatus` fijado a `not-executed`; A6 documentada como fuera.

## Trazabilidad

- ADR-0013 … ADR-0018.
- Requisitos previstos (sin ampliar veredicto MVP-5): FR-DRAFT / FR-MANIFEST / NFR-DET / NFR-PORT.
- Riesgos: RSK-004 (semántica fuera de Rust), amenaza de ejecución prematura en discos (mitigada por exclusión A6).
