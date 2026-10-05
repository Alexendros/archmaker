---
id: DOC-ARCH-VWP-001
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

# Viewpoints

- ID: DOC-ARCH-VWP-001
- Estado: in-review
- Propietario: Arquitectura
- Fecha: 2026-10-06
- Requisitos relacionados: NFR-PORT-001, NFR-DET-001, NFR-SEC-001, NFR-OBS-001
- Fase: R4 (issues `AUD-007`/`AUD-008`)

## Propósito

Definir los **viewpoints** de ArchMaker conforme a ISO/IEC/IEEE 42010 y su **correspondencia con
las views**. Cada viewpoint fija los concerns que encuadra, los stakeholders interesados, los
**model kinds** (tipos de modelo), el lenguaje/notación y las técnicas de construcción y análisis.
Los concerns se definen en [`stakeholders-concerns.md`](stakeholders-concerns.md).

Un viewpoint es una convención; una view es su instanciación sobre el sistema de interés. Este
documento no instancia las views: enlaza a los artefactos que las contienen.

## Índice de viewpoints

| ID | Viewpoint | Concerns | Vistas que genera | Notación |
|---|---|---|---|---|
| VP-01 | Contexto | CN-01, CN-05, CN-07, CN-15 | VW-01 | Mermaid `C4Context` |
| VP-02 | Contenedores | CN-01, CN-05, CN-07, CN-08, CN-12, CN-14, CN-15 | VW-02 | Mermaid `C4Container` |
| VP-03 | Componentes | CN-01, CN-02, CN-03, CN-04, CN-05, CN-06, CN-08, CN-09, CN-11, CN-13, CN-14, CN-15, CN-16, CN-17 | VW-03, VW-04, VW-05, VW-06 | Mermaid `flowchart` y diagramas de dependencia |
| VP-04 | Deployment | CN-01, CN-06, CN-07, CN-08, CN-10, CN-12 | VW-07, VW-08, VW-09, VW-10 | Mermaid `flowchart` con nodos y procesos |
| VP-05 | Interacción y runtime | CN-01, CN-02, CN-03, CN-04, CN-05, CN-06, CN-11, CN-12, CN-17 | VW-11 | Mermaid `sequenceDiagram` |

## VP-01 — Contexto

- **Concerns encuadrados:** CN-01 (configurar sin terminal), CN-05 (exportar),
  CN-07 (gobierno de flotas), CN-15 (catálogos firmados).
- **Stakeholders:** SH-01, SH-02, SH-04, SH-11, SH-13.
- **Model kinds:** actor, sistema de interés, sistema externo, relación y frontera de confianza.
- **Lenguaje/notación:** Mermaid `C4Context`; personas, sistemas y relaciones etiquetadas.
- **Técnicas de construcción:** identificar actores y sistemas externos; trazar quién cruza cada
  frontera del sistema.
- **Técnicas de análisis:** análisis de fronteras y de confianza; verificar que ninguna
  dependencia externa es obligatoria en el MVP.
- **Vista generada:** VW-01, en [`c4/README.md`](c4/README.md).

## VP-02 — Contenedores

- **Concerns encuadrados:** CN-01, CN-05, CN-07, CN-08, CN-12, CN-14, CN-15.
- **Stakeholders:** SH-01, SH-02, SH-04, SH-06, SH-08, SH-11.
- **Model kinds:** contenedor desplegable, almacén de datos, frontera (boundary), protocolo e
  interfaz entre contenedores.
- **Lenguaje/notación:** Mermaid `C4Container` con `System_Boundary`; tablas de contenedores,
  fronteras y protocolos.
- **Técnicas de construcción:** asignar responsabilidad por unidad desplegable; nombrar cada
  frontera y el protocolo que la cruza; distinguir contenedor de componente.
- **Técnicas de análisis:** comprobar que ninguna **librería** aparece como unidad desplegable;
  comprobar que el `Rust Core` es componente interno y no contenedor; revisar deny-by-default por
  frontera.
- **Vista generada:** VW-02, en [`c4/README.md`](c4/README.md).

## VP-03 — Componentes

- **Concerns encuadrados:** CN-01, CN-02, CN-03, CN-04, CN-05, CN-06, CN-08, CN-09, CN-11,
  CN-13, CN-14, CN-15, CN-16, CN-17.
- **Stakeholders:** SH-01, SH-02, SH-06, SH-07, SH-09, SH-10.
- **Model kinds:** componente, interfaz/puerto, dependencia y flujo de datos canónico.
- **Lenguaje/notación:** Mermaid `flowchart` y grafos de dependencia; subgrafos para fronteras de
  proceso (`Desktop`, `Browser`, `Rust Core`, `Runner`).
- **Técnicas de construcción:** descomponer cada contenedor en componentes; declarar puertos
  (`CorePort`) y dependencias; mapear componentes a crates.
- **Técnicas de análisis:** detección de ciclos de dependencia (`NFR-PORT-001`); verificación de
  que la UI no reimplementa reglas (`ADR-0001`); separación adaptador/dominio.
- **Vistas generadas:** VW-03 Desktop, VW-04 Browser, VW-05 Rust Core y VW-06 Runner, en
  [`c4/component-views.md`](c4/component-views.md).

## VP-04 — Deployment

- **Concerns encuadrados:** CN-01 (frontera local), CN-06 (ejecución segura), CN-07
  (Enterprise), CN-08 (privilegio mínimo), CN-10 (offline), CN-12 (cadena de suministro).
- **Stakeholders:** SH-08, SH-10, SH-11, SH-13.
- **Model kinds:** nodo de despliegue, proceso, artefacto desplegable, almacén y red/canal.
- **Lenguaje/notación:** Mermaid `flowchart` con subgrafos que representan nodos y procesos;
  inventario de artefactos.
- **Técnicas de construcción:** mapear contenedores a nodos y procesos por fase (MVP Desktop, MVP
  Web, v1, Enterprise); declarar qué es artefacto desplegable y qué es componente enlazado.
- **Técnicas de análisis:** matriz de privilegios y de red; verificar aislamiento del runner y
  operación offline; comprobar que ningún crate se despliega por separado.
- **Vistas generadas:** VW-07, VW-08, VW-09 y VW-10, en
  [`c4/deployment-views.md`](c4/deployment-views.md).

## VP-05 — Interacción y runtime

- **Concerns encuadrados:** CN-01, CN-02, CN-03, CN-04, CN-05, CN-06, CN-11, CN-12, CN-17.
- **Stakeholders:** SH-01, SH-02, SH-06, SH-08, SH-10, SH-11.
- **Model kinds:** interacción, mensaje, precondición/postcondición y estado observable.
- **Lenguaje/notación:** Mermaid `sequenceDiagram` con `alt`/`else` para caminos negativos.
- **Técnicas de construcción:** derivar cada secuencia de un caso de uso (`UC-001`..`UC-006`) y de
  una operación de `CorePort`; declarar precondiciones (revisión, hash, protocolo).
- **Técnicas de análisis:** análisis de caminos feliz y negativo; verificar que las operaciones
  puras no acceden a filesystem o red; verificar precondiciones de hash y de revisión.
- **Vista generada:** VW-11, en
  [`c4/runtime-sequences.md`](c4/runtime-sequences.md).

## Correspondencia viewpoint → view → concerns

| Viewpoint | Vistas | Concerns cubiertos |
|---|---|---|
| VP-01 Contexto | VW-01 | CN-01, CN-05, CN-07, CN-15 |
| VP-02 Contenedores | VW-02 | CN-01, CN-05, CN-07, CN-08, CN-12, CN-14, CN-15 |
| VP-03 Componentes | VW-03, VW-04, VW-05, VW-06 | CN-01, CN-02, CN-03, CN-04, CN-05, CN-06, CN-08, CN-09, CN-11, CN-13, CN-14, CN-15, CN-16, CN-17 |
| VP-04 Deployment | VW-07, VW-08, VW-09, VW-10 | CN-01, CN-06, CN-07, CN-08, CN-10, CN-12 |
| VP-05 Interacción y runtime | VW-11 | CN-01, CN-02, CN-03, CN-04, CN-05, CN-06, CN-11, CN-12, CN-17 |

## Reglas de gobierno

- Toda view se construye conforme a un viewpoint de este documento; una view sin viewpoint es
  inválida.
- Añadir un concern `P0` exige asignarle al menos una view (criterio de salida R4).
- Un cambio de boundaries, contenedores o protocolos exige actualizar
  [`stakeholders-concerns.md`](stakeholders-concerns.md) y las views afectadas.
- La autoridad semántica de los concern y su prioridad es
  [`stakeholders-concerns.md`](stakeholders-concerns.md); la de los estados es
  `docs/00-governance/status-model.md`.
