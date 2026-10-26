# Política de seguridad

ArchMaker está en desarrollo activo. **v0.3.0 es la última versión publicada**; los binarios de producto (AppImage Forge Linux x86_64) se publican mediante `publish.yml` con firmas cosign keyless y provenance.

## Versiones soportadas

| Versión                                   | Soporte                                                  |
| ----------------------------------------- | -------------------------------------------------------- |
| `v0.3.0`                                  | Última release pública — AppImage Forge x86_64           |
| `main`                                    | Rama de desarrollo; sujeta a los gates de CI (`ci-gate`) |
| Versiones anteriores (`v0.1.0`, `v0.2.0`) | Sin release ni AppImage publicados; no soportadas        |

## Cómo reportar una vulnerabilidad

Usa **GitHub Private Vulnerability Reporting** (pestaña _Security_ → _Report a vulnerability_). **No** abras issues públicos con detalles explotables.

Incluye, si es posible:

- Descripción del problema y su impacto.
- Pasos de reproducción o prueba de concepto.
- Componente afectado (documento, contrato, workflow, `reference/`, dependencia, AppImage).
- Versión/commit y entorno.

## Tiempos objetivo

- Acuse de recibo: 3 días hábiles.
- Triage inicial: 7 días hábiles.
- Plan de corrección y fecha de divulgación coordinada: según severidad, tras el triage.

## Divulgación

Embargo hasta que exista corrección o mitigación y una fecha de divulgación acordada. Se dará crédito a quien reporte salvo petición en contra.

## Alcance

- **En alcance**: documentación, contratos (`contracts/`), workflows de CI, scripts de validación, `reference/` (evidencia inmutable), dependencias y artefactos publicados (AppImage, SBOM, firmas, provenance).
- **Fuera de alcance**: entornos de terceros, ingeniería social y denegación de servicio no específica del proyecto.

## Cadena de suministro

- Los workflows usan acciones pinnadas por SHA.
- `publish.yml` firma el AppImage, el SBOM y los checksums con cosign keyless (identidad `https://github.com/Soluciones-Alexendros/archmaker`).
- `actions/attest-build-provenance` genera provenance para los artefactos publicados.
- `gh attestation verify` valida la provenance en el job `verify-release`.
- SCA: cargo audit, pnpm audit high, OSV-Scanner y gitleaks forman parte de `ci-gate`.

## Contacto

Canal principal: GitHub Private Vulnerability Reporting. Alternativa: el propietario del repositorio (`CODEOWNERS`, `@Alexendros`).
