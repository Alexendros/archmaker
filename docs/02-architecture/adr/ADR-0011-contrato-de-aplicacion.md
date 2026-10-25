---
id: ADR-0011
phase: planning
priority: P0
documentStatus: superseded
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0003
    expectedDocumentStatus: accepted
  - id: ADR-0005
    expectedDocumentStatus: accepted
---

# ADR-0011: Contrato de aplicación desde el configurador

- Estado: superseded
- Fecha: 2026-10-14
- Propietario: Arquitectura
- Requisitos: FR-EXPORT-001, FR-VALIDATE-001, FR-MANIFEST-001, NFR-DET-001, NFR-OFF-001, NFR-MARK-001
- Superseded by: ADR-0020

## Contexto

Una especificación de ISO elegida en el configurador debe materializarse como un resultado
comprobable: el programa cumple un contrato y lo verifica. El seguimiento de cada cláusula no
puede vivir solo en Markdown: hace falta un esquema JSON Schema (ADR-0005), un issue de GitHub
con formulario y tests estrictos. ADR-0003 mantiene el manifiesto canónico separado del target.
DEC-001 fija hoy `archmaker-profile`, `archinstall-profile` y `report`; el formato de imagen
sigue fuera hasta un alcance y un registro de decisiones que lo admitan.

## Drivers

- FR-VALIDATE-001 / FR-MANIFEST-001: no se materializa un artefacto con errores bloqueantes.
- FR-EXPORT-001 / NFR-DET-001: cada formato es una proyección del mismo manifiesto.
- NFR-OFF-001: cumplir y verificar no exigen red ni shell en el puesto de quien configura.
- NFR-MARK-001 / ADR-0010: el artefacto es un derivado de Arch Linux, no Arch Linux oficial.
- ADR-0005: la norma de cada cláusula es un schema con `additionalProperties: false`.

## Opciones

1. Documentar solo en Markdown los objetivos de aplicación y dejar el cumplimiento a la
   revisión humana.
2. Generar un contrato JSON desde la selección resuelta, con una cláusula tipada por
   obligación, verificada en Rust y seguida en issues de GitHub.
3. Ampliar DEC-001 con un target de imagen ahora y ligar el contrato a `mkarchiso`.

## Decisión

Se adopta la opción 2 como propuesta. La selección resuelta, la identidad de la base Arch
Linux x86_64 y el formato admitido producen un contrato. El programa verifica cada cláusula
exigible y rechaza el conjunto si una falla. Cada cláusula tiene:

1. Un schema Draft 2020-12 bajo `contracts/json-schema/` (norma).
2. Un issue de GitHub con etiqueta `kind:clause` (seguimiento).
3. Tests estrictos de cumplimiento escritos por el agente de pruebas y evaluados por un
   verificador Rust (autoridad semántica, ADR-0001).

No se añade un target de imagen. Pedir la cláusula de arranque como cumplida antes de que el
formato de imagen esté admitido es un incumplimiento, no un target nuevo. ADR-0003 y ADR-0005
siguen vigentes. OBJ-001..006 no se enmiendan.

## Consecuencias

- Aparecen `application-contract.schema.json` y siete schemas de cláusula con corpus
  válido/inválido.
- El job `json-schema` meta-valida esos contratos.
- Los issues `CLAUSE-*` rastrean el estado `pending` / `red` / `verified`.
- El verificador Rust no llama a `pacstrap`, `mkarchiso`, root ni shell.

## Riesgos

- Ampliar DEC-001 en silencio: se mitiga rechazando formatos fuera del enum actual.
- Confundir el contrato con autorización de imagen: la cláusula de arranque permanece
  incumplible hasta un veredicto y un alcance `accepted`.

## Verificación

- Corpus JSON Schema en verde para el contrato y las siete cláusulas.
- Suite Rust de cumplimiento: caso positivo y negativo por cláusula.
- Issues de GitHub abiertos con id, ruta de schema y nombre de test.

## Sustituye

—

## Sustituido por

—
