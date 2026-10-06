---
id: DOC-ARCH-C4-SEQ-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Secuencias de runtime

- ID: DOC-ARCH-C4-SEQ-001
- Estado: in-review
- Propietario: Arquitectura
- Fecha: 2026-10-06
- Requisitos relacionados: FR-DRAFT-001, FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001, FR-EXPORT-001, FR-RUN-001, NFR-DET-001
- Viewpoint: VP-05 Interacción y runtime ([`../viewpoints.md`](../viewpoints.md))
- Fase: R4

## Secuencia — Editar y guardar draft

```mermaid
sequenceDiagram
  autonumber
  actor U as Usuario
  participant UI as UI Desktop o Browser
  participant CP as CorePort
  participant CORE as Rust Core
  participant DS as Local Draft Store
  U->>UI: crea o edita el draft
  UI->>CP: saveDraft request con expectedRevision
  CP->>CORE: saveDraft tipado
  CORE->>CORE: valida esquema y SelectionValue
  alt revision vigente
    CORE->>DS: escritura atomica
    DS-->>CORE: revision actualizada
    CORE-->>CP: Draft con revision y digest
    CP-->>UI: draft guardado
  else conflicto de revision
    CORE-->>CP: CoreError REVISION_CONFLICT
    CP-->>UI: conflicto sin escritura parcial
  end
```

Precondición: catálogo cargado. Postcondición: draft persistido con `expectedRevision` verificado;
ante conflicto o error de E/S, el draft previo permanece intacto (`FR-DRAFT-001`).

## Secuencia — Resolver

```mermaid
sequenceDiagram
  autonumber
  actor U as Usuario
  participant UI as UI
  participant CORE as Rust Core
  participant CAT as Local Catalog Store
  U->>UI: solicita resolver
  UI->>CORE: resolveDraft con draft y catalogRef
  CORE->>CAT: loadCatalog namespace id version
  CAT-->>CORE: catalogo con digest y procedencia
  CORE->>CORE: verifica digest y firma
  CORE->>CORE: resuelve capabilities y conflictos
  CORE->>CORE: valida y produce Resolution y Diagnostic
  CORE-->>UI: Resolution efimera y Diagnostic
  Note over CORE: La Resolution no se persiste
```

La resolución es determinista e idempotente; un conflicto bloqueante impide construir el manifest
(`FR-RESOLVE-001`, `FR-VALIDATE-001`, `NFR-DET-001`).

## Secuencia — Exportar

```mermaid
sequenceDiagram
  autonumber
  actor U as Usuario
  participant UI as UI
  participant CORE as Rust Core
  participant EXP as Export Adapter
  participant FS as Scope de escritura
  U->>UI: solicita exportar
  UI->>CORE: buildManifest y exportArtifact
  CORE->>CORE: canonicaliza y calcula digest
  CORE->>EXP: traduce Manifest a Artifact
  EXP-->>CORE: Artifact con mediaType y versiones
  CORE->>FS: escribe artefacto y reporte
  FS-->>CORE: confirmacion de escritura
  CORE-->>UI: Artifact y digest y reporte
```

Un target no soportado o un fallo de escritura parcial no produce artefacto válido a medias
(`FR-MANIFEST-001`, `FR-EXPORT-001`).

## Secuencia — Actualizar

```mermaid
sequenceDiagram
  autonumber
  participant DESK as Desktop Application
  participant UPD as Updater
  participant NET as Canal de actualizacion
  participant APP as Producto local
  DESK->>UPD: comprueba actualizacion
  alt sin red
    UPD-->>DESK: fail-open, la app sigue operativa
  else con red
    UPD->>NET: solicita manifiesto firmado
    NET-->>UPD: manifiesto y firma
    UPD->>UPD: verifica firma y allowlist del endpoint
    alt firma valida
      UPD->>APP: aplica actualizacion
      APP-->>UPD: aplicada con rollback disponible
    else firma invalida
      UPD-->>DESK: rechaza, no se aplica
    end
  end
```

Sin red la aplicación arranca y opera con normalidad; la actualización nunca bloquea el uso
(`NFR-OFF-001`, ADR-0009).

## Secuencia — Ejecutar plan

```mermaid
sequenceDiagram
  autonumber
  actor U as Usuario
  participant UI as UI
  participant HOST as Host Desktop fuera del WebView
  participant RUN as Runner v1
  participant OS as Sistema operativo
  U->>UI: confirma la ejecucion del plan
  UI->>HOST: request con plan y protocolVersion
  HOST->>RUN: conecta por Unix socket y autentica sesion
  RUN->>RUN: preflight y dry-run
  RUN-->>HOST: preflight correcto y hash de plan
  U->>UI: confirma el hash
  UI->>HOST: confirmacion ligada al hash
  HOST->>RUN: execute con operations allowlisted
  RUN->>OS: operaciones tipadas con elevacion temporal
  OS-->>RUN: resultados
  RUN->>RUN: journal append-only redactado
  RUN-->>HOST: resultado y journal
  HOST-->>UI: estado final
```

Un hash de confirmación distinto o un cambio de inventario rechaza la ejecución; el WebView nunca
ejecuta operaciones privilegiadas (ADR-0004, `FR-RUN-001`).
