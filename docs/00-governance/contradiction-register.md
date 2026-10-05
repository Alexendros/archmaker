# Registro de contradicciones v5.1

| ID | Evidencia | Contradicción | Impacto | Resolución requerida |
|---|---|---|---|---|
| CON-001 | Schema original vs instancia | `mode: single` no existe en schema original. | Migración | Mapear a cardinalidad explícita. |
| CON-002 | Reglas vs ambos schemas | Se usa `count_gt`, operador no definido. | Resolver | Especificar AST y tipos antes de implementarlo. |
| CON-003 | Schema original vs presets | Tres selecciones usan arrays donde el schema solo admite escalares. | Datos | Definir `SelectionValue` discriminado. |
| CON-004 | Instancia | `base-devel` aparece dos veces. | Identidad | Resolver DEC-002 y crear detector de duplicados. |
| CON-005 | Schema v5.1 | Existe `cmd`. | Seguridad | Rechazar ejecución; migrar como evidencia no ejecutable. |
| CON-006 | Schema original | Existen hooks `pre/post` string. | Seguridad | No migrarlos a contratos runtime. |
| CON-007 | README | Declara `archmaker-v10.4-p3.yaml` e `i18n.es.json`, no recibidos. | Procedencia | Recuperar o declarar definitivamente ausentes. |
| CON-008 | README | Declara `neubat_forge_v4_final.html`; se recibió `forge_v4_final.html`. | Procedencia | No asumir equivalencia. |
| CON-009 | HTML/TSX | Catálogo, reglas, comandos, estado y UI están embebidos. | Arquitectura | Separar por puertos y módulos. |
| CON-010 | Prototipos | React, Babel, Tailwind y fuentes dependen de CDN. | Offline/CSP | Empaquetar recursos; CSP `self` por defecto. |
| CON-011 | Datos | Afirmaciones de versiones, tamaños y compatibilidad no citan fuentes. | Exactitud | Crear registro de evidencia académica/oficial. |
| CON-012 | Build heredado | Pipeline incluye `pacstrap` en configuración de UI. | Privilegios | Convertir a target/plan tipado de v1. |
