# Política de seguridad

ArchMaker está en fase de planificación (`planning-v1`). **Todavía no hay binarios de producto publicados**; el repositorio contiene documentación, contratos y validaciones.

## Versiones soportadas

| Versión | Soporte |
|---|---|
| `planning-v1` (documental) | Sí — fase actual |
| MVP / v1 / Enterprise | Aún no publicadas |

## Cómo reportar una vulnerabilidad

Usa **GitHub Private Vulnerability Reporting** (pestaña *Security* → *Report a vulnerability*). **No** abras issues públicos con detalles explotables.

Incluye, si es posible:

- Descripción del problema y su impacto.
- Pasos de reproducción o prueba de concepto.
- Componente afectado (documento, contrato, workflow, `reference/`, dependencia).
- Versión/commit y entorno.

## Tiempos objetivo

- Acuse de recibo: 3 días hábiles.
- Triage inicial: 7 días hábiles.
- Plan de corrección y fecha de divulgación coordinada: según severidad, tras el triage.

## Divulgación

Embargo hasta que exista corrección o mitigación y una fecha de divulgación acordada. Se dará crédito a quien reporte salvo petición en contra.

## Alcance

- **En alcance**: documentación, contratos (`contracts/`), workflows de CI, scripts de validación, `reference/` (evidencia inmutable) y dependencias.
- **Fuera de alcance**: entornos de terceros, ingeniería social y denegación de servicio no específica del proyecto.

## Contacto

Canal principal: GitHub Private Vulnerability Reporting. Alternativa: el propietario del repositorio (`CODEOWNERS`, `@Alexendros`).
