# CI/CD

- Estado: draft
- Propietario: Calidad
- Última revisión: 2026-10-05
- Requisitos relacionados: NFR-DET-001, NFR-OBS-001, NFR-SEC-001, NFR-PORT-001
- Sustituye / sustituido por: —
- Sujeto a: DEC-005 (licencia), DEC-006 (firma), DEC-007 (actualización), DEC-009 (fuentes)

## Alcance

Pipeline de integración y entrega para `planning-v1`. En esta fase **solo se valida
documentación y contratos**; no se construye producto. La validación documental se define en
[documentation-validation.md](documentation-validation.md) y su ejecución vive en
`.github/workflows/`. Los artefactos de release se describen en
[packaging-release.md](../10-delivery/packaging-release.md). Todo el contenido es `draft`.

## Estrategia de gates

| Disparador | Workflows | Gates | Naturaleza |
|---|---|---|---|
| PR / push a `main` | `.github/workflows/docs-validate.yml` | G0, G4, G5, G7, G9 | Bloqueante |
| PR / push a `main` | `.github/workflows/json-schema.yml` | G4, G5 | Bloqueante |
| `nightly` | los anteriores + build/attest de artefactos | G8, G9, G10 | No bloqueante (señal) |
| `release` | promoción de artefactos ya construidos | G8, G9, G10 | Bloqueante |

- **PR**: verificación rápida y determinista. Los checks de documentación no requieren secretos.
- **Nightly**: re-ejecuta los gates sobre `main` y añade construcción, SBOM, provenance y firma
  para detectar deriva de la cadena de suministro. Ver gates en [gates.md](../10-delivery/gates.md).
- **Release**: promoción sin rebuild (ver más abajo); los artefactos firmados y atestados del
  nightly promovido son los que se publican.

PENDIENTE (G9): el workflow de build/attest no existe aún; se crea tras aceptar DEC-005 y DEC-006.

## Supply chain

### Acciones fijadas por SHA

Toda acción de terceros se referencia por **commit SHA** (no por tag flotante) para que el
contenido ejecutado sea inmutable. En los workflows actuales los SHA aún no están fijados: se
marcan como `@<PENDIENTE-SHA> # TODO no verificado — fijar por SHA`. No deben introducirse hashes
no verificados contra la fuente.

- Acciones oficiales (p. ej. `actions/checkout`, `actions/setup-python`, `actions/*-attest`).
- Acciones de terceros reputadas (p. ej. `DavidAnson/markdownlint-cli2-action`).
- Renovación de pins mediante Dependabot (`.github/dependabot.yml`, ecosistema `github-actions`).

### Least privilege

- `permissions: contents: read` por defecto en todos los workflows.
- Los permisos elevados (`id-token: write`, `attestations: write`, `packages: write`) se conceden
  **solo** en el job que atesta o publica, nunca a nivel de workflow.
- Sin secretos de larga duración si puede usarse OIDC federado.

### Concurrencia y timeouts

- `concurrency` por workflow y ref con `cancel-in-progress: true` para evitar ejecuciones solapadas.
- `timeout-minutes` en cada job para acotar coste y ejecuciones colgadas.

## Artefactos: attestation, SBOM y firma

PENDIENTE (DEC-005/DEC-006): la política de firma y licencia no está aceptada; lo siguiente es
propuesta `draft` sujeta a esas decisiones.

| Elemento | Mecanismo | Estándar | Estado |
|---|---|---|---|
| Provenance | GitHub Artifact Attestations (`actions/attest-build-provenance`) | SLSA v1.2 / in-toto | Propuesto |
| SBOM | Generación CycloneDX y/o SPDX; `actions/attest-sbom` | CycloneDX / SPDX | Propuesto |
| Firma de artefactos | Sigstore (keyless, ligada a OIDC) o firma offline | Sigstore / Minisign | PENDIENTE DEC-006 |
| Checksums | `SHA256SUMS` + verificación `sha256sum --check` | SHA-256 | Propuesto |

- **Provenance**: se atesta cada artefacto de build para ligar binario, commit y workflow. La
  fuente institucional (NIST SSDF/SLSA/Sigstore) está registrada como `SRC-008`, con captura
  pendiente; no se declara conformidad verificada hasta capturarla.
- **SBOM**: se publica CycloneDX y/o SPDX junto al release para inventario de dependencias.
- **Firma**: Sigstore para releases y firma offline evaluada; el detalle depende de DEC-006.
- Toda atestación requiere `id-token: write` y `attestations: write` en el job correspondiente.

## Promoción sin rebuild

Los canales `nightly`, `beta` y `stable` **promueven el mismo artefacto** (mismos digests), sin
recompilar. La promoción reutiliza el binario ya firmado y atestado; cambia la etiqueta/`channel`,
no el contenido. Cualquier rebuild invalida la atestación y exige re-firmar. Detalle de canales y
rollback en [packaging-release.md](../10-delivery/packaging-release.md).

## Pendientes y dependencias de decisión

- **DEC-002** (ámbito de IDs): semántica del chequeo de unicidad de IDs en `docs-validate.yml`.
- **DEC-005** (licencia): términos de artefactos y SBOM; bloquea la política de release.
- **DEC-006** (firma de catálogos): mecanismo de firma (Sigstore/Minisign/ambos).
- **DEC-007** (actualización): habilitación del updater de Tauri; ver `packaging-release.md`.
- **DEC-009** (fuentes): confirma la prohibición de CDN en docs de producción.
- **G9**: flujo de build/attest/SBOM aún no implementado (solo existe validación documental).

## Referencias

- Reglas de validación: [documentation-validation.md](documentation-validation.md).
- Gates G0–G10: [gates.md](../10-delivery/gates.md).
- Artefactos y canales: [packaging-release.md](../10-delivery/packaging-release.md).
- Decisiones abiertas: [decision-register.md](../00-governance/decision-register.md).
- Contratos JSON Schema: `contracts/json-schema/README.md`.
- Política Tauri (CSP sin CDN en producción): [tauri-policy.md](../08-security/tauri-policy.md).
