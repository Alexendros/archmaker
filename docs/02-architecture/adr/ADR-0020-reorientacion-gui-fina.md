---
id: ADR-0020
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: passed
releaseStatus: ineligible
evidence:
  - docs/10-delivery/iso05-scope.md (suite completa en verde, 2026-10-25)
owners:
  - product
  - architecture
reviewers:
  - independent-reviewer
---

# ADR-0020: Reorientación — GUI fina sobre archiso/archinstall

- Estado: accepted
- Fecha: 2026-10-25
- Propietario: Producto y Arquitectura
- Aprobación: operador, 2026-10-25 (reorientación)
- Requisitos: FR-EXPORT-001, NFR-SEC-001, NFR-DET-001
- Verificación: ISO-05, 2026-10-25 ([`../../10-delivery/iso05-scope.md`](../../10-delivery/iso05-scope.md)); gates en `main` con la suite completa en verde.

## Contexto

Forge creció hacia un runner de instalación, planificación de discos, aarch64 y contratos Enterprise sin producir artefactos que las herramientas oficiales consuman. La auditoría del 25/10 mostró que el export `archinstall-profile` era un envelope propio y que el perfil vivo no permitía construir una ISO arrancable. El operador decidió reorientar el producto sin reescribir historia: el estado anterior queda congelado en la etiqueta `pre-reorientacion`.

## Decisión

1. El objetivo es una GUI fina sin root que valide la selección con explicaciones, genere un perfil archiso completo y un `user_configuration.json` real de archinstall, y construya la ISO con `mkarchiso` siempre dentro de un contenedor Docker.
2. El disco lo instala archinstall en la sesión viva. Forge no particiona, no formatea y no escribe discos.
3. Quedan fuera de alcance el runner de instalación, la elevación desde el WebView, aarch64, Enterprise, las actualizaciones firmadas, UKI/ESP, A6, el contrato de aplicación y los digest v2.
4. Se conservan la GUI Tauri/WASM, la validación con explicación (7 reglas), el manifiesto canónico con sha256, el catálogo con procedencia y la sonda de hardware de solo lectura.
5. Las herramientas oficiales son la autoridad del destino: archiso para el perfil y la ISO, archinstall para la configuración. Rust no duplica sus reglas ni inventa paquetes.
6. Los ADR 0006, 0009, 0011, 0013, 0014, 0015, 0016, 0018 y 0019 quedan superseded por este ADR. ADR-0004 queda superseded por ADR-0021. De ADR-0008 se conserva la procedencia de catálogo; la parte de firma queda superseded.
7. La documentación viva se reduce a lo necesario para operar y verificar el producto; el resto permanece en la etiqueta `pre-reorientacion`.

## Consecuencias

- El producto entrega una ISO arrancable y una configuración consumible por archinstall, con evidencia en CI mediante QEMU/OVMF.
- Menos superficie: sin runner, discos ni contratos Enterprise; menos documentos y herramientas que mantener.
- El coste de archiso/archinstall se concentra en la imagen Docker fijada por digest y en el contrato validado en CI.
- Riesgo residual: deriva de versión de archiso/archinstall; se mitiga fijando el digest y validando con `archinstall --dry-run`.
