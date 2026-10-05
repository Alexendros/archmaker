# ADR-0008: Catálogos firmados y procedencia

- Estado: proposed
- Fecha: 2026-10-05
- Propietario: Seguridad
- Requisitos: FR-CAT-001, NFR-SEC-001, NFR-OBS-001

## Contexto

El catálogo es un conjunto versionado e inmutable de definiciones que declara qué se puede elegir; `THR-CAT-001` describe el catálogo manipulado y `THR-SUP-001` la dependencia comprometida. `RSK-002` cubre la ejecución arbitraria desde catálogo/UI, controlada por ausencia de shell y por firmas. El registro de fuentes marca `SRC-008` (NIST SSDF/SLSA/Sigstore) como institucional con captura pendiente, y DEC-006 mantiene abierta la política de firma (Minisign; Sigstore; ambos), con recomendación de Sigstore para releases y firma offline evaluada.

Documentos de apoyo: `docs/08-security/threat-model.md`, `docs/00-governance/decision-register.md`, `docs/00-governance/source-register.md`, `docs/00-governance/glossary.md` (`Catalog`, `Policy`).

## Drivers

- `THR-CAT-001`: detectar catálogo manipulado.
- `THR-SUP-001`: procedencia de la cadena de suministro.
- `NFR-SEC-001`: deny-by-default; lo no verificado no se acepta.
- `NFR-OBS-001`: procedencia observable y trazable.
- Los catálogos declaran `digest`, procedencia y evidencias (`DM-CATALOG`).

## Opciones consideradas

1. **Sin firma, solo digest**: detecta corrupción, no autentica el origen.
2. **Minisign**: firma offline verificable; gestión de claves propia.
3. **Sigstore**: firma/provenance ligada a la cadena de build/release.
4. **Ambos**: Sigstore para releases y firma offline evaluada para catálogos distribuidos por otros canales.

## Decisión propuesta

1. Todo catálogo debe declarar **digest** y **procedencia verificable**; un catálogo sin procedencia válida no se carga (`FR-CAT-001`).
2. La **política de firma queda como `DECISION-REQUIRED` (DEC-006)**. Opciones abiertas: Minisign, Sigstore o ambos; recomendación vigente: **Sigstore para releases + firma offline evaluada**.
3. La **firma offline se evalúa** pero no se adopta en firme hasta que DEC-006 se resuelva y exista fuente primaria (hoy **no verificado — fuente primaria pendiente**, `SRC-008`).
4. La verificación de procedencia es requisito de carga, no una opción de UI; su fallo se reporta como diagnóstico tipado y redactado.

## Consecuencias

- Se reduce el riesgo de catálogo manipulado y de dependencia comprometida.
- La distribución de catálogos requiere infraestructura de firma y gestión de claves.
- La firma offline añade coste operativo y de verificación; queda supeditada a DEC-006.
- Los catálogos históricos sin firma no se aceptan como runtime sin procedencia.

## Riesgos

- `RSK-002` (ejecución arbitraria): firmas y ausencia de shell reducen la superficie.
- `RSK-007` (supply-chain comprometida): provenance y SBOM complementan la firma.
- `RSK-005` (compatibilidad obsoleta): la procedencia no garantiza frescura; se requiere evidencia con caducidad.

## Verificación

- Prueba negativa: catálogo con digest/firma inválidos no se carga.
- Verificación de digest y procedencia como paso previo a `FR-CAT-001`.
- `VAL-*` de procedencia registra evidencia (URL, fecha de captura, verificador, caducidad).
- La conformidad con `SRC-008` queda **no verificada — fuente primaria pendiente** hasta su captura.

## Sustituye

—

## Sustituido por

—

## Requisitos relacionados

FR-CAT-001, NFR-SEC-001, NFR-OBS-001. Depende de DEC-006. Relaciona `THR-CAT-001`, `THR-SUP-001`, `RSK-002`, `RSK-007`, `DM-CATALOG`, `DM-POLICY`, `SRC-008`.
