---
id: DOC-ARCH-C4-CMP-001
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

# Vistas de componentes

- ID: DOC-ARCH-C4-CMP-001
- Estado: in-review
- Propietario: Arquitectura
- Fecha: 2026-10-06
- Requisitos relacionados: NFR-PORT-001, NFR-DET-001, NFR-OBS-001
- Viewpoint: VP-03 Componentes ([`../viewpoints.md`](../viewpoints.md))
- Fase: R4; resuelve `AUD-F-08`

## VW-03 — Componentes de Desktop Application

```mermaid
flowchart TB
  subgraph DESKTOP["Desktop Application (contenedor)"]
    UI["UI WebView React — view models, sin reglas de dominio"]
    subgraph ADAPT["Adaptadores"]
      TAURI["archmaker-tauri — IPC, capabilities y FS con scope"]
    end
    subgraph CORE["Rust Core (componente enlazado, nativo)"]
      API["archmaker-api — casos de uso y puertos CorePort"]
      DOMAIN["archmaker-domain"]
      SCHEMA["archmaker-schema"]
      CATALOG["archmaker-catalog"]
      RULES["archmaker-rules"]
      RESOLVER["archmaker-resolver"]
      VALIDATION["archmaker-validation"]
      MIGRATIONS["archmaker-migrations"]
      EXPORT["archmaker-export"]
    end
  end
  UI -->|"CorePort"| TAURI
  TAURI -->|"CorePort"| API
  API --> DOMAIN
  API --> CATALOG
  API --> MIGRATIONS
  API --> RESOLVER
  API --> VALIDATION
  API --> EXPORT
  RESOLVER --> RULES
  VALIDATION --> RULES
  CATALOG --> DOMAIN
  RULES --> DOMAIN
  MIGRATIONS --> DOMAIN
  EXPORT --> DOMAIN
```

El adaptador `archmaker-tauri` aplica deny-by-default por ventana y scope mínimo de filesystem
(ADR-0002). La UI solo conoce `CorePort` y view models; no reimplementa reglas (ADR-0001).

## VW-04 — Componentes de Browser Application

```mermaid
flowchart TB
  subgraph BROWSER["Browser Application (contenedor)"]
    UI["UI React en el hilo principal"]
    WORKER["Web Worker — ejecuta el nucleo WASM"]
    subgraph CORE["Rust Core (componente enlazado, WASM)"]
      ADAPTER["archmaker-wasm — bindings CorePort"]
      MODULES["archmaker-api y crates de dominio (mismos modulos que Desktop)"]
    end
  end
  UI -->|"postMessage"| WORKER
  WORKER --> ADAPTER
  ADAPTER --> MODULES
```

El mismo `Rust Core` se compila a WASM y se ejecuta en un Web Worker; el bundle es estático y no
requiere backend ni CDN (`NFR-OFF-001`). La paridad semántica con Desktop se verifica por golden
parity (`NFR-DET-001`).

## VW-05 — Componentes de Rust Core

```mermaid
flowchart LR
  subgraph ADAPT["Adaptadores (fuera del Rust Core)"]
    TA["archmaker-tauri"]
    WA["archmaker-wasm"]
  end
  subgraph CORE["Rust Core (componente interno)"]
    API["archmaker-api"]
    DOMAIN["archmaker-domain"]
    SCHEMA["archmaker-schema"]
    CATALOG["archmaker-catalog"]
    RULES["archmaker-rules"]
    RESOLVER["archmaker-resolver"]
    VALIDATION["archmaker-validation"]
    MIGRATIONS["archmaker-migrations"]
    EXPORT["archmaker-export"]
  end
  TA --> API
  WA --> API
  API --> DOMAIN
  API --> CATALOG
  API --> MIGRATIONS
  API --> RESOLVER
  API --> VALIDATION
  API --> EXPORT
  RESOLVER --> RULES
  VALIDATION --> RULES
  CATALOG --> DOMAIN
  RULES --> DOMAIN
  MIGRATIONS --> DOMAIN
  EXPORT --> DOMAIN
```

Pipeline canónico de datos:

```mermaid
flowchart LR
  D["Draft"] --> R["Resolution"]
  R --> V["Validacion"]
  V --> M["Manifest canonico"]
  M --> A["Artifact"]
  M --> P["Plan v1"]
  CC["Canonicalizacion y digest"] -.-> R
  CC -.-> M
  CC -.-> A
```

Reglas de dependencia aplicadas ([`../dependency-rules.md`](../dependency-rules.md)): el grafo es
acíclico, el dominio no conoce infraestructura y los DTO públicos no exponen tipos de
infraestructura. La descomposición a nivel de crate está en
[`../module-map.md`](../module-map.md).

## VW-06 — Componentes de Runner v1

```mermaid
flowchart TB
  subgraph HOST["Desktop Application (host, no privilegiado)"]
    UI["UI"]
    REQ["Cliente del protocolo runner"]
  end
  subgraph RUNNER["Runner v1 (proceso separado)"]
    SESSION["Autenticacion de sesion"]
    PROTO["archmaker-runner-protocol — mensajes y state machine"]
    PRE["Preflight y dry-run"]
    ALLOW["Allowlist tipada de operations"]
    EXEC["Ejecutor"]
    JOURNAL["Journal append-only redactado"]
  end
  subgraph SOS["Sistema operativo"]
    ELEV["Mecanismo de elevacion del SO"]
    TARGET["Operaciones tipadas sobre el sistema"]
  end
  UI --> REQ
  REQ -->|"Unix socket + sesion"| SESSION
  SESSION --> PROTO
  PROTO --> PRE
  PRE --> ALLOW
  ALLOW --> EXEC
  EXEC -->|"elevacion temporal fuera del WebView"| ELEV
  ELEV --> TARGET
  EXEC --> JOURNAL
  PROTO -.->|"hash de plan"| REQ
```

El runner recibe un plan inmutable confirmado por hash y ejecuta solo operaciones allowlisted; el
WebView nunca controla root (ADR-0004, `THR-RUN-001`).

## Inventario de componentes e interfaces

| View | Componente | Tipo | Puerto/interfaz | No puede depender de |
|---|---|---|---|---|
| VW-03 | UI WebView React | Componente de UI | `CorePort` (vía `archmaker-tauri`) | Dominio interno, FS directo |
| VW-03 | `archmaker-tauri` | Adaptador | IPC + capabilities | Runner internals |
| VW-03 | `archmaker-api` | Caso de uso y puertos | `CorePort` | Implementaciones concretas |
| VW-04 | UI React + Web Worker | Componente de UI | `CorePort` (vía `archmaker-wasm`) | FS, red |
| VW-04 | `archmaker-wasm` | Adaptador | Bindings WASM | Tauri |
| VW-05 | `archmaker-domain` | Dominio | Value types y entidades | UI, Tauri, WASM, FS, red |
| VW-05 | `archmaker-schema` | Contratos | Meta-validación | UI/infraestructura |
| VW-05 | `archmaker-catalog` | Dominio de datos | Carga, índice y procedencia | Tauri/UI |
| VW-05 | `archmaker-rules` | Dominio | Evaluación pura | Infraestructura |
| VW-05 | `archmaker-resolver` | Dominio | Capabilities y ChangeSet | UI |
| VW-05 | `archmaker-validation` | Dominio | Pipeline y diagnósticos | UI |
| VW-05 | `archmaker-migrations` | Dominio | Transformaciones puras | Infraestructura mutable |
| VW-05 | `archmaker-export` | Dominio | SPI de targets y artifacts | Tauri |
| VW-06 | `archmaker-runner-protocol` | Contrato | Mensajes y state machine | UI |
| VW-06 | `archmaker-plan` | Modelo | Plan tipado | Ejecución |

**Ninguna crate es una unidad desplegable.** Todas son componentes enlazados dentro de los
contenedores; el detalle de artefactos está en [`deployment-views.md`](deployment-views.md).

## Boundaries nombrados

| Boundary | Incluye | Excluye |
|---|---|---|
| Desktop Application | UI, adaptador Tauri, `Rust Core` nativo | Runner, almacenes |
| Browser Application | UI, Web Worker, `Rust Core` WASM | Backend, red |
| Rust Core (componente) | Crates de dominio y `archmaker-api` | Adaptadores e infraestructura |
| Runner v1 | Protocolo, preflight, allowlist, ejecutor, journal | WebView y UI |
