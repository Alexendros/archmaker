---
id: DOC-GOV-CON-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - governance
reviewers:
  - independent-reviewer
---

# Registro de contradicciones v5.1

| ID      | Evidencia                      | Contradicción                                                                               | Impacto                                               | Resolución requerida                                                                                        |
| ------- | ------------------------------ | ------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| CON-001 | Schema original vs instancia   | `mode: single` no existe en schema original.                                                | Migración                                             | Mapear a cardinalidad explícita.                                                                            |
| CON-002 | Reglas vs ambos schemas        | Se usa `count_gt`, operador no definido.                                                    | Resolver                                              | Especificar AST y tipos antes de implementarlo.                                                             |
| CON-003 | Schema original vs presets     | Tres selecciones usan arrays donde el schema solo admite escalares.                         | Datos                                                 | Definir `SelectionValue` discriminado.                                                                      |
| CON-004 | Instancia                      | `base-devel` aparece dos veces.                                                             | Identidad                                             | Resolver DEC-002 y crear detector de duplicados.                                                            |
| CON-005 | Schema v5.1                    | Existe `cmd`.                                                                               | Seguridad                                             | Rechazar ejecución; migrar como evidencia no ejecutable.                                                    |
| CON-006 | Schema original                | Existen hooks `pre/post` string.                                                            | Seguridad                                             | No migrarlos a contratos runtime.                                                                           |
| CON-007 | README                         | Declara `archmaker-v10.4-p3.yaml` e `i18n.es.json`, no recibidos.                           | Procedencia                                           | Recuperar o declarar definitivamente ausentes.                                                              |
| CON-008 | README                         | Declara `neubat_forge_v4_final.html`; se recibió `forge_v4_final.html`.                     | Procedencia                                           | No asumir equivalencia.                                                                                     |
| CON-009 | HTML/TSX                       | Catálogo, reglas, comandos, estado y UI están embebidos.                                    | Arquitectura                                          | Separar por puertos y módulos.                                                                              |
| CON-010 | Prototipos                     | React, Babel, Tailwind y fuentes dependen de CDN.                                           | Offline/CSP                                           | Empaquetar recursos; CSP `self` por defecto.                                                                |
| CON-011 | Datos                          | Afirmaciones de versiones, tamaños y compatibilidad no citan fuentes.                       | Exactitud                                             | Crear registro de evidencia académica/oficial.                                                              |
| CON-012 | Build heredado                 | Pipeline incluye `pacstrap` en configuración de UI.                                         | Privilegios                                           | Convertir a target/plan tipado de v1.                                                                       |
| CON-013 | Metadatos plan                 | Plan y definición del walking skeleton figuran `not-started` pese a existir implementación. | Fuente de verdad falsa                                | Recalcular metadatos desde evidencia (I0, walking-skeleton.md, implementation-plan-mvp0.md).                |
| CON-014 | I12 no cerrado                 | I12 no cerrado (revisión/aprobaciones pendientes).                                          | Tag no es aceptación final                            | Sign-off independiente + acta verificable; nuevo tag `implementation-baseline-mvp0.1`.                      |
| CON-015 | I11 placeholder                | I11 conserva `trusted_root.json` placeholder; sin release público.                          | SBOM, provenance, firmas, verificación offline reales | Materializar raíz no-placeholder; SBOM + provenance + firmas verificadas; release bloqueado hasta resolver. |
| CON-016 | G2-C04 pendiente               | G2-C04 pendiente → materializar IDs TST/VAL y matriz FR/NFR→prueba completa.                | Trazabilidad incompleta                               | Completar matriz FR/NFR→TST/VAL con casos válidos/inválidos por operación CorePort.                         |
| CON-017 | G6-C01/C03 abiertos            | G6-C01 y G6-C03 abiertos → aprobar contratos DS y ejecutar axe/manual/visual.               | Accesibilidad crítica                                 | Aprobar contratos design system; ejecutar axe-core + revisión manual WCAG 2.2 AA.                           |
| CON-018 | README inconsistente           | README describe repo documental y bloqueo previo.                                           | D-06                                                  | Actualizar README: walking skeleton existe, runner excluido; rutas de ejecución activas.                    |
| CON-019 | Issues AUD abiertos            | Issues AUD-001..023 abiertos aunque completos.                                              | D-07                                                  | Cerrar/superseder issues AUD con criterio demostrado; convertir restantes en issues MVP-0.1.                |
| CON-020 | `skip-if-missing` en workflows | Workflow raíz omite frontend si falta `check`; scripts raíz no exponen lint/typecheck/E2E.  | D-08                                                  | Prohibir `skip-if-missing` en controles requeridos; script ausente = fallar.                                |
| CON-021 | build Tauri local              | build Tauri local falló validando `app.security.devtools` con CLI instalada.                | D-09                                                  | Alinear schema/CLI y añadir build a CI; fijar `allowlist` vs `deny-by-default`.                             |
| CON-022 | tags baseline                  | Tags de baseline 6 commits detrás de HEAD.                                                  | D-10                                                  | NO mover tags; emitir versión sucesora `implementation-baseline-mvp0.1` apuntando al commit revisado.       |

## Cierre CON-013..CON-022 (baseline MVP-0.1)

Estado de las desviaciones D-01..D-10 al estabilizar `stabilization/mvp0.1` y el tag
`implementation-baseline-mvp0.1`. CON-001..CON-012 permanecen como deuda de migración v5.1
(fuera del slice walking skeleton).

| ID      | Estado                | Evidencia de cierre                                                                                            | Residual                                                  |
| ------- | --------------------- | -------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| CON-013 | resuelta              | Metadatos de `walking-skeleton.md` / `implementation-plan-mvp0.md` → `partial`/`complete` alineados con código | —                                                         |
| CON-014 | resuelta              | PR-07 E7 + tag anotado `implementation-baseline-mvp0.1`                                                        | Sign-off independiente formal sigue en review             |
| CON-015 | resuelta-con-residual | `trusted_root.json` con `_placeholder: false`; job `sign-verify` en `release.yml`                              | Release público / verificación offline en primer tag push |
| CON-016 | resuelta              | G2-C04 `complete`; matriz FR/NFR→TST/VAL en CI (`traceability_matrix.py`)                                      | —                                                         |
| CON-017 | resuelta-con-residual | G6-C01/C03 `complete`; `wcag_validate.py` + axe/visual; checklist manual RC                                    | Firma manual 1.4.1 en release candidate                   |
| CON-018 | resuelta              | README describe walking skeleton MVP-0.1; runner excluido                                                      | —                                                         |
| CON-019 | resuelta              | AUD-001..023 `superseded` en `backlog.md`; issues MVP-0.1-01..08                                               | —                                                         |
| CON-020 | resuelta              | CI exige `scripts.check`; ausencia = fallo; root `package.json` expone `check`                                 | —                                                         |
| CON-021 | resuelta              | `check_negative_capabilities.py` + ausencia `devtools` en schema Tauri v2                                      | Build Tauri completo sigue fuera del job mínimo           |
| CON-022 | resuelta              | Tag `implementation-baseline-mvp0.1` emitido (no se movió `implementation-baseline-mvp0`)                      | HEAD puede adelantar el tag hasta el próximo baseline     |
