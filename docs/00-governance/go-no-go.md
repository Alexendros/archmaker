# Resultado Go/No-Go — planning-v1

- Documento: DOC-GOV-GNG-001 · Estado: draft · Propietario: Arquitectura · Fecha: 2026-10-05
- Resultado: **No-Go: planificación en curso.**
- Alcance: autorizar el inicio de `MVP-0` (código funcional de producto).

## Motivo (resumen)

La planificación avanzó (ver «Progreso»), pero **no puede autorizarse el desarrollo** porque:

1. Los documentos de producto, arquitectura y datos siguen en `draft`/`in-review`: falta la **aprobación humana** (`accepted`) de FR/NFR, modelo de dominio y C4/módulos.
2. Persisten **riesgos P0** (RSK-001/002/003/004/007/008) sin tratamiento cerrado.
3. Las fuentes mutables **SRC-005 (Rust/RustSec)** y **SRC-008 (SSDF/Sigstore)** están `pending-capture`, y **SRC-007 (ARIA APG)** parcial; imprescindibles para G8/G9.
4. Los **contratos P0** existen con corpus válido/inválido, pero el **job de CI de corpus** está pendiente y la CI no se ha ejecutado en verde.
5. La **aprobación formal de Seguridad** (sign-off del privilege model) está pendiente antes de G8.

## Progreso desde la evaluación anterior

- **DEC-001..DEC-008 accepted** (2026-10-05): alcance de exportación, IDs globales `vendor.kind.id`, adapter archinstall, transporte del runner (Unix socket + auth), licencia (Apache-2.0/dual), firma (Sigstore + offline), **updater firmado en MVP** (desviación), **solo Arch x86_64** (desviación).
- **ADR-0001..ADR-0009 accepted**.
- **8 JSON Schema Draft 2020-12** cerrados (`additionalProperties:false`) + corpus válido/inválido + fixture de migración v5.1.
- **Fuentes SRC-002/003/006 capturadas**; SRC-007/008 partial; SRC-005 pendiente.
- **CI documental** con acciones fijadas por SHA; `sha256sum --check` en verde.

## Cobertura de gates

| Gate | Estado | Falta para `complete` |
|---|---|---|
| G0 Fuentes | complete | — (evidencia v5.1 íntegra; capturas primarias pendientes como tareas) |
| G1 Problema/usuarios | partial | aprobar objetivos/personas; métricas no verificadas |
| G2 Alcance/requisitos | partial | aprobar FR/NFR (hoy draft) |
| G3 Arquitectura | partial | aprobar C4/módulos (hoy draft) |
| G4 Datos | partial | meta-validación CI en verde; aprobar modelo e identidad |
| G5 Interfaces | partial | aprobar CorePort/DTO; ratificar cobertura CorePort |
| G6 UX/diseño | partial | validación de accesibilidad; aprobar tokens preservados |
| G7 Validación | partial | corpus ejecutable; ratificar `required`; RULE-GPU-001 no implementable |
| G8 Seguridad | partial | aprobación formal de Seguridad; cerrar RSK-001/002/008 |
| G9 Calidad | partial | CI en verde; capturar SRC-005/SRC-008; SBOM/firmas reales |
| G10 Delivery | partial | owners en backlog; releases |

## Criterios de salida (para pasar a Go)

- G0–G10 en `complete` con evidencia ejecutable y sin P0 abiertos asociados.
- FR/NFR, modelo de dominio y C4 aprobados (`accepted`).
- Contratos P0 con corpus válido **e** inválido validado en CI (job de corpus activo).
- CI en verde (documental + meta-validación + corpus).
- `sha256sum --check reference/v5.1/SHA256SUMS` en verde y sin regresiones de trazabilidad.
- Aprobación formal de Seguridad (G8).

## Consecuencia inmediata

Autorizado únicamente: trabajo documental, captura de fuentes primarias y validaciones. **No** autorizado: código funcional, prototipo ejecutable, acceso a discos/root, shell arbitraria ni runner real.
