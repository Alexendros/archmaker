---
id: DOC-GOV-G0-SRC-002
phase: planning
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: passed
releaseStatus: ineligible
owners:
  - governance
reviewers:
  - independent-reviewer
evidence:
  - "sha256sum --check reference/v5.1/SHA256SUMS (11/11 OK)"
  - "cmp reference/v5.1/forge_v4_final.html reference/v5.1/aliases/neubat_forge_v4_final.html"
  - "validación JSON de ALIASES.json, UNAVAILABLE-SOURCES.json y PROVENANCE.json"
---

# Disposición G0 de fuentes v5.1

ID `DOC-GOV-G0-SRC-002` · Estado `accepted` · Propietario Gobierno · Fecha 2026-10-06.

## Decisión

G0 mide si cada fuente declarada tiene identidad, procedencia, integridad, autoridad y una disposición verificable. No exige inventar ni reconstruir como original una fuente que nunca se recibió.

## Clasificación de SRC-001

`SRC-001` debe cambiar de `received-partial` a `controlled-incomplete`.

- Once originales recibidos permanecen inmutables y cubiertos por `SHA256SUMS` y `PROVENANCE.json`.
- Dos fuentes siguen `unavailable-declared-only`.
- Un nombre HTML se trata como `probable-rename-not-proven` y dispone de alias byte a byte.
- La colección histórica continúa sin autoridad normativa sobre datos actuales de Arch Linux.

## Condiciones de cierre

G0 puede permanecer `complete` si se cumplen todas:

- `sha256sum --check reference/v5.1/SHA256SUMS` finaliza correctamente.
- El alias HTML coincide byte a byte con `forge_v4_final.html`.
- `ALIASES.json` y `UNAVAILABLE-SOURCES.json` validan como JSON.
- Ningún original ausente figura como `received`.
- Ningún contrato o requisito P0 depende de contenido exclusivo no recuperado.
- El registro de fuentes enlaza esta disposición.
- CI comprueba que originales, alias y derivados no se mezclan.

G0 debe reabrirse si aparece una dependencia P0 sobre el YAML o el catálogo i18n histórico, o si se recibe un blob candidato al HTML original con hash distinto.

## Validaciones

```bash
sha256sum --check reference/v5.1/SHA256SUMS
cmp --silent \
  reference/v5.1/forge_v4_final.html \
  reference/v5.1/aliases/neubat_forge_v4_final.html
python -m json.tool reference/v5.1/ALIASES.json >/dev/null
python -m json.tool reference/v5.1/UNAVAILABLE-SOURCES.json >/dev/null
```
