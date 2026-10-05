# Resultado Go/No-Go — planning-v1

- Documento: DOC-GOV-GNG-001 · Estado: draft · Propietario: Arquitectura · Fecha: 2026-10-05
- Resultado: **No-Go: planificación en curso.**
- Alcance: autorizar el inicio de `MVP-0` (código funcional de producto).

## Motivo (resumen)

La planificación documental está trazada de extremo a extremo, pero **no puede autorizarse el desarrollo** porque:

1. Hay **8 decisiones P0 sin resolver** (DEC-001..DEC-008) que fijan alcance, identidad de datos, target de instalación, privilegios/transporte del runner, licencia y soporte.
2. Ningún ADR está `accepted` (los 8 están `proposed`): las decisiones estructurales no han sido aprobadas por la persona propietaria.
3. Los **contratos P0** (JSON Schema, protocolo del runner) no existen como artefactos con fixtures válidos e inválidos; están correctamente bloqueados por dependencias.
4. Persisten **riesgos P0** (RSK-001/002/003/004/007/008) sin tratamiento cerrado.
5. Las fuentes primarias de datos mutables (ArchWiki, archinstall, Tauri, JSON Schema, WCAG) están `pending-capture`.

## Cobertura de gates

| Gate | Estado | Falta para `complete` |
|---|---|---|
| G0 Fuentes | complete | — (evidencia v5.1 íntegra; capturas primarias pendientes como tareas) |
| G1 Problema/usuarios | partial | aprobar objetivos/personas; métricas no verificadas |
| G2 Alcance/requisitos | partial | resolver DEC-001/008; aprobar FR/NFR (hoy draft) |
| G3 Arquitectura | partial | aceptar ADR-0001/0002; aprobar C4/módulos |
| G4 Datos | partial | resolver DEC-002; aceptar modelo e identidad; schemas |
| G5 Interfaces | partial | aceptar CorePort/DTO; schemas de errores/eventos |
| G6 UX/diseño | partial | validación de accesibilidad; aprobar tokens preservados |
| G7 Validación | partial | fijar AST/operadores; corpus ejecutable |
| G8 Seguridad | partial | resolver DEC-004; cerrar RRSK-001/002/008 |
| G9 Calidad | partial | CI en verde; SBOM/firmas tras DEC-005/006 |
| G10 Delivery | partial | backlog priorizado y con owners; releases |

## Criterios de salida (para pasar a Go)

- G0–G10 en `complete` con evidencia ejecutable y sin P0 abiertos asociados.
- DEC-001..DEC-008 resueltas y registradas en `decision-register.md`.
- ADR-0001..0008 con estado `accepted` (por el propietario humano).
- Contratos P0 con JSON Schema Draft 2020-12, `additionalProperties:false`, fixtures válidos **y** inválidos en CI.
- `sha256sum --check reference/v5.1/SHA256SUMS` en verde y sin regresiones de trazabilidad.

## Consecuencia inmediata

Autorizado únicamente: trabajo documental, captura de fuentes primarias y validaciones. **No** autorizado: código funcional, prototipo ejecutable, acceso a discos/root, shell arbitraria ni runner real.
