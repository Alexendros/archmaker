---
id: ADR-0007
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# ADR-0007: Canonicalización y hashing

- Estado: accepted
- Fecha: 2026-10-05 (aceptado en la fase R5, 2026-10-06)
- Propietario: Arquitectura
- Requisitos: FR-MANIFEST-001, FR-RESOLVE-001, FR-EXPORT-001, NFR-DET-001

## Contexto

`docs/03-data/canonicalization.md` fijaba el objetivo de obtener identificadores reproducibles
para Catalog, Resolution, Manifest, Artifact y Plan, y proponía serialización canónica JSON (con
RFC 8785 solo como candidata), UTF-8 sin BOM, orden canónico de claves y hash SHA-256 con etiqueta
de dominio. El `audit-baseline.md` (`AUD-F-09`) registraba dos carencias P0: la ausencia de las
doce decisiones normativas de canonicalización y la falta de fuente primaria verificada de
RFC 8785 (JCS). La determinación (`NFR-DET-001`), la confirmación por hash del runner (ADR-0004)
y la verificación de integridad (`VAL-*`) dependen de una canonicalización estable y de excluir
los campos que no forman parte de la intención. El prototipo heredado mezclaba estado de UI y
datos de catálogo (`CON-009`).

Esta decisión (fase R5, issues `AUD-009`/`AUD-010`) acepta el ADR, adopta RFC 8785 (JCS) de forma
**definitiva** como serialización normativa y fija el perfil inmutable
`archmaker-canonicalization-profile@v1` en `docs/03-data/canonicalization-profile-v1.md`, con su
corpus golden versionado en `contracts/test-vectors/canonicalization/` y el verificador de
referencia `tools/canonicalize.py`.

Documentos de apoyo: `docs/03-data/canonicalization-profile-v1.md`,
`docs/03-data/domain-model.md`, `docs/04-interfaces/runner-protocol.md`.

## Drivers

- `NFR-DET-001`: estabilidad ante reordenación de claves y ausencia de campos efímeros.
- Confirmación ligada a hash (ADR-0004) y verificación de integridad (`VAL-*`).
- Separar la intención canónica del estado efímero y de los paths locales.
- Política explícita de Unicode, sin normalización silenciosa.
- Paridad entre implementaciones (Rust nativo y WASM) sobre bytes y digest idénticos.

## Opciones consideradas

1. **Serialización no canónica** (`JSON.stringify` y orden de inserción): digest inestable.
2. **RFC 8785 (JCS) como serialización normativa**: orden canónico, reglas numéricas y de cadenas
   fijadas. Adoptada de forma definitiva por esta decisión.
3. **Formato binario propietario**: cerrado y difícil de auditar.

## Decisión

Se adopta **RFC 8785 (JSON Canonicalization Scheme, JCS)** de forma **definitiva** como
serialización normativa del perfil `v1`, y se fijan las doce decisiones obligatorias de la fase R5.
Las cláusulas 1 a 12 son **normativas**: cualquier implementación que produzca o consuma digests
de archmaker debe cumplirlas.

| # | Decisión | Valor normativo |
|---|---|---|
| 1 | Serialización normativa | RFC 8785 (JCS), definitiva; salida UTF-8 sin espacios ni saltos de línea. |
| 2 | UTF-8 y BOM | UTF-8 estricto (RFC 3629); se rechaza un BOM inicial en lugar de eliminarlo en silencio. |
| 3 | Normalización Unicode | Ninguna: los puntos de código se preservan. NFC es requisito de validación del productor, no una transformación de la canonicalización. |
| 4 | Orden de propiedades | Claves de objeto en orden lexicográfico por unidades de código UTF-16 (regla de JCS). |
| 5 | Arrays ordenados/no ordenados | El orden de un array es semántico y se preserva. Los conjuntos no ordenados se materializan como arrays ordenados por una clave estable (p. ej. `id`, comparación UTF-16) **antes** de canonicalizar; es un paso de perfil, no de JCS. |
| 6 | Representación numérica | Todo número es IEEE-754 binary64; serialización `Number::toString` de ECMAScript (JCS); `-0` → `0`; se rechazan NaN, ±Infinity y el desbordamiento de binary64. |
| 7 | Tratamiento de unidades | Cantidad = `{ "value": <number>, "unit": "<code>" }` con código de unidad de vocabulario cerrado y sensible a mayúsculas. La canonicalización **no** convierte unidades; el código forma parte del payload. |
| 8 | Campos excluidos | Denylist recursiva antes de serializar: `createdAt`, `updatedAt`, `loadedAt`, `lastAccessedAt`, `uiState`, `localPath`, `sourcePath`, `absolutePath`, `workingDirectory`, `diagnostics`, `diagnosticLog`. `secretRef` se conserva como referencia; el secreto nunca se serializa. |
| 9 | Separación de dominio | `sha256( utf8(domainTag) ‖ 0x00 ‖ utf8(bytesCanónicos) )`, con `domainTag = archmaker:<tipo>:<schemaVersion>` (p. ej. `archmaker:manifest:v1`). |
| 10 | SHA-256 y formato hex | SHA-256 (FIPS 180-4); digest = 64 caracteres hexadecimales en minúsculas, sin prefijo; en el DTO `Digest { algorithm: "sha256", value }`. |
| 11 | Digest semántico vs binario | `contentDigest` = digest del payload canónico (intención); `artifactDigest`/`binaryDigest` = SHA-256 de los bytes exactos del artefacto. No se confunden ni se comparan entre sí. |
| 12 | Versionado del perfil | `archmaker-canonicalization-profile@v1` es inmutable; cualquier cambio de estas decisiones o de JCS exige nueva versión de perfil y nuevo `domainTag`; digests de versiones distintas no son comparables. |

### Alcance de JCS y decisiones de archmaker

JCS cubre la serialización (decisiones 1 y 4), la emisión UTF-8 (2), el no-normalizado de cadenas
(3), la representación numérica (6) y el algoritmo de digest como elección externa a JCS (10). Las
siguientes no están cubiertas por JCS y son decisiones de archmaker: **5** (materialización de
sets), **7** (unidades), **8** (campos excluidos), **9** (separación de dominio), **11** (dos
digests) y **12** (versionado del perfil). El perfil v1 las fija de forma explícita.

### Estado de verificación de RFC 8785

La conformidad byte a byte con RFC 8785 queda **no verificada — fuente primaria pendiente**: la
fuente primaria del RFC no está capturada en `docs/00-governance/source-register.md`. Los bytes y
digests del corpus `contracts/test-vectors/canonicalization/` son la autoridad ejecutable del
perfil `v1` de archmaker; no constituyen una certificación del RFC. La paridad de implementación
Rust nativo/WASM se exige cuando arranque el esqueleto (fase R12).

## Consecuencias

- Reordenar claves no cambia el digest; cambiar una selección sí.
- Dos adapters producen el mismo manifest digest para el mismo draft.
- El digest es estable, versionado y confirmable por el runner.
- Se asume el coste de mantener el perfil inmutable, el corpus de vectores y la paridad entre
  implementaciones; cambiar el esquema exige versionar el `domainTag`.

## Riesgos

- `RSK-004` (migración pierde selecciones): un digest distinto tras migración es evidencia de
  cambio, no pérdida silenciosa.
- `RSK-001` (disco equivocado): el hash liga plan y manifest a la confirmación.
- Ambigüedad Unicode si la política no se fija: se mitiga con la decisión 3 (sin normalización) y
  los vectores Unicode.
- Pérdida de precisión al convertir literales enteros fuera de ±2^53 a binary64 (decisión 6):
  documentada y cubierta por el vector `n-precision-boundary`.

## Verificación

- Corpus golden `contracts/test-vectors/canonicalization/`: 34 vectores (25 válidos + 9 rechazos)
  en las categorías Unicode, números, mapas, arrays/sets, timestamps, dominio y errores.
- `tools/canonicalize.py` regenera bytes canónicos y digest, exige byte-identidad y digest
  idénticos, y pasa con código de salida 0; incluye paridad cruzada Python/JS.
- Vectores mínimos cubiertos: reordenar claves no cambia el digest; cambiar una selección sí;
  cambiar solo un timestamp efímero no; dos adapters producen el mismo digest; NFC y NFD difieren
  (sin normalización silenciosa); dominios distintos producen digests distintos.
- Conformidad exacta con RFC 8785: **no verificada — fuente primaria pendiente**.

## Sustituye

—

## Sustituido por

—

## Requisitos relacionados

FR-MANIFEST-001, FR-RESOLVE-001, FR-EXPORT-001, NFR-DET-001. Relaciona `DM-MANIFEST`,
`DM-RESOLUTION`, `DM-PLAN`, ADR-0004, `DOC-DATA-CANON-001`.
