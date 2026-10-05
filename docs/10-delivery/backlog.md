# Backlog inicial ordenado

| Orden | Issue | Depende de | Gate |
|---:|---|---|---|
| 1 | Resolver DEC-001..008 | — | Todos |
| 2 | Completar source/evidence register | 1 | G0/G7 |
| 3 | Aceptar scope y requirements | 1 | G1/G2 |
| 4 | Aceptar ADR-0001/0002 y crear ADR restantes | 1,3 | G3 |
| 5 | Cerrar modelo de dominio e IDs | 3,4 | G4 |
| 6 | Crear schemas + fixtures | 5 | G4 |
| 7 | Especificar AST y reglas | 5,6 | G7 |
| 8 | Cerrar CorePort/errors/events | 5,7 | G5 |
| 9 | Cerrar UX/component contracts/tokens | 3,8 | G6 |
| 10 | Cerrar threat/privilege model | 4,8 | G8 |
| 11 | Definir test/CI/release/packaging | 6–10 | G9 |
| 12 | Materializar issues y dependencies | 3–11 | G10 |
| 13 | Revisión cross-functional | Todos | PD-8 |
| 14 | Tag `planning-v1` | 13 | Go |
