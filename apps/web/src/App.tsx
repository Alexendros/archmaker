import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type {
  CoreError,
  CorePort,
  Diagnostic,
  Draft,
  Manifest,
} from "./coreport.js";
import {
  DESKTOP_CATALOG_REF,
  DRAFT_TARGET,
  exportTarget,
  type CatalogRef,
} from "./coreport.js";
import type { PortKind } from "./create-core-port.js";
import { CommandPalette } from "./CommandPalette.js";
import {
  analyzeImportFile,
  applyConsentedImport,
  summarizeReport,
  type ImportDraftPayload,
} from "./consented-import.js";
import { draftForCore, listFaceExclusions } from "./draft-for-core.js";
import {
  exportDraftCopy,
  loadLastDraftLocal,
  persistDraftLocal,
  retryPersistDraftLocal,
  subscribePersistStatus,
  type PersistResult,
  type PersistStatus,
} from "./draft-persistence.js";
import {
  DeliveryError,
  deliverArtifact,
  destinationTokenFor,
  exportFilename,
  exportStatusMessage,
} from "./export-delivery.js";
import { isPreparedCurrent } from "./export-generation.js";
import { PRODUCT_WINDOW_TITLE } from "./build-meta.js";
import { ForgeAside } from "./ForgeAside.js";
import { ForgeHeader } from "./ForgeHeader.js";
import { ForgeRail } from "./ForgeRail.js";
import { ForgeStepper } from "./ForgeStepper.js";
import { GlossaryDrawer } from "./GlossaryDrawer.js";
import { ImportReportDialog } from "./ImportReportDialog.js";
import { PackageDialog } from "./PackageDialog.js";
import {
  appIds,
  boolValue,
  chooseSingle,
  setBool,
  setText,
  singleId,
  textValue,
  toggleApp,
} from "./forge-helpers.js";
import {
  BOOTLOADERS,
  ENVIRONMENTS,
  FILESYSTEMS,
  INITIAL_APP_OPTION_IDS,
  INITIAL_SINGLE,
  KERNELS,
  RAIL_STEPS,
  STEP_IDS,
  allEnvironments,
  allLayers,
  findOptionById,
  isUnverifiedOption,
  type FacePackage,
} from "./forge-face.js";
import { allStepProgress } from "./step-progress.js";
import {
  INITIAL_STORAGE_INTENT,
  labelStorageMode,
  type StorageIntent,
  type StorageMode,
} from "./storage-intent.js";
import {
  intentFromStoragePlan,
  uiStateForIntent,
  withFaceV2Fields,
} from "./storage-plan-face.js";
import {
  catalogOptionTitle,
  parseCatalogView,
  type CatalogView,
} from "./catalog-view.js";

type Props = { port: CorePort; portKind: PortKind };

type PrepareOk = {
  ok: true;
  manifest: Manifest;
  diagnostics: Diagnostic[];
  generation: number;
};
type PrepareErr = { ok: false; reason: string };
type PrepareResult = PrepareOk | PrepareErr;

type ToastState = {
  message: string;
  undo?: () => void;
} | null;

type ImportPending =
  | {
      kind: "ready";
      sourceBytes: number[];
      generation: number;
      payload: ImportDraftPayload;
      needsBlockingConfirm: boolean;
    }
  | {
      kind: "needs_blocking_confirm";
      sourceBytes: number[];
      generation: number;
      error: CoreError;
    };

function persistStatusLabel(status: PersistStatus): string {
  switch (status) {
    case "saving":
      return "guardando";
    case "saved":
      return "guardado";
    case "error":
      return "error";
    default:
      return "";
  }
}

function announce(kind: "status" | "alert", message: string): void {
  const region = document.getElementById(
    kind === "status" ? "toast-region" : "alert-region",
  );
  if (!region) return;
  region.textContent = "";
  const span = document.createElement("span");
  span.className = kind === "status" ? "toast" : "toast toast-error";
  span.textContent = message;
  region.append(span);
}

function fail(err: { code: string; messageKey: string }, where: string): void {
  announce("alert", `${where}: ${err.code} (${err.messageKey})`);
}

function withInitialSelections(draft: Draft): Draft {
  if (draft.selections.length > 0) return draft;
  return {
    ...draft,
    selections: [
      {
        stepId: STEP_IDS.intent,
        value: { kind: "single", optionId: INITIAL_SINGLE.intent },
      },
      {
        stepId: STEP_IDS.kernel,
        value: { kind: "single", optionId: INITIAL_SINGLE.kernel },
      },
      {
        stepId: STEP_IDS.boot,
        value: { kind: "single", optionId: INITIAL_SINGLE.boot },
      },
      {
        stepId: STEP_IDS.filesystem,
        value: { kind: "single", optionId: INITIAL_SINGLE.filesystem },
      },
      {
        stepId: STEP_IDS.compositor,
        value: { kind: "single", optionId: INITIAL_SINGLE.compositor },
      },
      {
        stepId: STEP_IDS.apps,
        value: {
          kind: "multiple",
          optionIds: [...INITIAL_APP_OPTION_IDS],
        },
      },
    ],
  };
}

export function App({ port, portKind }: Props) {
  const [draft, setDraft] = useState<Draft | null>(null);
  const [catalogRef, setCatalogRef] = useState<CatalogRef>(DESKTOP_CATALOG_REF);
  const [catalogView, setCatalogView] = useState<CatalogView | null>(null);
  const [manifest, setManifest] = useState<Manifest | null>(null);
  const [diagnostics, setDiagnostics] = useState<Diagnostic[]>([]);
  const [validated, setValidated] = useState(false);
  const [busy, setBusy] = useState(false);
  const [step, setStep] = useState(0);
  const [visited, setVisited] = useState<Set<number>>(() => new Set([0]));
  const [invalid, setInvalid] = useState<Set<number>>(() => new Set());
  const [storage, setStorage] = useState<StorageIntent>(INITIAL_STORAGE_INTENT);
  const [dark, setDark] = useState(false);
  const [toast, setToast] = useState<ToastState>(null);
  const [pkgModal, setPkgModal] = useState<FacePackage | null>(null);
  const [cmdOpen, setCmdOpen] = useState(false);
  const [glossaryOpen, setGlossaryOpen] = useState(false);
  const [ready, setReady] = useState(false);
  const [persistStatus, setPersistStatus] = useState<PersistStatus>("idle");
  const [persistExportable, setPersistExportable] = useState<string | null>(
    null,
  );
  const [importPending, setImportPending] = useState<ImportPending | null>(
    null,
  );
  const [importBusy, setImportBusy] = useState(false);
  const generationRef = useRef(0);
  const preparedGenerationRef = useRef<number | null>(null);
  const draftRef = useRef<Draft | null>(null);
  draftRef.current = draft;

  useEffect(() => subscribePersistStatus(setPersistStatus), []);

  const handlePersistResult = useCallback((result: PersistResult) => {
    if (result.ok) {
      setPersistExportable(null);
      return;
    }
    setPersistExportable(result.exportable);
    announce("alert", result.message);
  }, []);

  const bootId =
    singleId(draft, STEP_IDS.boot) ?? "archmaker.boot.systemd-boot";
  const kernelId = singleId(draft, STEP_IDS.kernel) ?? "archmaker.kernel.linux";
  const fsId =
    singleId(draft, STEP_IDS.filesystem) ?? "archmaker.filesystem.btrfs";
  const envId =
    singleId(draft, STEP_IDS.compositor) ?? "archmaker.compositor.gnome";
  const selectedApps = useMemo(() => new Set(appIds(draft)), [draft]);
  const hostname = textValue(draft, STEP_IDS.hostname);
  const encrypt = boolValue(draft, STEP_IDS.encrypt);

  const kernelFace = KERNELS.find((k) => k.optionId === kernelId);
  const bootFace = BOOTLOADERS.find((b) => b.optionId === bootId);
  const filesystemFace = FILESYSTEMS.find((f) => f.optionId === fsId);
  const env = ENVIRONMENTS.find((e) => e.optionId === envId);
  const kernel = kernelFace
    ? {
        ...kernelFace,
        title: catalogOptionTitle(catalogView, kernelId) ?? kernelFace.title,
      }
    : undefined;
  const boot = bootFace
    ? {
        ...bootFace,
        title: catalogOptionTitle(catalogView, bootId) ?? bootFace.title,
      }
    : undefined;
  const filesystem = filesystemFace
    ? {
        ...filesystemFace,
        title: catalogOptionTitle(catalogView, fsId) ?? filesystemFace.title,
      }
    : undefined;
  const envs = allEnvironments();
  const layers = allLayers();
  const stepProgress = useMemo(
    () =>
      allStepProgress({
        current: step,
        visited,
        invalid,
        storage,
        hasDefaults: Boolean(draft),
        reviewReady: validated && Boolean(manifest),
      }),
    [step, visited, invalid, storage, draft, validated, manifest],
  );

  const selectedPackages = useMemo(() => {
    const pkgs: FacePackage[] = [];
    for (const id of selectedApps) {
      const hit = findOptionById(id);
      if (hit && "size" in hit) pkgs.push(hit);
    }
    return pkgs;
  }, [selectedApps]);

  /** Suma solo tamaños de catálogo de apps seleccionadas; sin base inventada. */
  const totalSize = useMemo(
    () => selectedPackages.reduce((acc, p) => acc + p.size, 0),
    [selectedPackages],
  );

  const sideDiagnostics = useMemo(() => {
    const list: {
      title: string;
      description: string;
      kind: "info" | "warn";
    }[] = [];
    list.push({
      kind: "info",
      title: `Kernel ${kernel?.title ?? "—"}`,
      description: kernel?.description ?? "Sin kernel seleccionado.",
    });
    return list;
  }, [kernel]);

  const showToast = useCallback((message: string, undo?: () => void) => {
    setToast({ message, undo });
    announce("status", message);
    window.setTimeout(() => setToast(null), 2800);
  }, []);

  const invalidateDerived = useCallback(() => {
    generationRef.current += 1;
    preparedGenerationRef.current = null;
    setManifest(null);
    setDiagnostics([]);
    setValidated(false);
  }, []);

  const applyDraft = useCallback(
    (next: Draft, nextStorage?: StorageIntent) => {
      const intent =
        nextStorage ??
        (next.storagePlan ? intentFromStoragePlan(next.storagePlan) : storage);
      const projected = withFaceV2Fields(next, intent);
      setDraft(projected);
      setStorage({ ...intent, uiState: uiStateForIntent(intent) });
      invalidateDerived();
      void persistDraftLocal(projected).then(handlePersistResult);
    },
    [handlePersistResult, invalidateDerived, storage],
  );

  useEffect(() => {
    let cancelled = false;
    void (async () => {
      const loaded = await port.loadCatalog({ source: "embedded" });
      if (cancelled) return;
      if (!loaded.ok) {
        fail(loaded.error, "Cargar catálogo");
        return;
      }
      const viewed = parseCatalogView(loaded.value);
      if (viewed) setCatalogView(viewed);
      const ref = DESKTOP_CATALOG_REF;
      setCatalogRef(ref);

      const local = await loadLastDraftLocal();
      if (cancelled) return;
      if (local.kind === "error") {
        announce(
          "alert",
          `${local.message} Se abrirá un perfil nuevo (recuperación fallida).`,
        );
        // Continúa a createDraft con causa visible (H14): no es silencio.
      } else if (local.kind === "found") {
        const opened = await port.openDraft({
          draftId: local.draft.id,
          seed: local.draft,
        });
        if (cancelled) return;
        if (opened.ok) {
          setDraft(opened.value);
          setStorage(intentFromStoragePlan(opened.value.storagePlan));
          setReady(true);
          announce("status", "Perfil recuperado.");
          return;
        }
        announce(
          "alert",
          `Perfil local incompatible: ${opened.error.code} (${opened.error.messageKey})`,
        );
      }

      const res = await port.createDraft({
        catalogRef: ref,
        targetRef: DRAFT_TARGET,
      });
      if (cancelled) return;
      if (!res.ok) {
        fail(res.error, "Crear draft");
        return;
      }
      let next = withFaceV2Fields(
        withInitialSelections(res.value),
        INITIAL_STORAGE_INTENT,
      );
      const saved = await port.saveDraft({
        draft: next,
        expectedRevision: next.revision,
      });
      if (cancelled) return;
      if (!saved.ok) {
        fail(saved.error, "Guardar selecciones iniciales");
        return;
      }
      next = saved.value;
      setDraft(next);
      setStorage(intentFromStoragePlan(next.storagePlan));
      void persistDraftLocal(next).then(handlePersistResult);
      setReady(true);
      announce("status", `Draft listo (${portKind}). Arranque: systemd-boot.`);
    })();
    return () => {
      cancelled = true;
    };
  }, [port, portKind]);

  useEffect(() => {
    document.documentElement.classList.toggle("dark", dark);
  }, [dark]);

  useEffect(() => {
    document.title = PRODUCT_WINDOW_TITLE;
  }, []);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setCmdOpen(true);
      }
      if (e.key === "Escape") {
        setCmdOpen(false);
        setPkgModal(null);
        setGlossaryOpen(false);
        setImportPending(null);
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  function requireDraft(): Draft | null {
    if (!draft) {
      announce("alert", "El draft aún no está listo.");
      return null;
    }
    return draft;
  }

  function pickSingle(stepId: string, optionId: string): void {
    const d = requireDraft();
    if (!d) return;
    const opt = findOptionById(optionId);
    applyDraft(chooseSingle(d, stepId, optionId));
    const title = opt && "title" in opt ? opt.title : optionId;
    if (isUnverifiedOption(opt)) {
      showToast(`${title} seleccionada (solicitud no verificada).`);
    } else {
      showToast(`${title} seleccionada.`);
    }
  }

  function onTogglePackage(pkg: FacePackage): void {
    const d = requireDraft();
    if (!d) return;
    const { draft: next, added } = toggleApp(d, pkg.optionId);
    applyDraft(next);
    showToast(added ? `Añadido ${pkg.title}` : `Quitado ${pkg.title}`, () => {
      // H11: invalidación y persistencia fuera del actualizador de setDraft.
      const prev = draftRef.current;
      if (!prev) return;
      applyDraft(toggleApp(prev, pkg.optionId).draft);
    });
  }

  async function prepareExport(): Promise<PrepareResult> {
    const d = requireDraft();
    if (!d) return { ok: false, reason: "draft-missing" };
    const generation = generationRef.current;
    const snapshot = draftForCore(structuredClone(d));
    const resolved = await port.resolveDraft({
      draft: snapshot,
      catalogRef,
    });
    if (generation !== generationRef.current) {
      return { ok: false, reason: "stale" };
    }
    if (!resolved.ok) {
      fail(resolved.error, "Resolver");
      return { ok: false, reason: "resolve" };
    }
    const validatedRes = await port.validateDraft({
      draft: snapshot,
      catalogRef,
    });
    if (generation !== generationRef.current) {
      return { ok: false, reason: "stale" };
    }
    if (!validatedRes.ok) {
      fail(validatedRes.error, "Validar");
      return { ok: false, reason: "validate" };
    }
    setDiagnostics(validatedRes.value.diagnostics);
    setValidated(true);
    if (validatedRes.value.blocking) {
      setManifest(null);
      preparedGenerationRef.current = null;
      announce("alert", "Validación con bloqueos.");
      return { ok: false, reason: "blocked" };
    }
    const built = await port.buildManifest({
      draft: snapshot,
      catalogRef,
      targetRef: exportTarget("archmaker-profile"),
      expectedResolutionDigest: resolved.value.resolutionDigest,
    });
    if (generation !== generationRef.current) {
      return { ok: false, reason: "stale" };
    }
    if (!built.ok) {
      fail(built.error, "Manifest");
      return { ok: false, reason: "manifest" };
    }
    setManifest(built.value);
    preparedGenerationRef.current = generation;
    return {
      ok: true,
      manifest: built.value,
      diagnostics: validatedRes.value.diagnostics,
      generation,
    };
  }

  async function onGenerate(): Promise<void> {
    if (busy) return;
    setBusy(true);
    goToStep(RAIL_STEPS.length - 1);
    try {
      const prepared = await prepareExport();
      if (prepared.ok) {
        // El manifiesto de catálogo no certifica el plan de almacenamiento (H02).
        setInvalid((prev) => {
          const copy = new Set(prev);
          copy.delete(RAIL_STEPS.length - 1);
          return copy;
        });
        const exclusions = draft ? listFaceExclusions(draft).length : 0;
        const storageNote =
          storage.mode !== null && storage.uiState !== "validated"
            ? " Plan de disco: declarado/propuesto, no validado."
            : "";
        showToast(
          exclusions > 0
            ? `Manifest listo con ${exclusions} exclusión(es) de cara.${storageNote}`
            : `Validación sin bloqueos. Manifest listo.${storageNote}`,
        );
      } else if (prepared.reason === "draft-missing") {
        // requireDraft already announced; keep the user on revisión with status.
        announce("alert", "El draft aún no está listo.");
      }
    } finally {
      setBusy(false);
    }
  }

  async function onForge(): Promise<void> {
    if (busy) return;
    const d = requireDraft();
    if (!d) return;
    setBusy(true);
    try {
      let man = manifest;
      const gate = {
        generation: generationRef.current,
        preparedGeneration: preparedGenerationRef.current,
      };
      if (!man || !isPreparedCurrent(gate)) {
        const prepared = await prepareExport();
        if (!prepared.ok) return;
        man = prepared.manifest;
      }
      if (
        !isPreparedCurrent({
          generation: generationRef.current,
          preparedGeneration: preparedGenerationRef.current,
        })
      ) {
        announce("alert", "La configuración cambió; vuelve a generar.");
        return;
      }
      const token = destinationTokenFor(d.id);
      const res = await port.exportArtifact({
        manifest: man,
        targetRef: exportTarget("archmaker-profile"),
        destination: { token },
      });
      if (!res.ok) {
        fail(res.error, "Exportar");
        return;
      }
      if (
        !isPreparedCurrent({
          generation: generationRef.current,
          preparedGeneration: preparedGenerationRef.current,
        })
      ) {
        announce(
          "alert",
          "Resultado de exportación descartado (edición concurrente).",
        );
        return;
      }
      try {
        const name = exportFilename(token, res.value.targetRef.id);
        // Web: download-started. Escritorio F5: file-saved tras diálogo acotado.
        const outcome = await deliverArtifact(res.value, name);
        if (outcome.kind === "cancelled") {
          announce("status", "Guardado cancelado.");
          return;
        }
        showToast(exportStatusMessage(outcome));
      } catch (err) {
        const message =
          err instanceof DeliveryError
            ? err.message
            : "No se pudo entregar el archivo.";
        announce("alert", message);
      }
    } finally {
      setBusy(false);
    }
  }

  async function onImportFile(file: File): Promise<void> {
    setImportBusy(true);
    // H12: congelar generación al elegir el archivo, antes del análisis async.
    const generationAtPick = generationRef.current;
    try {
      const analysis = await analyzeImportFile({
        file,
        port,
        catalogRef,
        targetRef: DRAFT_TARGET,
      });
      if (generationAtPick !== generationRef.current) {
        announce(
          "alert",
          "La configuración cambió durante el análisis; vuelve a importar.",
        );
        return;
      }
      if (analysis.kind === "too-large") {
        announce(
          "alert",
          `Archivo demasiado grande (${analysis.size} > ${analysis.max} bytes).`,
        );
        return;
      }
      if (analysis.kind === "rejected") {
        fail(analysis.error, "Importar");
        return;
      }
      if (analysis.kind === "needs_blocking_confirm") {
        setImportPending({
          kind: "needs_blocking_confirm",
          sourceBytes: analysis.sourceBytes,
          generation: generationAtPick,
          error: analysis.error,
        });
        announce(
          "status",
          "La importación tiene pérdidas bloqueantes; confirma para continuar.",
        );
        return;
      }
      setImportPending({
        kind: "ready",
        sourceBytes: analysis.sourceBytes,
        generation: generationAtPick,
        payload: analysis.payload,
        needsBlockingConfirm: analysis.needsBlockingConfirm,
      });
      announce(
        "status",
        `Informe de importación: ${summarizeReport(analysis.payload.report)}`,
      );
    } finally {
      setImportBusy(false);
    }
  }

  async function onConfirmImport(
    confirmBlockingLosses: boolean,
  ): Promise<void> {
    if (!importPending) return;
    setImportBusy(true);
    try {
      const applied = await applyConsentedImport({
        port,
        catalogRef,
        targetRef: DRAFT_TARGET,
        sourceBytes: importPending.sourceBytes,
        confirmBlockingLosses,
        expectedGeneration: importPending.generation,
        getCurrentGeneration: () => generationRef.current,
      });
      if (!applied.ok) {
        if (applied.error) {
          fail(applied.error, "Importar");
        } else {
          announce("alert", applied.message);
        }
        return;
      }
      setImportPending(null);
      applyDraft(applied.draft);
      showToast("Perfil importado.");
    } finally {
      setImportBusy(false);
    }
  }

  async function onRetryPersist(): Promise<void> {
    if (!draft) return;
    const result = await retryPersistDraftLocal(draft);
    handlePersistResult(result);
    if (result.ok) {
      showToast("Perfil guardado localmente.");
    }
  }

  function onDownloadPersistCopy(): void {
    const text = persistExportable ?? (draft ? exportDraftCopy(draft) : null);
    if (!text) {
      announce("alert", "No hay copia exportable.");
      return;
    }
    const blob = new Blob([text], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "archmaker-draft-copy.json";
    a.rel = "noopener";
    document.body.append(a);
    a.click();
    a.remove();
    URL.revokeObjectURL(url);
    showToast("Copia exportable descargada.");
  }

  function goToStep(index: number): void {
    const next = Math.min(RAIL_STEPS.length - 1, Math.max(0, index));
    setVisited((prev) => new Set(prev).add(next));
    // H13: visitar un paso no elimina su invalidez; solo una corrección o
    // nueva validación puede limpiarla (p. ej. onGenerate en revisión).
    setStep(next);
  }

  function nav(delta: number): void {
    goToStep(step + delta);
  }

  function patchStorage(patch: Partial<StorageIntent>): void {
    const d = draft;
    if (!d) return;
    const next: StorageIntent = {
      ...storage,
      ...patch,
    };
    next.uiState = uiStateForIntent(next);
    setInvalid((prev) => new Set(prev).add(RAIL_STEPS.length - 1));
    applyDraft(d, next);
  }

  function pickStorageMode(mode: StorageMode): void {
    patchStorage({
      mode,
      lossRiskAcknowledged:
        mode === "replace-disk" ? storage.lossRiskAcknowledged : false,
    });
  }

  return (
    <>
      <a className="skip-link" href="#main">
        Saltar al contenido
      </a>
      <ForgeHeader
        dark={dark}
        onToggleDark={() => setDark((v) => !v)}
        onGlossary={() => setGlossaryOpen(true)}
        onCommand={() => setCmdOpen(true)}
        onGenerate={() => void onGenerate()}
      />
      <div className="forge-shell">
        <ForgeRail step={step} progress={stepProgress} onSelect={goToStep} />
        <ForgeStepper step={step} progress={stepProgress} onSelect={goToStep} />
        <main id="main" className="forge-main" tabIndex={-1}>
          {!ready ? <p role="status">Preparando draft…</p> : null}

          {step === 0 ? (
            <section aria-labelledby="storage-h">
              <h1 id="storage-h">Disco y particiones</h1>
              <p className="forge-lead">
                Declara el modo de instalación. No hay detección en esta entrega
                web; el perfil no modifica discos.
              </p>
              <div className="forge-callout forge-callout-info" role="status">
                Esta fase prepara un perfil y no modifica discos.
              </div>
              <div
                className="forge-grid-2"
                role="radiogroup"
                aria-label="Modo de disco"
              >
                {(
                  [
                    {
                      mode: "new-disk" as const,
                      title: "Disco nuevo o sin datos a conservar",
                      description:
                        "Declaración del usuario. No se deduce de un sondeo.",
                    },
                    {
                      mode: "replace-disk" as const,
                      title: "Reemplazo completo",
                      description:
                        "Se perderán los datos del destino. Requiere confirmación.",
                    },
                  ] as const
                ).map((opt) => {
                  const selected = storage.mode === opt.mode;
                  return (
                    <button
                      key={opt.mode}
                      type="button"
                      className={`forge-card${selected ? " is-selected" : ""}`}
                      role="radio"
                      aria-checked={selected}
                      data-storage-mode={opt.mode}
                      onClick={() => pickStorageMode(opt.mode)}
                    >
                      <span className="forge-card-title">{opt.title}</span>
                      <span className="forge-card-desc">{opt.description}</span>
                    </button>
                  );
                })}
              </div>
              <div className="forge-stack forge-narrow">
                <label className="forge-check">
                  <input
                    type="checkbox"
                    checked={storage.intentAcknowledged}
                    onChange={(e) =>
                      patchStorage({ intentAcknowledged: e.target.checked })
                    }
                  />
                  Confirmo que esta es una declaración de intención para el
                  perfil (no una escritura en disco).
                </label>
                {storage.mode === "replace-disk" ? (
                  <label className="forge-check">
                    <input
                      type="checkbox"
                      checked={storage.lossRiskAcknowledged}
                      onChange={(e) =>
                        patchStorage({
                          lossRiskAcknowledged: e.target.checked,
                        })
                      }
                    />
                    Entiendo el riesgo de pérdida total en un reemplazo.
                  </label>
                ) : null}
                <p className="forge-muted" role="status">
                  Modo: {labelStorageMode(storage.mode)}. Estado:{" "}
                  {storage.uiState}. No hay botón Formatear en esta entrega.
                </p>
              </div>
              <div className="forge-nav-row forge-nav-end">
                <button
                  type="button"
                  className="forge-btn-primary"
                  onClick={() => nav(1)}
                >
                  Continuar → Sistema base
                </button>
              </div>
            </section>
          ) : null}

          {step === 1 ? (
            <section aria-labelledby="base-h">
              <h2 id="base-h">Sistema base</h2>
              <p className="forge-lead">
                Elige kernel, arranque y sistema de archivos. UKI no es un
                gestor de arranque aparte.
              </p>
              <div className="forge-stack">
                <h3 className="forge-section-label">Kernel</h3>
                <div
                  role="radiogroup"
                  aria-label="Kernel"
                  className="forge-stack"
                >
                  {KERNELS.map((k) => {
                    const selected = kernelId === k.optionId;
                    const unverified = isUnverifiedOption(k);
                    return (
                      <button
                        key={k.optionId}
                        type="button"
                        className={`forge-card${selected ? " is-selected" : ""}${unverified ? " is-gap" : ""}`}
                        role="radio"
                        aria-checked={selected}
                        data-option-id={k.optionId}
                        onClick={() => pickSingle(STEP_IDS.kernel, k.optionId)}
                      >
                        <span className="forge-card-row">
                          <span className="forge-card-title">
                            {catalogOptionTitle(catalogView, k.optionId) ??
                              k.title}{" "}
                            <span className="forge-chip">{k.version}</span>
                          </span>
                          {k.tag ? (
                            <span className="forge-tag">{k.tag}</span>
                          ) : null}
                        </span>
                        <span className="forge-card-desc">{k.description}</span>
                      </button>
                    );
                  })}
                </div>
                <fieldset className="forge-fieldset">
                  <legend>Bootloader</legend>
                  <div
                    role="radiogroup"
                    aria-label="Bootloader"
                    className="forge-radio-row"
                  >
                    {BOOTLOADERS.map((b) => {
                      const selected = bootId === b.optionId;
                      const unverified = isUnverifiedOption(b);
                      return (
                        <label
                          key={b.optionId}
                          className={`forge-radio-label${unverified ? " is-gap" : ""}`}
                        >
                          <input
                            type="radio"
                            name="boot"
                            checked={selected}
                            onChange={() =>
                              pickSingle(STEP_IDS.boot, b.optionId)
                            }
                            data-option-id={b.optionId}
                          />
                          {catalogOptionTitle(catalogView, b.optionId) ??
                            b.title}
                        </label>
                      );
                    })}
                  </div>
                </fieldset>
                <fieldset className="forge-fieldset">
                  <legend>Filesystem</legend>
                  <div
                    role="radiogroup"
                    aria-label="Filesystem"
                    className="forge-radio-row"
                  >
                    {FILESYSTEMS.map((f) => {
                      const selected = fsId === f.optionId;
                      return (
                        <label key={f.optionId} className="forge-radio-label">
                          <input
                            type="radio"
                            name="fs"
                            checked={selected}
                            onChange={() =>
                              pickSingle(STEP_IDS.filesystem, f.optionId)
                            }
                            data-option-id={f.optionId}
                          />
                          {catalogOptionTitle(catalogView, f.optionId) ??
                            f.title}
                        </label>
                      );
                    })}
                  </div>
                </fieldset>
              </div>
              <div className="forge-nav-row">
                <button
                  type="button"
                  className="forge-btn-ghost"
                  onClick={() => nav(-1)}
                >
                  ← Atrás
                </button>
                <button
                  type="button"
                  className="forge-btn-primary"
                  onClick={() => nav(1)}
                >
                  Continuar → Entorno
                </button>
              </div>
            </section>
          ) : null}

          {step === 2 ? (
            <section aria-labelledby="env-h">
              <div className="forge-heading-row">
                <h2 id="env-h">Entorno gráfico</h2>
                <span className="forge-chip">{envs.length} opciones</span>
              </div>
              <p className="forge-lead">
                Escritorios y gestores de ventanas del catálogo.
              </p>
              <div
                className="forge-grid-2"
                role="radiogroup"
                aria-label="Entorno gráfico"
              >
                {envs.map((e) => {
                  const selected = envId === e.optionId;
                  const unverified = isUnverifiedOption(e);
                  return (
                    <button
                      key={e.optionId}
                      type="button"
                      className={`forge-card${selected ? " is-selected" : ""}${unverified ? " is-gap" : ""}`}
                      role="radio"
                      aria-checked={selected}
                      data-option-id={e.optionId}
                      onClick={() =>
                        pickSingle(STEP_IDS.compositor, e.optionId)
                      }
                    >
                      <span className="forge-card-row">
                        <span className="forge-card-title">
                          {e.title}{" "}
                          <span className="forge-chip">{e.version}</span>
                        </span>
                        <span className="forge-chip">{e.protocol}</span>
                      </span>
                      <span
                        className="forge-preview-mini"
                        data-env={e.htmlId}
                      />
                      <span className="forge-card-desc">
                        <strong>Ideal:</strong> {e.ideal}
                      </span>
                    </button>
                  );
                })}
              </div>
              <div className="forge-nav-row">
                <button
                  type="button"
                  className="forge-btn-ghost"
                  onClick={() => nav(-1)}
                >
                  ← Atrás
                </button>
                <button
                  type="button"
                  className="forge-btn-primary"
                  onClick={() => nav(1)}
                >
                  Continuar → Apps
                </button>
              </div>
            </section>
          ) : null}

          {step === 3 ? (
            <section aria-labelledby="apps-h">
              <h2 id="apps-h">Apps y paquetes</h2>
              <p className="forge-lead">
                {selectedApps.size} seleccionadas · ≈{totalSize} MB (estimación
                de cara)
              </p>
              <div className="forge-stack" id="options">
                {layers.map((layer) => {
                  const count = layer.packages.filter((p) =>
                    selectedApps.has(p.optionId),
                  ).length;
                  return (
                    <div key={layer.id} className="forge-layer">
                      <div className="forge-layer-head">
                        <div>
                          <span aria-hidden="true">{layer.icon} </span>
                          <strong>{layer.title}</strong>
                          <p className="forge-muted">{layer.description}</p>
                        </div>
                        <span className="forge-chip">
                          {count}/{layer.packages.length}
                        </span>
                      </div>
                      <div
                        className="forge-stack"
                        role="group"
                        aria-label={layer.title}
                      >
                        {layer.packages.map((pkg) => {
                          const selected = selectedApps.has(pkg.optionId);
                          const unverified = isUnverifiedOption(pkg);
                          return (
                            <div
                              key={pkg.optionId}
                              className={`forge-pkg-row${selected ? " is-selected" : ""}${unverified ? " is-gap" : ""}`}
                            >
                              <input
                                type="checkbox"
                                role="checkbox"
                                checked={selected}
                                aria-checked={selected}
                                aria-label={pkg.title}
                                data-option-id={pkg.optionId}
                                onChange={() => onTogglePackage(pkg)}
                              />
                              <button
                                type="button"
                                className="forge-pkg-open"
                                onClick={() => setPkgModal(pkg)}
                              >
                                <span className="forge-card-row">
                                  <span className="forge-card-title">
                                    {pkg.title}{" "}
                                    <span className="forge-muted">
                                      {pkg.size} MB
                                    </span>
                                  </span>
                                  <span className="forge-tag">{pkg.tag}</span>
                                </span>
                                <span className="forge-card-desc">
                                  {pkg.description}
                                </span>
                              </button>
                            </div>
                          );
                        })}
                      </div>
                    </div>
                  );
                })}
              </div>
              <div className="forge-nav-row">
                <button
                  type="button"
                  className="forge-btn-ghost"
                  onClick={() => nav(-1)}
                >
                  ← Atrás
                </button>
                <button
                  type="button"
                  className="forge-btn-primary"
                  onClick={() => nav(1)}
                >
                  Continuar → Identidad
                </button>
              </div>
            </section>
          ) : null}

          {step === 4 ? (
            <section aria-labelledby="id-h">
              <h2 id="id-h">Identidad y seguridad</h2>
              <div className="forge-stack forge-narrow">
                <label className="forge-field">
                  <span>Hostname</span>
                  <input
                    value={hostname}
                    placeholder="mi-equipo"
                    onChange={(e) => {
                      const d = requireDraft();
                      if (!d) return;
                      applyDraft(setText(d, STEP_IDS.hostname, e.target.value));
                    }}
                  />
                </label>
                <div className="forge-info-card">
                  <strong>Protege el disco (cifrado)</strong>
                  <p className="forge-muted">
                    Solo intención en el perfil exportado. Esta cara no cifra el
                    disco.
                  </p>
                  <label className="forge-check">
                    <input
                      type="checkbox"
                      role="checkbox"
                      checked={encrypt}
                      aria-checked={encrypt}
                      onChange={(e) => {
                        const d = requireDraft();
                        if (!d) return;
                        applyDraft(
                          setBool(d, STEP_IDS.encrypt, e.target.checked),
                        );
                      }}
                    />
                    Activar cifrado
                  </label>
                </div>
              </div>
              <div className="forge-nav-row">
                <button
                  type="button"
                  className="forge-btn-ghost"
                  onClick={() => nav(-1)}
                >
                  ← Atrás
                </button>
                <button
                  type="button"
                  className="forge-btn-primary"
                  onClick={() => nav(1)}
                >
                  Continuar → Revisión
                </button>
              </div>
            </section>
          ) : null}

          {step === 5 ? (
            <section aria-labelledby="review-h">
              <h2 id="review-h">Revisión</h2>
              <div className="forge-callout forge-callout-info">
                <strong>Listo para forjar</strong>
                <p>
                  Disco {labelStorageMode(storage.mode)} · Kernel{" "}
                  {kernel?.title ?? "—"} · Arranque {boot?.title ?? "—"} ·{" "}
                  {filesystem?.title ?? "—"} · {selectedApps.size} apps ·{" "}
                  {env?.title ?? "—"}
                </p>
                {!validated ? (
                  <p role="status">Pendiente de validar.</p>
                ) : diagnostics.length > 0 ? (
                  <p role="status">{diagnostics.length} diagnósticos.</p>
                ) : (
                  <p role="status">Sin incidencias en la última validación.</p>
                )}
                {draft && listFaceExclusions(draft).length > 0 ? (
                  <p role="status">
                    {listFaceExclusions(draft).length} exclusión(es) de cara
                    (intención o no verificada) no entran al perfil sellado.
                  </p>
                ) : null}
                {draft?.storagePlan ? (
                  <p role="status">
                    Plan de disco: {draft.storagePlan.planStatus} (estado UI{" "}
                    {storage.uiState}). La generación del manifiesto no valida
                    el plan.
                  </p>
                ) : (
                  <p role="status">Plan de disco: sin declarar.</p>
                )}
                {hostname ? <p role="status">Hostname: {hostname}</p> : null}
                {encrypt ? (
                  <p role="status">
                    Cifrado: intención marcada (no aplicada al disco).
                  </p>
                ) : null}
                {portKind === "mock" ? (
                  <p role="status">Modo demostración (MockCorePort).</p>
                ) : null}
                {persistStatus !== "idle" ? (
                  <p role="status" data-persist-status={persistStatus}>
                    Persistencia local: {persistStatusLabel(persistStatus)}
                    {persistStatus === "error" ? (
                      <>
                        {" "}
                        <button
                          type="button"
                          className="forge-btn-ghost"
                          onClick={() => void onRetryPersist()}
                        >
                          Reintentar
                        </button>{" "}
                        <button
                          type="button"
                          className="forge-btn-ghost"
                          onClick={onDownloadPersistCopy}
                        >
                          Descargar copia
                        </button>
                      </>
                    ) : null}
                  </p>
                ) : null}
              </div>
              <div className="forge-nav-row">
                <button
                  type="button"
                  className="forge-btn-ghost"
                  onClick={() => nav(-1)}
                >
                  ← Atrás
                </button>
                <label className="forge-btn-ghost">
                  {importBusy ? "Analizando…" : "Importar"}
                  <input
                    type="file"
                    accept="application/json,.json"
                    hidden
                    disabled={importBusy || Boolean(importPending)}
                    onChange={(e) => {
                      const file = e.target.files?.[0];
                      if (file) void onImportFile(file);
                      e.target.value = "";
                    }}
                  />
                </label>
                <button
                  type="button"
                  className="forge-btn-primary"
                  id="btn-export"
                  disabled={busy}
                  onClick={() => void onForge()}
                >
                  {busy ? "Exportando…" : "Exportar"}
                </button>
              </div>
            </section>
          ) : null}
        </main>
        <ForgeAside
          kernel={kernel}
          bootTitle={boot?.title ?? "systemd-boot"}
          filesystemTitle={filesystem?.title ?? "Btrfs"}
          env={env}
          storage={storage}
          hasManifest={Boolean(manifest)}
          apps={selectedPackages}
          appCount={selectedApps.size}
          totalSizeEstimate={totalSize}
          diagnostics={sideDiagnostics}
          effectiveSelections={manifest?.effectiveSelections}
          engineDiagnostics={diagnostics}
          onPickEnv={(optionId) => pickSingle(STEP_IDS.compositor, optionId)}
        />
      </div>

      {toast ? (
        <div className="forge-toast" role="status">
          <span>{toast.message}</span>
          {toast.undo ? (
            <button type="button" onClick={() => toast.undo?.()}>
              Deshacer
            </button>
          ) : null}
        </div>
      ) : null}

      {pkgModal ? (
        <PackageDialog
          pkg={pkgModal}
          selected={selectedApps.has(pkgModal.optionId)}
          unavailable={isUnverifiedOption(pkgModal)}
          onToggle={() => {
            onTogglePackage(pkgModal);
            setPkgModal(null);
          }}
          onClose={() => setPkgModal(null)}
        />
      ) : null}

      {importPending ? (
        <ImportReportDialog
          pending={importPending}
          busy={importBusy}
          onConfirm={(confirmBlocking) => void onConfirmImport(confirmBlocking)}
          onClose={() => setImportPending(null)}
        />
      ) : null}

      {cmdOpen ? (
        <CommandPalette
          selectedIds={selectedApps}
          onToggle={onTogglePackage}
          onClose={() => setCmdOpen(false)}
        />
      ) : null}

      {glossaryOpen ? (
        <GlossaryDrawer onClose={() => setGlossaryOpen(false)} />
      ) : null}

      <div id="toast-region" aria-live="polite" role="status" />
      <div id="alert-region" aria-live="assertive" role="alert" />
    </>
  );
}
