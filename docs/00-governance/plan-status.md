# Estado de planificación

- Estado: in-review
- Gate global: No-Go
- Baseline: planning-v1
- Fecha de corte: 2026-10-05

## Bloqueos P0

Sin bloqueos P0 activos. DEC-001..DEC-008 quedaron **resueltas y aceptadas** el 2026-10-05 (ver `docs/00-governance/decision-register.md`).

Consecuencias que pasan a ser obligaciones de MVP (no bloqueos de planificación):

- DEC-007 (Tauri updater en MVP): `THR-UPD-001` debe cerrarse antes de MVP-0; el canal de actualización debe estar firmado (arrastra DEC-006) y ser fail-open offline (`NFR-OFF-001`).
- DEC-008 (solo Arch x86_64): aarch64 sale del alcance; matriz de soporte y pruebas reducidas a x86_64.

## Decisiones aún abiertas (P1, no bloqueantes)

- DEC-009: fuentes sin CDN runtime.
- DEC-010: telemetría (ninguna en MVP).

## Regla de avance

Solo se autoriza MVP-0 cuando G0–G10 tengan evidencia, no existan riesgos P0 sin tratamiento y todos los contratos P0 posean fixtures válidos e inválidos.

## Referencias

- Resultado motivado: `docs/00-governance/go-no-go.md` (DOC-GOV-GNG-001) → **No-Go: planificación en curso**.
- Cobertura de gates: `docs/10-delivery/gates.md`.
- Próximos pasos ordenados: `docs/10-delivery/next-issues.md` (ISSUE-001..010).
- Índice documental: `docs/index.md`.
