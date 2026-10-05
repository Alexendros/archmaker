# ADR-0009: Actualización firmada en el MVP (Tauri updater)

- Estado: proposed
- Fecha: 2026-10-05
- Propietario: Release
- Requisitos: NFR-SEC-001, NFR-OFF-001, NFR-OBS-001

## Contexto

DEC-007 (aceptada el 2026-10-05) introduce el **updater de Tauri ya en el MVP**, en desviación de la recomendación inicial («manual/repo en MVP, updater firmado tras threat model»). Esto convierte a la actualización de la aplicación en una superficie de red del MVP y hace que `THR-UPD-001` (actualización comprometida) y `RSK-007` (supply chain) dejen de ser diferibles. Además, arrastra la firma (`DEC-006`, Sigstore para releases + firma offline evaluada) al alcance del MVP y debe convivir con `NFR-OFF-001` (operación offline).

Documentos de apoyo: `docs/08-security/threat-model.md` (`THR-UPD-001`), `docs/08-security/tauri-policy.md`, `docs/10-delivery/packaging-release.md`, `docs/00-governance/decision-register.md` (DEC-006, DEC-007).

## Drivers

- `DEC-007`: updater de Tauri en el MVP.
- `THR-UPD-001`: impedir una actualización comprometida.
- `RSK-007`: proteger la cadena de suministro.
- `NFR-OFF-001`: la aplicación debe seguir siendo plenamente usable sin red.
- `DEC-006`: firma de artefactos y releases.

## Opciones consideradas

- **Manual/repo en MVP, updater tras threat model** (recomendación original): menor superficie, pero descartada por decisión del propietario.
- **Tauri updater firmado en el MVP** (elegida): actualización automática, exige canal firmado y cierre previo de `THR-UPD-001`.
- **Repos de distro**: fuera de alcance para el MVP.

## Decisión propuesta

1. El MVP usa el **updater de Tauri** con **canal firmado**: cada artefacto de actualización se verifica con firma antes de aplicarse (alineado con DEC-006).
2. El updater es **fail-open offline**: si no hay red, la aplicación arranca y funciona con normalidad; la comprobación de actualizaciones solo ocurre cuando hay conectividad y **nunca bloquea** el uso (`NFR-OFF-001`).
3. Endpoint de actualización en **allowlist**, transporte TLS y verificación de firma obligatoria; sin ejecución de contenido no firmado.
4. **Versionado y rollback**: se documenta la política de versión mínima, compatibilidad y reversión en `docs/10-delivery/packaging-release.md`.
5. **`THR-UPD-001` debe cerrarse antes de MVP-0**; hasta entonces el updater permanece deshabilitado por defecto en builds de desarrollo.
6. La firma de releases sigue DEC-006 (Sigstore para releases; firma offline evaluada).

## Consecuencias

- El MVP incorpora una superficie de red adicional que debe probarse (firma inválida, downgrade, offline, endpoint no permitido).
- La promoción entre canales nightly/beta/stable no recompila (mismos artefactos firmados).
- El proceso de release incorpora firma y verificación como pasos obligatorios.

## Riesgos

- `THR-UPD-001` / `RSK-007`: mitigados por firma obligatoria, allowlist de endpoint y verificación previa a la aplicación.
- Riesgo de degradación offline: mitigado por diseño fail-open.
- Riesgo de downgrade/rollback mal gestionado: mitigado por política de versión mínima.

## Verificación

- Prueba negativa: un artefacto con firma inválida o manipulado se rechaza y no se aplica.
- Prueba negativa: sin red, la aplicación arranca y opera con normalidad (fail-open).
- Prueba negativa: endpoint fuera de allowlist no se contacta.
- `THR-UPD-001` cerrado con evidencia antes de MVP-0.

## Sustituye

—

## Sustituido por

—

## Requisitos relacionados

NFR-SEC-001, NFR-OFF-001, NFR-OBS-001. Implementa DEC-007 y depende de DEC-006. Relaciona `THR-UPD-001`, `RSK-007`, `docs/10-delivery/packaging-release.md`.
