# ADR-0004: Runner como proceso separado; transporte y elevación fuera del WebView

- Estado: proposed
- Fecha: 2026-10-05
- Propietario: Seguridad
- Requisitos: FR-RUN-001, NFR-SEC-001, NFR-OBS-001

## Contexto

La instalación (v1) implica privilegio elevado y operaciones destructivas. El WebView no puede controlar root: `THR-RUN-001` describe ese riesgo y `RSK-008` la elevación insegura. El modelo de privilegios exige que la elevación ocurra fuera del contenido WebView, que el runner autentique peer y sesión, que las confirmaciones destructivas expiren y se liguen a hashes y que los cambios de inventario invaliden el plan. El protocolo v1 ya define un envelope tipado, una máquina de estados y comandos allowlisted. La decisión DEC-004 fijó el transporte el 2026-10-05: **Unix socket con autenticación de sesión**, con elevación fuera del WebView.

Documentos de apoyo: `docs/04-interfaces/runner-protocol.md`, `docs/08-security/privilege-model.md`, `docs/08-security/threat-model.md`, `docs/03-data/domain-model.md` (`DM-PLAN`, `DM-SESSION`).

## Drivers

- `NFR-SEC-001`: deny-by-default y mínimo privilegio.
- `THR-RUN-001`: impedir que el WebView controle root.
- `THR-RUN-002`: evitar TOCTOU tras confirmar.
- `RSK-001`: evitar el borrado del disco equivocado.
- `RSK-008`: elevación y autenticación seguras.
- Plan reproducible, inmutable y confirmado por hash.

## Opciones consideradas

- **Transporte**: Unix socket con autenticación de sesión; stdio sidecar; D-Bus.
- **Ejecución**: runner en el mismo proceso que el host Tauri; runner como proceso separado con operaciones tipadas.
- **Elevación**: dentro del WebView; fuera del WebView mediante un componente dedicado.

## Decisión propuesta

1. El **runner es un proceso separado** que recibe un `InstallationPlan` inmutable, confirmado por **hash de plan y de manifest**, y ejecuta únicamente **operaciones allowlisted** (`enum`). Nunca interpreta scripts, catálogo ni HTML.
2. El **transporte y la elevación quedan fuera del WebView**. La elevación se realiza por un componente dedicado fuera del contenido web, conforme al modelo de privilegios.
3. El **transporte es Unix socket con autenticación de sesión** (DEC-004 aceptada el 2026-10-05). Las alternativas stdio sidecar y D-Bus quedan descartadas para v1.
4. La confirmación destructiva expira y se liga a los hashes; un cambio de inventario invalida el plan.

Este ADR es referenciado por `docs/00-governance/traceability-matrix.md` (OBJ-005 / `DM-PLAN`).

## Consecuencias

- El runner es auditable y sustituible sin exponer privilegios al WebView.
- El protocolo tipado (`runner-protocol.md`) es la única frontera de ejecución.
- Se asume coste de empaquetado/distribución del proceso y de su ciclo de vida (arranque, sesión, expiración).
- La elección de transporte (Unix socket + autenticación de sesión) afecta empaquetado, sandbox y pruebas; fijada por DEC-004.

## Riesgos

- `RSK-008` elevación insegura: mitigado por autenticación de sesión y elevación fuera del WebView.
- `RSK-001` disco equivocado: mitigado por inventario estable, hash de plan y doble confirmación.
- `THR-RUN-001` / `THR-RUN-002`: mitigados por proceso separado, protocolo tipado y re-preflight.

## Verificación

- Prueba negativa: una ventana no autorizada no puede invocar al runner ni abrir el socket libremente.
- `TST-RUN-001` y `VAL-PLAN`: la confirmación ligada a hash es inválida si cambia el plan o el inventario.
- Journal append-only redactado y cancelación solo en safe points.
- Compatibilidad de protocolo negociada antes de aceptar el manifest (`protocolVersion`).
- El transporte (Unix socket + autenticación de sesión) queda verificado por DEC-004 (2026-10-05); su implementación concreta se valida en `TST-RUN-001`.

## Sustituye

—

## Sustituido por

—

## Requisitos relacionados

FR-RUN-001, NFR-SEC-001, NFR-OBS-001. Resuelve DEC-004 (transporte Unix socket + autenticación de sesión). Relaciona `DM-PLAN`, `DM-SESSION`, `THR-RUN-001`, `THR-RUN-002`, `RSK-008`.
