# ADR-0008: Catálogos firmados y procedencia

- Estado: accepted
- Fecha: 2026-10-05
- Propietario: Seguridad
- Requisitos: FR-CAT-001, NFR-SEC-001, NFR-OBS-001

## Contexto

El catálogo es un conjunto versionado e inmutable de definiciones que declara qué se puede elegir; `THR-CAT-001` describe el catálogo manipulado y `THR-SUP-001` la dependencia comprometida. `RSK-002` cubre la ejecución arbitraria desde catálogo/UI, controlada por ausencia de shell y por firmas. El registro de fuentes marca `SRC-008` (NIST SSDF/SLSA/Sigstore) como institucional con captura pendiente, y DEC-006 quedó resuelta el 2026-10-05: **Sigstore para releases + firma offline evaluada**.

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
2. La **política de firma queda fijada por DEC-006 (aceptada el 2026-10-05)**: **Sigstore para releases + firma offline evaluada**. Las opciones «sin firma» y «Minisign en solitario» quedan descartadas.
3. La **firma offline se adopta como evaluada** conforme a DEC-006; la conformidad con la fuente institucional `SRC-008` queda pendiente de captura (**no verificado — fuente primaria pendiente**).
4. La verificación de procedencia es requisito de carga, no una opción de UI; su fallo se reporta como diagnóstico tipado y redactado.

## Consecuencias

- Se reduce el riesgo de catálogo manipulado y de dependencia comprometida.
- La distribución de catálogos requiere infraestructura de firma y gestión de claves.
- La firma offline añade coste operativo y de verificación; su alcance queda definido por DEC-006.
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

FR-CAT-001, NFR-SEC-001, NFR-OBS-001. Resuelve DEC-006 (2026-10-05). Relaciona `THR-CAT-001`, `THR-SUP-001`, `RSK-002`, `RSK-007`, `DM-CATALOG`, `DM-POLICY`, `SRC-008`.
