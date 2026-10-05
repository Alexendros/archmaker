# Canonicalización y hashing

## Objetivo

Obtener identificadores reproducibles para Catalog, Resolution, Manifest, Artifact y Plan.

## Algoritmo propuesto

- Serialización canónica JSON conforme a un ADR específico; RFC 8785 es candidata.
- UTF-8, sin BOM.
- Claves ordenadas por el algoritmo canónico.
- Números restringidos por schema; evitar floats donde no sean imprescindibles.
- Arrays conservan orden si es semántico; los sets se ordenan por ID antes de canonicalizar.
- Campos efímeros (`createdAt`, UI state, paths locales) quedan fuera del payload hasheado.
- Hash inicial: SHA-256 con etiqueta de dominio, por ejemplo `archmaker:manifest:v1\0<payload>`.

## Vectores mínimos

- Reordenar claves no cambia digest.
- Cambiar una selección sí cambia digest.
- Cambiar solo timestamp efímero no cambia digest.
- Dos adapters producen el mismo manifest digest.
- Unicode equivalente debe seguir la política decidida por ADR; no normalizar silenciosamente.
