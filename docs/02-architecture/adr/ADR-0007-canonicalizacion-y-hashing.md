# ADR-0007: Canonicalización y hashing

- Estado: accepted
- Fecha: 2026-10-05
- Propietario: Arquitectura
- Requisitos: FR-MANIFEST-001, FR-RESOLVE-001, FR-EXPORT-001, NFR-DET-001

## Contexto

`docs/03-data/canonicalization.md` fija el objetivo de obtener identificadores reproducibles para Catalog, Resolution, Manifest, Artifact y Plan, y propone serialización canónica JSON (RFC 8785 como candidata), UTF-8 sin BOM, orden canónico de claves y hash SHA-256 con etiqueta de dominio. La determinación (`NFR-DET-001`) y la confirmación por hash del runner (ADR-0004) dependen de una canonicalización estable y de excluir campos que no son parte de la intención. El prototipo heredado mezclaba estado de UI y datos de catálogo (`CON-009`).

Documentos de apoyo: `docs/03-data/canonicalization.md`, `docs/03-data/domain-model.md`, `docs/04-interfaces/runner-protocol.md`.

## Drivers

- `NFR-DET-001`: estabilidad ante reordenación de claves y ausencia de campos efímeros.
- Confirmación ligada a hash (ADR-0004) y verificación de integridad (`VAL-*`).
- Separar intención canónica de estado efímero y de paths locales.
- Política explícita de Unicode, sin normalización silenciosa.

## Opciones consideradas

1. **Serialización no canónica** (JSON.stringify y orden de inserción): digest inestable.
2. **Serialización canónica JSON según RFC 8785 (JCS) como candidata**: orden canónico, reglas numéricas y de cadenas fijadas.
3. **Formato binario propietario**: cerrado, difícil de auditar.

## Decisión propuesta

1. La **serialización canónica JSON** sigue un algoritmo canónico; **RFC 8785 (JCS) es la candidata** a confirmar contra fuente primaria antes de implementarlo (hoy **no verificado — fuente primaria pendiente**).
2. El **digest es SHA-256**, con etiqueta de dominio: `archmaker:manifest:v1\0<payload>`. La etiqueta desambigua el contexto y versiona el esquema de hash.
3. Los **campos efímeros** (`createdAt`, estado de UI, paths locales) **quedan fuera del payload hasheado**.
4. UTF-8 sin BOM; claves en orden canónico; arrays conservan orden solo si es semántico; los sets se ordenan por ID antes de canonicalizar.
5. La política de Unicode no normaliza silenciosamente: se decide de forma explícita y se cubre con vectores.

## Consecuencias

- Reordenar claves no cambia el digest; cambiar una selección sí.
- Dos adapters producen el mismo manifest digest para el mismo draft.
- El digest es estable y confirmable por el runner.
- Se asume coste de implementar y probar la canonicalización y de versionar la etiqueta de dominio si cambia el esquema.

## Riesgos

- `RSK-004` (migración pierde selecciones): un digest distinto tras migración es evidencia de cambio, no pérdida silenciosa.
- `RSK-001` (disco equivocado): el hash liga plan y manifest a la confirmación.
- Ambigüedad Unicode si la política no se fija: se mitiga con vectores y decisión explícita.

## Verificación

Vectores mínimos de `canonicalization.md`:

- Reordenar claves no cambia el digest.
- Cambiar una selección sí cambia el digest.
- Cambiar solo un timestamp efímero no cambia el digest.
- Dos adapters producen el mismo manifest digest.
- Equivalencia Unicode según la política decidida, sin normalización silenciosa.

Además: la conformidad exacta con RFC 8785 queda **no verificada — fuente primaria pendiente** hasta su captura.

## Sustituye

—

## Sustituido por

—

## Requisitos relacionados

FR-MANIFEST-001, FR-RESOLVE-001, FR-EXPORT-001, NFR-DET-001. Relaciona `DM-MANIFEST`, `DM-RESOLUTION`, `DM-PLAN`, ADR-0004.
