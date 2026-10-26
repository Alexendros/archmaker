# Policy exceptions

Registro de excepciones a la norma de flota (etiqueta `policy-exception` en el PR).

| PR   | Fecha (Madrid) | Motivo                                                                                                                                                                                                                              | Autorizado por                   |
| ---- | -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------- |
| #195 | 2026-10-26     | Inventario de CI y creación del registro de excepciones antes de consolidar los workflows.                                                                                                                                          | Arquitecto, plan archmaker-adr-e |
| #197 | 2026-10-26     | Jobs Rust, Tauri y web en línea porque reusable-rust/node v6.0.0 no cubren GTK/WebKit, smoke de GUI ni los scripts del repo. Prettier se limita a archivos modificados por excepción autorizada; la deuda global queda documentada. | Arquitecto, plan archmaker-adr-e |
| #198 | 2026-10-26     | Seis avisos OSV transitivos de Tauri/GTK/rfd aceptados temporalmente según D-E1, con revisión obligatoria el 2027-01-25.                                                                                                            | Arquitecto, plan archmaker-adr-e |
