---
id: DOC-OPS-MAN-USAGE
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
---

# Manual de uso

Texto canónico: [`docs/11-operations/manual/Manual-de-uso.md`](https://github.com/Soluciones-Alexendros/archmaker/blob/main/docs/11-operations/manual/Manual-de-uso.md).

Flujos cubiertos: JNY-001 (crear → exportar) y JNY-002 (importar). Rutas según `docs/05-ux/interaction-matrix.md`.

## 0. Tres capas (no confundir)

| Capa                 | Qué hace Forge                                              | Qué no hace                                     |
| -------------------- | ----------------------------------------------------------- | ----------------------------------------------- |
| Instalar la GUI      | Abrir AppImage/Tauri o web; configurar perfil               | Instalar Arch en un disco                       |
| Detectar este equipo | En desktop Linux, sondeo **solo lectura** (firmware/discos) | Formatear, cifrar, montar ni elevar privilegios |
| Exportar un perfil   | Manifest/artefacto con plan declarativo                     | Ejecutar el plan (A6 fuera de alcance)          |

En web no hay inspección local fingida: formulario manual o importación de informe.

## 1. Inicio (`/`)

Abre la aplicación (escritorio Tauri o adaptador web). En inicio puedes:

- ver drafts recientes
- crear un draft nuevo
- importar un documento local

La app opera sin CDN ni servidor de contenidos (NFR-OFF-001).

## 2. Nuevo draft (`/new`)

1. Elige el objetivo (target) disponible.
2. Opcionalmente aplica un preset como patch.
3. Confirma: se crea un draft versionado (UC-001 / FR-DRAFT-001).

El catálogo debe estar cargado y con digest de procedencia válido (FR-CAT-001). Si falta, el flujo se bloquea con un diagnóstico.

## 3. Configurar (`/configure/:step`)

1. Recorre los pasos del rail de configuración.
2. Ajusta opciones; la resolución marca valores manuales, derivados, bloqueados o en conflicto (FR-RESOLVE-001).
3. Corrige conflictos señalados antes de revisar.

No se ejecutan comandos del host. Solo se muta el draft en memoria/disco local de la app.

## 4. Revisar (`/review`)

1. Ejecuta la validación del pipeline (FR-VALIDATE-001).
2. Lee diagnósticos tipados; los bloqueantes impiden construir el manifest.
3. Cuando no hay bloqueos, construye el manifest canónico (FR-MANIFEST-001).

## 5. Exportar (`/export`)

1. Elige un target habilitado.
2. Exporta artefacto y reporte (FR-EXPORT-001).
3. Conserva digests y versiones del resultado.

Si el target no está habilitado, la exportación se rechaza; el manifest sigue válido.

## 6. Importar (`/import`)

1. Selecciona un documento local heredado (p. ej. formato v5.1).
2. La app detecta formato y versión, simula la migración y muestra el informe (UC-002).
3. Confirma para obtener un draft nuevo. El archivo original no se modifica.

## Atajos y accesibilidad

El flujo MVP está pensado para teclado y puntero (NFR-ACC-001 / WCAG 2.2 AA). Si un control no recibe foco visible o el contraste falla, es un defecto: ábrelo como bug con el plan `security-hygiene` o el hito MVP vigente.
