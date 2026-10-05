# Casos de uso

| ID | Actor | Precondición | Flujo principal | Postcondición | Fase |
|---|---|---|---|---|---|
| UC-001 | PER-001/2 | Catálogo cargado | Crear draft y seleccionar objetivo | Draft versionado | MVP |
| UC-002 | PER-001/2 | Documento local | Detectar, simular migración, confirmar | Nuevo draft + informe | MVP |
| UC-003 | PER-001/2 | Draft | Resolver capacidades y diagnosticar | Resolution reproducible | MVP |
| UC-004 | PER-001/2 | Resolution | Revisar manual/derived/locked/conflict | Decisiones explicadas | MVP |
| UC-005 | PER-001/2 | Sin bloqueos | Construir manifest y exportar | Artifact + digest | MVP |
| UC-006 | PER-001/2 | Runner compatible | Preflight, plan, dry-run, confirmar, ejecutar | Journal + resultado | v1 |
| UC-007 | PER-004 | Org y policy | Aprobar perfil y campaña | Despliegue auditado | Enterprise |
