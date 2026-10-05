# Modelo de privilegios

## MVP

- Web/desktop: usuario normal.
- Core: computación pura.
- Filesystem: solo ficheros seleccionados o directorios de aplicación.
- Red: no necesaria para configurar con catálogo embebido.
- Shell/sidecars/root/discos: prohibidos.

## v1

| Componente | Privilegio | Puede | No puede |
|---|---|---|---|
| WebView | Usuario | Presentar y solicitar | Abrir discos/root/socket libremente |
| Tauri host | Usuario | Validar, serializar, autenticar sesión | Ejecutar instalación |
| Runner | Elevado temporal | Preflight/operaciones allowlisted | Interpretar scripts/catálogo |
| Adapter | Dentro runner | Traducir plan a API fijada | Modificar el plan confirmado |

## Reglas

- La elevación ocurre fuera del contenido WebView.
- El runner autentica peer y sesión.
- Confirmaciones destructivas expiran y se ligan a hashes.
- Cambios del inventario invalidan el plan.
- Los secretos se suministran just-in-time y se borran de memoria cuando sea posible.
