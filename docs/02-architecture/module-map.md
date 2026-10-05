# Mapa de módulos

| Crate/paquete | Responsabilidad | No puede depender de |
|---|---|---|
| `archmaker-domain` | Entidades, value objects e invariantes. | UI, Tauri, WASM, FS, red. |
| `archmaker-schema` | Schemas y meta-validación. | UI/infra. |
| `archmaker-catalog` | Carga, índice, procedencia. | Tauri/UI. |
| `archmaker-rules` | AST tipado y evaluación pura. | Infra. |
| `archmaker-resolver` | Capabilities, conflictos, ChangeSet. | UI. |
| `archmaker-validation` | Pipeline y diagnósticos. | UI. |
| `archmaker-migrations` | Transformaciones puras e informes. | Infra mutable. |
| `archmaker-export` | SPI de targets y artifacts. | Tauri. |
| `archmaker-api` | Casos de uso y puertos. | Implementaciones concretas. |
| `archmaker-wasm` | Adapter browser. | Tauri. |
| `archmaker-tauri` | Adapter desktop. | Runner internals. |
| `archmaker-plan` | Modelo de plan tipado. | Ejecución. |
| `archmaker-runner-protocol` | Mensajes/state machine. | UI. |
| `archmaker-runner` | Preflight/ejecución/journal. | WebView. |
