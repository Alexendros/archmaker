# Reglas de dependencia

- El grafo debe ser acíclico.
- Dominio no conoce serialización externa salvo value types necesarios.
- Los DTO públicos no exponen tipos de infraestructura.
- UI solo conoce `CorePort` y view models.
- WASM y Tauri deben superar pruebas de paridad.
- El catálogo contiene intención y metadatos, nunca comandos ejecutables.
- Los exporters traducen manifest canónico a targets; no alteran el draft.
- El runner recibe un plan inmutable y operations enum.
- Enterprise implementa puertos opcionales; el core local no depende del control plane.
