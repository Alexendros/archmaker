---
id: DOC-DATA-CANON-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - data
reviewers:
  - independent-reviewer
---

# Perfil de canonicalización v1

ID `DOC-DATA-CANON-001` · Estado `accepted` · Propietario Datos · Inmutable · Versión de perfil
`archmaker-canonicalization-profile@v1` · Fecha 2026-10-06.

Autoridad semántica de la fase R5 (`AUD-009`/`AUD-010`). Este perfil fija las doce decisiones
obligatorias de canonicalización y hashing. Su decisión de arquitectura es `ADR-0007`, ya
`accepted`. El perfil `v1` es **inmutable**: cualquier cambio de estas reglas exige un perfil nuevo
(`v2`) y un `domainTag` nuevo; los digests de versiones distintas no son comparables.

## Alcance

Rige la serialización canónica, el cálculo del digest y las transformaciones de perfil previas
sobre el payload de Catalog, Resolution, Manifest, Artifact y Plan. No rige la validación de
esquema (JSON Schema, ADR-0005) ni la conversión de unidades de dominio.

## Relación con ADR-0007

- `ADR-0007` decide **qué** se hace y por qué; este perfil define **cómo** se ejecuta, sin ambigüedad.
- RFC 8785 (JCS) es la serialización normativa adoptada de forma definitiva por `ADR-0007`.
- El corpus `contracts/test-vectors/canonicalization/` y `tools/canonicalize.py` materializan el
  perfil y son su evidencia ejecutable.

## Decisiones normativas

Las cláusulas siguientes son **normativas**. El verificador de referencia las implementa.

### 1. Serialización normativa

La serialización es **RFC 8785 (JCS)**. La salida es un flujo UTF-8 sin espacios, sin saltos de
línea y sin BOM. No se admite serialización no canónica ni orden de inserción.

### 2. UTF-8 y BOM

La entrada se interpreta como UTF-8 estricto (RFC 3629). Un BOM inicial (`EF BB BF`) se **rechaza**;
no se elimina en silencio. La salida nunca lleva BOM.

### 3. Normalización Unicode

**No hay normalización.** Los puntos de código se preservan tal cual. NFC es un requisito de
validación de los productores, no una transformación de la canonicalización; por tanto, dos formas
canónicamente equivalentes (NFC y NFD) producen bytes y digests **distintos**.

### 4. Orden de propiedades

Las claves de un objeto se ordenan por la secuencia de unidades de código **UTF-16**, en orden
lexicográfico (regla de JCS). El orden de inserción no influye.

### 5. Arrays ordenados y no ordenados

El orden de un array es semántico y se **preserva**. Un conjunto no ordenado se representa como un
array ordenado por una clave estable declarada (p. ej. `id`, comparación UTF-16) **antes** de
canonicalizar. Esta ordenación es un paso de perfil, nunca de JCS.

### 6. Representación numérica

Todo número JSON es IEEE-754 **binary64** y se serializa con la semántica `Number::toString` de
ECMAScript (JCS):

- `-0` se serializa como `0`.
- Los enteros sin parte fraccionaria se escriben sin decimal (`1`, no `1.0`).
- Se usa la representación decimal más corta que reencoda el mismo binary64.
- Se usa notación exponencial fuera del rango plano de JCS (`1e+21`, `1e-7`).
- Se rechazan `NaN`, `±Infinity` y cualquier valor fuera del rango de binary64.
- Un literal entero fuera de ±2^53 se convierte al binary64 más cercano (pérdida de precisión
  documentada, no error); un valor que desborda binary64 se rechaza.

### 7. Tratamiento de unidades

Una cantidad se representa como `{ "value": <number>, "unit": "<code>" }`. El código de unidad
pertenece a un vocabulario cerrado, sensible a mayúsculas, y forma parte del payload hasheado. La
canonicalización **no** convierte ni normaliza unidades; la coherencia dimensional es
responsabilidad de la validación de dominio.

### 8. Campos excluidos

Antes de serializar se elimina recursivamente la siguiente denylist de campos efímeros o no
intencionales:

`createdAt`, `updatedAt`, `loadedAt`, `lastAccessedAt`, `uiState`, `localPath`, `sourcePath`,
`absolutePath`, `workingDirectory`, `diagnostics`, `diagnosticLog`.

`secretRef` se conserva como **referencia**; el valor del secreto nunca se serializa.

### 9. Separación de dominio

El digest se calcula sobre una trama con etiqueta de dominio:

```text
digest = sha256( utf8(domainTag) ‖ 0x00 ‖ utf8(bytesCanónicos) )
domainTag = "archmaker:" + <tipo> + ":" + <schemaVersion>
```

Ejemplo: `archmaker:manifest:v1`. El separador `0x00` evita colisiones entre etiquetas y payload.

### 10. SHA-256 y formato hexadecimal

El algoritmo es **SHA-256** (FIPS 180-4). El digest es una cadena de 64 caracteres hexadecimales en
**minúsculas**, sin prefijo `0x`. En el DTO se expresa como `Digest { algorithm: "sha256", value }`.

### 11. Digest semántico frente a binario

Se distinguen dos digests, nunca intercambiables:

- **`contentDigest`** (semántico): SHA-256 del payload canónico, tras las decisiones 5 y 8. Es la
  identidad reproducible.
- **`artifactDigest` / `binaryDigest`** (binario): SHA-256 de los bytes exactos del artefacto. Es la
  integridad del binario.

### 12. Versionado del perfil

El perfil se identifica como `archmaker-canonicalization-profile@v1`. El `domainTag` embebe el
`<schemaVersion>` del payload. Cambiar cualquiera de las doce decisiones o la adopción de JCS
obliga a publicar un perfil `v2` con `domainTag` nuevo; los digests de versiones distintas no son
comparables.

## Cobertura de JCS

| Decisión | ¿Cubierta por JCS? |
|---|---|
| 1 Serialización | Sí (es la serialización de JCS) |
| 2 UTF-8/BOM | Parcial (JCS emite UTF-8; el rechazo de BOM es del perfil) |
| 3 Unicode | Sí (JCS no normaliza) |
| 4 Orden de propiedades | Sí |
| 5 Arrays/sets | No (decisión de archmaker) |
| 6 Números | Sí |
| 7 Unidades | No (decisión de archmaker) |
| 8 Campos excluidos | No (decisión de archmaker) |
| 9 Separación de dominio | No (decisión de archmaker) |
| 10 SHA-256/hex | No (elección externa a JCS) |
| 11 Digest semántico/binario | No (decisión de archmaker) |
| 12 Versionado del perfil | No (decisión de archmaker) |

## Ejemplos normativos

Estos ejemplos proceden del corpus golden y son **normativos**; su byte-identidad y digest los
verifica `tools/canonicalize.py`.

Bytes canónicos (texto) con dominio `archmaker:manifest:v1`:

| Vector | Entrada | Bytes canónicos |
|---|---|---|
| `m-key-order` | `{"b":1,"a":2,"c":3,"za":0}` | `{"a":2,"b":1,"c":3,"za":0}` |
| `n-negative-zero` | `{"a":-0.0,"b":0.0}` | `{"a":0,"b":0}` |
| `t-ephemeral-a` | `{"id":"x","createdAt":"2026-01-01T00:00:00Z",...}` | `{"id":"x"}` |
| `s-pre-sorted` | set ya ordenado por `id` | igual que `s-declared-sort` |

Los digests completos no se reproducen aquí: la fuente autoritativa es
`contracts/test-vectors/canonicalization/manifest.json` y sus ficheros de vectores. El verificador
`tools/canonicalize.py` es quien comprueba la byte-identidad y el digest de cada vector.

Invariantes normativas (cada una cubierta por vectores):

- Reordenar claves **no** cambia el digest (`m-key-order`).
- Cambiar una selección **sí** cambia el digest (`a-order-1` frente a `a-order-2`).
- Cambiar solo un campo efímero **no** cambia el digest (`t-ephemeral-a` frente a `t-ephemeral-b`).
- NFC y NFD producen digests distintos (`u-nfc` frente a `u-nfd`).
- El mismo payload en dominios distintos produce digests distintos (`d-domain-manifest` frente a
  `d-domain-plan`).
- Un set declarado se ordena antes de serializar y coincide con el set ya ordenado
  (`s-declared-sort` frente a `s-pre-sorted`).

## Ejemplos no normativos

Los siguientes ejemplos son **ilustrativos** y no obligan por sí mismos; sirven para explicar el
perfil. No forman parte del corpus verificado.

- `{"x":1.0}` se explica como `{"x":1}`; el valor `1.0` es un binary64 sin fracción.
- Un offset horario `2026-10-06T14:00:00+02:00` **no** se normaliza en canonicalización; la
  validación de dominio exige la forma UTC con `Z`. Este ejemplo ilustra por qué el vector
  `t-offset-opaque` difiere de `t-utc-preserved`.
- Un literal `9007199254740993` se explica como el binary64 `9007199254740992`; la precisión
  perdida es inherente a IEEE-754 y está fuera del alcance de JCS.

## Verificación

- `python3 tools/canonicalize.py` regenera bytes y digest y comprueba el corpus completo; debe
  terminar con código 0.
- El corpus cubre Unicode, números (enteros, flotantes, negativos, exponenciales, límites de
  precisión), mapas, arrays/sets, timestamps, dominios y errores de entrada.
- Paridad de implementación Rust nativo/WASM: se exigirá al arrancar el esqueleto (fase R12).

## Limitaciones declaradas

- La conformidad byte a byte con RFC 8785 queda **no verificada — fuente primaria pendiente**: la
  fuente primaria del RFC no está capturada en `docs/00-governance/source-register.md`.
- Nombres de estándares externos citados (RFC 8785, RFC 3629, FIPS 180-4, IEEE-754, ECMAScript)
  se usan como descripción de la decisión; su captura primaria en el registro de fuentes sigue
  pendiente y no se afirma conformidad verificada.
- El verificador independiente `tools/canonicalize.py` es herramienta de referencia de CI, no
  código de producto.
- La detección de claves duplicadas está disponible en la implementación Python; la implementación
  JS de paridad no la reproduce y el vector `e-duplicate-keys` se excluye de la paridad cruzada.

## Trazabilidad

- Decisión: `ADR-0007` (accepted).
- Corpus: `contracts/test-vectors/canonicalization/`.
- Verificador: `tools/canonicalize.py`.
- Requisitos: FR-MANIFEST-001, FR-RESOLVE-001, FR-EXPORT-001, NFR-DET-001.
