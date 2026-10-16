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

| Id                     | Caso                                                   | Estado documentado                                                                 |
| ---------------------- | ------------------------------------------------------ | ---------------------------------------------------------------------------------- |
| F7-MOTOR-REAL          | Motor real; fallo sin mock                             | executable (parcial en CI; E2E-01 con WASM)                                        |
| F7-GEN-EDIT-EXPORT     | Generar A → editar B → exportar B; invalidar obsoletos | executable (parcial en CI; E2E-04 con WASM)                                        |
| F7-DBLCLICK            | Reentrada / doble clic mientras `busy`                 | executable                                                                         |
| F7-BLOCKING-VALIDATION | Validación bloqueante                                  | executable (`core_contract`)                                                       |
| F7-TARGET-INCOMPATIBLE | Target incompatible                                    | executable (`export-delivery`)                                                     |
| F7-REG-F0…F5           | Regresión F0–F5                                        | executable                                                                         |
| F7-REG-F6              | Regresión F6 (mensajes; P95 no medido)                 | executable; **P95 pending** (sin 30 muestras)                                      |
| F7-OFFLINE             | Offline según contrato                                 | **web passed** (local/`--with-e2e`); **escritorio pending** (sin humo de red)      |
| F7-E2E-X3              | Tres E2E consecutivos WASM                             | **passed** en última verificación local (3 intentos, retries=0); CI vía `--with-e2e` |

Detalle de comandos, hooks y `lastVerified`: `f7-required-cases.json`.

## Limitaciones registradas (sin etiqueta de aceptación independiente)

1. El dueño puede mergear sin checks bloqueantes; el verde de colaborador externo no es el mismo gate.
2. P95 F6 (inicialización ≤ 3.000 ms; resolve+validate+manifiesto ≤ 1.000 ms; n=30) sigue **pending**. No se fabrican cifras.
3. Humo GUI F5 no es E2E de `xdg-desktop-portal`; el diálogo nativo se prueba con picker simulado.
4. Offline **escritorio** (binario ya construido sin red) no tiene humo en CI; queda **pending** / documentado.

## Offline (contrato)

- **Escritorio:** binario ya construido sin red. Sin humo de red en CI; **pending** hasta corrida de evidencia fuera del árbol con portal/ACL reales.
- **Web:** página ya cargada, luego `context.setOffline(true)`. Cubierto por `apps/web/wasm-offline.playwright.test.ts` (requiere pkg WASM + Chromium). `lastVerified.web=passed` no cierra la aceptación funcional del conjunto.

## Cómo ejecutar

```bash
# Solo valida inventario + emite métricas/manifiesto pending (CI)
python3 tools/run_f7_candidate_matrix.py --validate-only

# Ejecuta el subconjunto local/CI (sin exigir E2E WASM ×3 / offline)
python3 tools/run_f7_candidate_matrix.py --execute

# Con E2E WASM ×3 + offline web si el pkg WASM y Chromium están listos
python3 tools/run_f7_candidate_matrix.py --execute --with-e2e
```

Las corridas reales de evidencia viven fuera del árbol en `closure-evidence/<source-sha>/<run-id>/`. El manifiesto de esa corrida debe conservar `result=pending` hasta aceptación independiente (gobierno), y listar las limitaciones en `residuals` (P95, escritorio offline, dueño sin checks bloqueantes, sin `RC_SHA_FUNCIONAL`).

## Fuera de alcance

<<<<<<< HEAD
La autorización de canal (AppImage / Neubat) vive en `f8-governance.md` y DEC-011; no cierra
esta matriz ni sustituye `RC_SHA_FUNCIONAL`. F9 publica bytes del canal autorizado; no es
aceptación funcional F7.
=======
F8 alinea gobierno sin aceptar MVP-6 ni resolver ADR-0010 (`f8-governance.md`). F9 sigue
**blocked**: no tags/releases/paquetes sin autorización explícita. Este documento no toca ADR-0010, MVP-6 ni release.
>>>>>>> d92faf8 (feat(cierre): residuales F7 E2E WASM×3 y offline web)
