# Prompt de integración

Audita e integra el paquete `archmaker-g0-reference-update` en el repositorio ArchMaker ya iniciado.

Reglas obligatorias:

1. No reinicies la planificación y no reemplaces documentos más completos sin fusionarlos.
2. No modifiques ningún original ya cubierto por `reference/v5.1/SHA256SUMS`.
3. Trata `aliases/neubat_forge_v4_final.html` como copia byte a byte de `forge_v4_final.html`, no como original recuperado.
4. Mantén `archmaker-v10.4-p3.yaml` e `i18n.es.json` como `unavailable-declared-only`; no fabriques su contenido ni crees archivos falsos con esos nombres.
5. Cambia SRC-001 de `received-partial` a `controlled-incomplete` y enlaza `G0-SOURCE-DISPOSITION.md`.
6. Corrige cualquier afirmación «evidencia v5.1 íntegra» por «originales recibidos íntegros; colección declarada incompleta y controlada».
7. Mantén G0 en `complete` solo si ningún requisito P0 depende de semántica exclusiva de las fuentes ausentes y pasan todas las validaciones.
8. Añade CI para: hashes originales, igualdad del alias, JSON de disposición y prohibición de clasificar ausentes como recibidos.
9. Actualiza `PROVENANCE.json` mediante una nueva versión o campos compatibles, sin borrar la historia.
10. Entrega diff, comandos ejecutados, resultados y cualquier motivo para reabrir G0.
