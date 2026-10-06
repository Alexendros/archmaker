---
id: DOC-SEC-SUPPLY-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: passed
evidence:
  - .github/workflows/release.yml (build + SBOM + sign + verify + provenance)
  - trusted_root.json (procedimiento de materialización documentado)
  - packaging-release.md (apéndice I11)
  - DEC-006 / DEC-007 / ADR-0008 / ADR-0009
releaseStatus: ineligible
owners:
  - security
reviewers:
  - independent-reviewer
---

# Política de supply-chain

Fase R9 (issue `AUD-018`). Define los controles de cadena de suministro para Rust, JS y los
artefactos de release. Amenazas asociadas: `THR-SUP-001` (dependencia comprometida) y
`THR-CAT-001` (catálogo manipulado). Riesgo: `RSK-007`. Decisiones citadas: `DEC-006` (firma) y
`DEC-007` (updater); su estado se lee en `docs/00-governance/decision-register.md`.

## Principios

1. **Procedencia verificable**: todo artefacto declara origen y digest; lo no verificado no se usa.
2. **Reproducibilidad**: builds reproducibles y promoción sin recompilar.
3. **Mínima dependencia**: se prefiere la biblioteca estándar; toda dependencia nueva exige
   justificación (FR/NFR) y ADR cuando proceda.
4. **Transparencia**: firma keyless con registro público (`Fulcio`/`Rekor`) y verificación offline
   con raíces fijadas.
5. **Bloqueo por defecto**: una dependencia o artefacto sin procedencia válida no entra en build ni
   en runtime.

## Controles por capa

| Capa              | Control                                                                                         | Evidencia                                                      |
| ----------------- | ----------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| Dependencias Rust | `Cargo.lock` versionado; `cargo audit` y `cargo deny`; sin features innecesarias.               | Lockfile en repo; run de CI.                                   |
| Dependencias JS   | Lockfile (`package-lock.json`/`pnpm-lock.yaml`); auditoría de dependencias; sin CDN en runtime. | Lockfile; `DEC-009`; CSP `self`.                               |
| CI                | Actions fijadas por SHA; permisos mínimos; sin secretos de larga duración donde OIDC basta.     | Workflows en `.github/workflows/`; `docs/09-quality/ci-cd.md`. |
| Build             | Build reproducible y determinista; artefactos con digest.                                       | Job de CI; `packaging-release.md`.                             |
| Release           | Firma Sigstore keyless ligada a identidad OIDC del workflow; SBOM y provenance.                 | `DEC-006`; `docs/10-delivery/packaging-release.md`.            |
| Verificación      | `cosign verify` con raíces de Fulcio/Rekor fijadas (`trusted_root.json`); verificación offline. | `DEC-006`; `updater-threat-model.md`.                          |
| Catálogo          | Digest y procedencia verificable como requisito de carga.                                       | `ADR-0008`; `docs/03-data/lifecycles.md`.                      |
| Actualización     | Canal firmado, endpoint en allowlist, versión mínima.                                           | `ADR-0009`; `updater-threat-model.md`.                         |

## Reglas

1. **Lockfiles obligatorios** y actualizados; una dependencia sin lock no se acepta.
2. **Auditoría automatizada** de vulnerabilidades y licencias en CI; hallazgos P0 bloquean el cierre.
3. **Actions por SHA**: ninguna action se referencia por etiqueta mutable.
4. **SBOM y provenance** por release; sin SBOM no hay release elegible.
5. **Firma obligatoria**: un artefacto de release o de actualización sin firma válida no se aplica.
6. **Sin CDN en runtime**; recursos empaquetados o del sistema (`DEC-009`).
7. **Cuarentena**: una dependencia comprometida se retira del build y se sustituye o fija en una
   versión segura; el incidente se trata según `vulnerability-response.md`.
8. **Verificación offline**: la comprobación de procedencia de releases no depende de red.

## Pruebas negativas (diseño)

| ID          | Given                                    | When                                          | Then                                |
| ----------- | ---------------------------------------- | --------------------------------------------- | ----------------------------------- |
| TST-SUP-001 | Artefacto sin firma o con firma inválida | Se intenta aplicar                            | Rechazado.                          |
| TST-SUP-002 | Action referenciada por etiqueta mutable | Se audita CI                                  | Falla la auditoría de supply-chain. |
| TST-SUP-003 | Catálogo sin procedencia válida          | Se intenta cargar                             | No se carga (`ADR-0008`).           |
| TST-SUP-004 | Dependencia con CVE P0 conocida          | Se ejecuta la auditoría                       | El gate falla y bloquea.            |
| TST-SUP-005 | Verificación de release sin red          | Se ejecuta `cosign verify` con raíces fijadas | Resuelve offline.                   |

## Trazabilidad

| Referencia                    | Relación                                                        |
| ----------------------------- | --------------------------------------------------------------- |
| `THR-SUP-001` / `THR-CAT-001` | Amenazas de dependencia y catálogo.                             |
| `RSK-007`                     | Riesgo de supply-chain; residuo aceptado en `DOC-SEC-RISK-001`. |
| `DEC-006` / `DEC-009`         | Firma keyless y sin CDN en runtime.                             |
| `ADR-0008` / `ADR-0009`       | Catálogos firmados y actualización firmada.                     |
| `G9`                          | Gate de calidad; exige CI verde y evidencia.                    |

## Límites declarados

- **Política, no pipeline**: no implementa el build, la atestación ni las firmas reales.
- La conformidad con la fuente institucional `SRC-008` queda pendiente de captura primaria.
- No modifica `reference/**` ni los contratos de `ADR-0007`/R5.
