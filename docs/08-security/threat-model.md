# Threat model

| ID | Amenaza | Frontera | Control |
|---|---|---|---|
| THR-IMP-001 | JSON/ZIP bomb o nesting. | Import | Límites antes y durante parse. |
| THR-CAT-001 | Catálogo manipulado. | Catalog/core | Digest, firma y provenance. |
| THR-XSS-001 | HTML/URL maliciosa. | Catalog/UI | Escapado, allowlist URL y CSP. |
| THR-IPC-001 | Invocación Tauri no autorizada. | WebView/backend | Capabilities, permissions y scopes. |
| THR-FS-001 | Traversal o overwrite. | Backend/FS | Picker, canonical path y atomic write. |
| THR-RUN-001 | WebView controla root. | Desktop/runner | Proceso separado y protocolo tipado. |
| THR-RUN-002 | TOCTOU tras confirmar. | Plan/runner | Hashes, snapshot de target y re-preflight. |
| THR-SEC-001 | Secreto en logs/draft. | Todas | SecretRef, redaction y canal temporal. |
| THR-UPD-001 | Update comprometida. | Release/client | **En alcance MVP (DEC-007).** Canal firmado (DEC-006), HTTPS, pin de canal, verificación de firma, rollback y fail-open offline; ver ADR-0009. Cerrable antes de MVP-0. |
| THR-SUP-001 | Dependencia comprometida. | Build | Lock, audit, SBOM y provenance. |
| THR-TEN-001 | Cruce de tenant. | Enterprise | Tenant from claims, RLS/tests y audit. |

## Notas (2026-10-05)

- Los controles de THR-RUN-001, THR-RUN-002 y THR-FS-001 quedan ligados a DEC-004 (proceso separado, transporte Unix socket + autenticación de sesión y elevación fuera del WebView; ver `privilege-model.md` y ADR-0004).
- THR-UPD-001 queda ligado a ADR-0009 (canal firmado, fail-open offline) y a DEC-006/DEC-007.
- THR-CAT-001 y THR-SUP-001 quedan ligados a DEC-006 (firma/procedencia) y ADR-0008.
