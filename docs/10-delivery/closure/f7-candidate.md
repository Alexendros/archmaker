---
id: DOC-DEL-CLO-F7-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-CLO-EVD-001
    expectedDocumentStatus: draft
---

# F7 — Candidato funcional (matriz sobre un RC_SHA)

- Documento: DOC-DEL-CLO-F7-001 · Estado: draft · Propietario: Arquitectura
- Inventario ejecutable: `f7-required-cases.json`
- Runner / validador: `python3 tools/run_f7_candidate_matrix.py` (VAL-F7-CANDIDATE)
- Manifiesto de evidencia: `result` permanece **`pending`**. F7 **no** declara `RC_SHA_FUNCIONAL` ni aceptación independiente firmada.

## Qué es un RC_SHA candidato

Cualquier commit de `main` (o el `HEAD` de esta PR antes del merge) sobre el que se reejecuta la matriz obligatoria. Cambiar el SHA obliga a repetir la matriz. Un check de `pull_request` no valida `main` por sí solo (`GITHUB_SHA` puede ser el merge sintético).

## Matriz obligatoria

| Id                     | Caso                                                   | Ejecutable en CI/local sin WASM E2E                                                  |
| ---------------------- | ------------------------------------------------------ | ------------------------------------------------------------------------------------ |
| F7-MOTOR-REAL          | Motor real; fallo sin mock                             | Parcial (regla mock + E2E-01 si WASM)                                                |
| F7-GEN-EDIT-EXPORT     | Generar A → editar B → exportar B; invalidar obsoletos | Parcial (gate de generación + E2E-04 si WASM)                                        |
| F7-DBLCLICK            | Reentrada / doble clic mientras `busy`                 | Sí                                                                                   |
| F7-BLOCKING-VALIDATION | Validación bloqueante                                  | Sí (`core_contract`)                                                                 |
| F7-TARGET-INCOMPATIBLE | Target incompatible                                    | Sí (`export-delivery`)                                                               |
| F7-REG-F0…F6           | Regresión F0–F6                                        | Sí (reutiliza f0/f1/f2/f3, export-delivery, consented-import, negative_capabilities) |
| F7-OFFLINE             | Offline según contrato                                 | **pending** sin Playwright WASM / portal                                             |
| F7-E2E-X3              | Tres E2E consecutivos WASM                             | **pending** si WASM no está disponible                                               |

Detalle de comandos y hooks: `f7-required-cases.json`.

## Limitaciones registradas (sin etiqueta de aceptación independiente)

1. El dueño puede mergear sin checks bloqueantes; el verde de colaborador externo no es el mismo gate.
2. P95 F6 (inicialización ≤ 3.000 ms; resolve+validate+manifiesto ≤ 1.000 ms; n=30) sigue **pending**.
3. Humo GUI F5 no es E2E de `xdg-desktop-portal`; el diálogo nativo se prueba con picker simulado.

## Offline (contrato)

- **Escritorio:** binario ya construido sin red. Sin humo de red en CI; queda pending/not-applicable hasta corrida de evidencia fuera del árbol.
- **Web:** página ya cargada, luego sin red. Requiere Playwright + build WASM; si no hay WASM en el ejecutor, el caso queda **pending** explícito (no se marca passed).

## Cómo ejecutar

```bash
# Solo valida inventario + emite métricas/manifiesto pending (CI)
python3 tools/run_f7_candidate_matrix.py --validate-only

# Ejecuta el subconjunto local/CI (sin exigir E2E WASM ×3)
python3 tools/run_f7_candidate_matrix.py --execute

# Con E2E WASM ×3 si el pkg WASM y Chromium están listos
python3 tools/run_f7_candidate_matrix.py --execute --with-e2e
```

Las corridas reales de evidencia viven fuera del árbol en `closure-evidence/<source-sha>/<run-id>/`. El manifiesto de esa corrida debe conservar `result=pending` hasta aceptación independiente (F8+ gobierno), y listar las limitaciones en `residuals`.

## Fuera de alcance

La autorización de canal (AppImage / Neubat) vive en `f8-governance.md` y DEC-011; no cierra
esta matriz ni sustituye `RC_SHA_FUNCIONAL`. F9 publica bytes del canal autorizado; no es
aceptación funcional F7.
