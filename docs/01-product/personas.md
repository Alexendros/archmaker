# Personas

## PER-001 — Usuario guiado

- Objetivo: obtener una configuración segura sin aprender comandos.
- Necesita: defaults explicados, progresión clara, prevención y recuperación.
- Riesgo: aceptar cambios automáticos sin comprenderlos.
- Éxito: completa y exporta un manifest válido solo con teclado o puntero.

## PER-002 — Usuario avanzado

- Objetivo: controlar cada selección y comparar resultados.
- Necesita: diff, perfiles, overrides seguros, metadatos y reproducibilidad.
- Riesgo: que la simplificación oculte restricciones.
- Éxito: entiende cada selección derivada y su fuente.

## PER-003 — Maintainer de catálogo

- Objetivo: actualizar opciones sin introducir incompatibilidades.
- Necesita: schema, lint, evidencia, firma, fixtures y release workflow.
- Riesgo: datos obsoletos o catálogos comprometidos.

## PER-004 — Operador Enterprise

- Objetivo: gobernar perfiles, políticas y campañas.
- Necesita: RBAC, aprobaciones, auditoría, rollout y observabilidad.
- Riesgo: cruce de tenant o ejecución no autorizada.
