---
id: DOC-SEC-SECRET-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - security
reviewers:
  - independent-reviewer
---

# Política de secretos

Fase R9 (issue `AUD-018`). Define cómo se suministran, usan, almacenan y borran los secretos en
ArchMaker. La amenaza asociada es `THR-SEC-001` (secreto en logs/draft); el modelo de privilegios
se lee en `docs/08-security/privilege-model.md`. Las decisiones se citan por su identificador
(`DEC-*`) y su estado se lee en `docs/00-governance/decision-register.md`; este documento no lo
reafirma.

## Principios

1. **Deny-by-default**: ningún secreto se incluye en el producto, el repositorio ni el catálogo.
2. **Just-in-time**: el secreto se suministra en el momento de uso y no se persiste.
3. **Mínimo privilegio**: cada secreto se limita al proceso y a la operación que lo necesita.
4. **Redacción**: los secretos nunca aparecen en logs, diagnósticos, eventos ni borradores.
5. **Trazabilidad sin valor**: se registra el uso (quién, cuándo, para qué), nunca el valor.

## Clasificación

| Clase | Ejemplo | MVP | Manejo |
|---|---|---|---|
| Secreto de release | Identidad OIDC del workflow | Sí (CI) | Keyless; sin clave privada persistente (`DEC-006`). |
| Secreto de usuario | Credencial introducida por el usuario | No | Fuera de alcance del MVP; sin shell ni instalación real. |
| Secreto de runner | Token de sesión del runner (v1) | No (v1) | Canal temporal, expiración y borrado; ver `runner-hazard-analysis.md`. |
| Secreto de desarrollo | Tokens locales de prueba | Sí | Nunca versionados; `.env` fuera de git. |

## Reglas

1. **Nunca en el repositorio**: no se versionan secretos ni ficheros `.env`; se verifica en el gate
   de higiene y en la revisión de PR.
2. **Nunca en `reference/**` ni en contratos**: la evidencia inmutable no contiene secretos.
3. **Redacción obligatoria**: logs, diagnósticos (`Diagnostic`) y eventos redactan cualquier valor
   sensible; un diagnóstico nunca transporta el secreto, solo su referencia.
4. **`SecretRef`**: el código maneja referencias opacas, no valores; el valor se resuelve en el
   borde de uso y se descarta.
5. **Borrado de memoria**: el secreto se borra de memoria en cuanto es posible; no se copia a
   estructuras de larga vida.
6. **Canal temporal**: en v1, el secreto de sesión viaja por el transporte autenticado
   (Unix socket con autenticación de sesión, `DEC-004`) y expira.
7. **CI**: los secretos de CI viven en el almacén del proveedor, se exponen al job mínimo y se
   enmascaran; las credenciales de larga duración están prohibidas donde la identidad OIDC basta.
8. **Rotación**: un secreto sospechoso se rota y se revoca; el incidente se registra según
   `vulnerability-response.md`.

## Pruebas negativas (diseño)

| ID | Given | When | Then |
|---|---|---|---|
| TST-SEC-001 | Un diagnóstico/log con un valor sensible | Se emite | El valor aparece redactado; nunca el original. |
| TST-SEC-002 | Un `.env` o secreto en el árbol | Se ejecuta el gate de higiene | El gate falla y bloquea el cierre. |
| TST-SEC-003 | Un secreto de sesión (v1) | La sesión expira | El secreto deja de ser válido y se descarta. |

## Trazabilidad

| Referencia | Relación |
|---|---|
| `THR-SEC-001` | Amenaza de secreto en logs/draft. |
| `DEC-006` | Firma keyless sin clave privada persistente. |
| `DEC-004` | Canal temporal autenticado para el runner (v1). |
| `DOC-SEC-RISK-001` | Modelo de riesgo residual. |

## Límites declarados

- No implementa gestores de secretos ni integración con llaveros; define política y pruebas.
- No cubre secretos de usuario final (fuera del MVP).
