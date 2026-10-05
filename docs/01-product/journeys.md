---
id: DOC-PROD-JNY-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - product
reviewers:
  - independent-reviewer
---

# Journeys

- ID: DOC-PROD-JNY-001
- Estado: accepted
- Propietario: Producto
- Última revisión: 2026-10-05
- Requisitos relacionados: JNY-001..JNY-004; FR-DRAFT-001, FR-CAT-001, FR-PRESET-001, FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001, FR-EXPORT-001, FR-IMPORT-001, FR-RUN-001, FR-ENT-001
- Sustituye / sustituido por: —

## Reglas

1. Cada journey mapea a casos de uso de `use-cases.md` y a requisitos de `requirements.md`; no introduce IDs nuevos.
2. Los pasos describen intención de usuario y resultado observable, no comandos ni shell.
3. `JNY-001..JNY-002` son MVP; `JNY-003` es v1; `JNY-004` es Enterprise (no se mezclan fases).

## JNY-001 — Crear, configurar, validar y exportar

- **Actor:** PER-001 (guiado), PER-002 (avanzado).
- **Precondición:** catálogo versionado disponible (FR-CAT-001); aplicación operativa sin red (NFR-OFF-001).
- **Pasos:**
  1. Abrir la aplicación y crear un draft nuevo (UC-001).
  2. Elegir las definiciones del catálogo y valores `SelectionValue` para el objetivo.
  3. Aplicar, si procede, un preset como patch y revisar el `ChangeSet` (UC-001; FR-PRESET-001).
  4. Resolver capacidades y conflictos sobre el draft (UC-003; FR-RESOLVE-001).
  5. Revisar el resultado manual/derived/locked/conflict y las decisiones explicadas (UC-004; FR-VALIDATE-001).
  6. Corregir cualquier error bloqueante señalado.
  7. Construir el manifest canónico (UC-005; FR-MANIFEST-001).
  8. Elegir target y exportar artefacto y reporte (UC-005; FR-EXPORT-001).
- **Postcondición:** draft versionado persistido + `Resolution` reproducible + `Manifest` inmutable + `Artifact` con digests, o un conjunto de diagnósticos bloqueantes que impide exportar.
- **UC asociados:** UC-001, UC-003, UC-004, UC-005.
- **FR asociados:** FR-DRAFT-001, FR-CAT-001, FR-PRESET-001, FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001, FR-EXPORT-001.
- **Excepciones / caminos alternativos:**
  - Catálogo ausente o con digest de procedencia inválido → se bloquea el paso 2 con `Diagnostic`; no se resuelve.
  - Preset incompatible con selección lock/manual → conflicto reportado; el draft no se muta silenciosamente.
  - Conflicto de capacidades no resoluble → la `Resolution` queda marcada como bloqueante; no se construye manifest.
  - Target no habilitado → exportación rechazada con `CoreError`; el manifest sigue válido.
  - Fallo de E/S al guardar o exportar → escritura atómica; el estado previo permanece intacto.

## JNY-002 — Importar y migrar

- **Actor:** PER-001, PER-002.
- **Precondición:** documento local heredado v5.1 disponible; no se requiere red. *(Formato v5.1: no verificado — fuente primaria pendiente.)*
- **Pasos:**
  1. Seleccionar el documento local heredado.
  2. Detectar formato y versión (UC-002).
  3. Simular la migración y mostrar el informe previo de cambios y pérdidas.
  4. Confirmar explícitamente la migración.
  5. Generar un nuevo draft (identidad nueva) más el informe de migración.
- **Postcondición:** nuevo draft importado + informe que lista cambios y pérdidas explícitas; el documento original permanece byte-idéntico.
- **UC asociados:** UC-002.
- **FR asociados:** FR-IMPORT-001; apoya NFR-MIG-001.
- **Excepciones / caminos alternativos:**
  - Formato no reconocido o corrupto → rechazo con `CoreError`; sin draft.
  - Campos desconocidos → se marcan como pérdida explícita; nunca se descartan en silencio.
  - Pérdida bloqueante y usuario no confirma → no se crea draft.
  - Contenido que pretende comandos/hooks/shell → no se interpreta ni ejecuta; se marca no soportado.

## JNY-003 — Instalar (v1)

- **Actor:** PER-001, PER-002.
- **Precondición:** runner compatible disponible y manifest válido; fuera del MVP (v1). *(Transporte del runner: DEC-004; motor: DEC-003.)*
- **Pasos:**
  1. Verificar compatibilidad del runner y preflight del entorno (UC-006).
  2. Derivar el `InstallationPlan` inmutable desde el manifest.
  3. Ejecutar dry-run y revisar el plan y sus precondiciones.
  4. Confirmar la ejecución ligada al hash del plan.
  5. Ejecutar las operaciones tipadas allowlisted con journal append-only.
  6. Mostrar resultado y journal.
- **Postcondición:** journal completo y resultado de la ejecución; sistema en el estado indicado por el plan, o aborto seguro con causa registrada.
- **UC asociados:** UC-006.
- **FR asociados:** FR-RUN-001; apoya NFR-SEC-001, NFR-OBS-001.
- **Excepciones / caminos alternativos:**
  - Hash de confirmación distinto del plan → ejecución rechazada.
  - Operación fuera del allowlist o entrada no tipada → rechazo en el runner; nunca se interpreta script.
  - Fallo de preflight → aborto sin mutación y con diagnóstico.
  - Interrupción durante la ejecución → journal preservado; sin operaciones implícitas de reparación.

## JNY-004 — Gobernar flotas (Enterprise)

- **Actor:** PER-004.
- **Precondición:** organización con RBAC y `PolicyBundle` firmado; servicio Enterprise, no producto local. *(Firma: DEC-006.)*
- **Pasos:**
  1. Definir o seleccionar perfil y policies de organización (UC-007).
  2. Validar la firma y el ámbito de aplicación.
  3. Aprobar el perfil y la campaña de despliegue.
  4. Desplegar a los objetivos gobernados respetando `allow`/`deny`/`require`/`lock`.
  5. Registrar la aprobación y el resultado en la auditoría.
- **Postcondición:** despliegue auditado dentro del tenant, con traza de aprobación y aplicación de policies.
- **UC asociados:** UC-007.
- **FR asociados:** FR-ENT-001; apoya NFR-SEC-001, NFR-OBS-001.
- **Excepciones / caminos alternativos:**
  - Acceso cruzado entre tenants → denegado y registrado (THR-TEN-001).
  - Firma de policy inválida → aplicación rechazada.
  - Policy en conflicto con selección manual → prevalece la política; conflicto explicado.
  - Sin aprobación registrada → despliegue bloqueado.
