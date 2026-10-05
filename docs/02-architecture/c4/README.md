# C4

## Contexto

```mermaid
C4Context
  title ArchMaker — contexto
  Person(user, "Usuario", "Configura y revisa una instalación")
  Person(operator, "Operador", "Gobierna perfiles y campañas")
  System(archmaker, "ArchMaker", "Configura, valida, exporta y, en v1, instala")
  System_Ext(sources, "Fuentes oficiales", "ArchWiki, repos, archinstall")
  System_Ext(control, "Control plane", "Enterprise opcional")
  Rel(user, archmaker, "Usa")
  Rel(operator, control, "Administra")
  Rel(archmaker, sources, "Verifica metadatos")
  Rel(archmaker, control, "Sincroniza opcionalmente")
```

## Contenedores

```mermaid
C4Container
  title ArchMaker — contenedores
  Person(user, "Usuario")
  Container(web, "Web App", "React + WASM", "MVP browser")
  Container(desktop, "Desktop App", "Tauri 2 + React", "MVP Linux")
  Container(core, "Core", "Rust", "Dominio y casos de uso")
  ContainerDb(local, "Local Store", "JSON/files", "Drafts y preferencias")
  Container(runner, "Runner", "Rust", "v1; ejecución separada")
  Container(api, "Enterprise API", "Rust/Axum", "Control plane opcional")
  ContainerDb(db, "Enterprise Store", "PostgreSQL", "Tenants y auditoría")
  Rel(user, web, "Usa")
  Rel(user, desktop, "Usa")
  Rel(web, core, "CorePort/WASM")
  Rel(desktop, core, "CorePort/Tauri")
  Rel(desktop, local, "Acceso scoped")
  Rel(desktop, runner, "Protocolo versionado", "v1")
  Rel(runner, api, "mTLS/OIDC", "Enterprise")
  Rel(api, db, "SQL")
```

## Componentes Core

```mermaid
flowchart LR
  API[archmaker-api] --> DOMAIN[domain]
  API --> CATALOG[catalog]
  API --> MIG[migrations]
  API --> RES[resolver]
  API --> VAL[validation]
  API --> EXP[export]
  RES --> RULES[rules]
  VAL --> RULES
  CATALOG --> DOMAIN
  RULES --> DOMAIN
  MIG --> DOMAIN
  EXP --> DOMAIN
```

## Deployments

- MVP web: assets estáticos + WASM + Web Worker, sin backend obligatorio.
- MVP desktop: bundle Tauri con core enlazado y filesystem limitado por picker/scope.
- v1: desktop no privilegiado + runner autenticado/elevado por mecanismo del SO.
- Enterprise: control plane separado; el producto local mantiene modo autónomo.
