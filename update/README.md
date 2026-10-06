# Actualización G0 de referencias ArchMaker

Paquete incremental para corregir el tratamiento de tres fuentes declaradas pero no recibidas.

## Contenido

- Alias HTML byte a byte del único artefacto NEUBAT recibido.
- Manifest de alias con identidad prudente.
- Disposición de dos fuentes no recuperadas.
- Decisión G0 aplicable al registro y a los gates.
- Prompt de integración para el agente activo.

## Integración

Copiar el contenido sobre la raíz del repositorio sin sobrescribir los originales de `reference/v5.1/`. El único HTML añadido vive en `reference/v5.1/aliases/` y no sustituye `forge_v4_final.html`.

```bash
rsync -av --ignore-existing archmaker-g0-reference-update/ /ruta/absoluta/ArchMaker/
cd /ruta/absoluta/ArchMaker
sha256sum --check reference/v5.1/SHA256SUMS
cmp --silent reference/v5.1/forge_v4_final.html reference/v5.1/aliases/neubat_forge_v4_final.html
python -m json.tool reference/v5.1/ALIASES.json >/dev/null
python -m json.tool reference/v5.1/UNAVAILABLE-SOURCES.json >/dev/null
```

Después deben actualizarse de forma revisada `PROVENANCE.json`, `docs/00-governance/source-register.md`, `docs/10-delivery/gates.md` y cualquier afirmación que describa SRC-001 como íntegro o completamente recibido.
