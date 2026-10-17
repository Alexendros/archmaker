---
id: ADR-0017
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - product
reviewers:
  - independent-reviewer
---

# ADR-0017: ONLYOFFICE canónico y cardinalidad de navegadores

- Estado: proposed
- Fecha: 2026-10-17
- Propietario: Producto

## Contexto

El catálogo ya incluye `archmaker.pkg.onlyoffice-bin`. Los perfiles nuevos pedirán Zen como
default, pero la migración no debe reescribir elecciones históricas.

## Decisión

- Identidad canónica ONLYOFFICE: `archmaker.pkg.onlyoffice-bin` (sin inventar paquetes).
- `browserPolicy`: `selectedBrowserIds` (1..n, únicos) + `defaultBrowserId` ∈ selección.
- La migración v1→v2 **no** sustituye Firefox→Zen ni LibreOffice→ONLYOFFICE.

## Consecuencias

- Defaults de perfil nuevo (Zen, etc.) pertenecen a fases de catálogo/UI posteriores.
- El contrato v2 ya valida la cardinalidad cuando `browserPolicy` está presente.

## Verificación

- Tests `BrowserPolicy::validate` y corpus draft v2.
