import type { Draft } from "./coreport.js";

const DB_NAME = "archmaker-profiles";
const STORE = "drafts";
const LAST_ID_KEY = "archmaker.lastDraftId";

/** Estados tipados de persistencia local (C-05 / F4). */
export type PersistStatus = "idle" | "saving" | "saved" | "error";

export type PersistFailureReason = "unavailable" | "quota" | "unknown";

export type PersistResult =
  | { ok: true; status: "saved" }
  | {
      ok: false;
      status: "error";
      reason: PersistFailureReason;
      message: string;
      /** Copia JSON exportable cuando el almacén no está disponible. */
      exportable: string;
    };

type StatusListener = (status: PersistStatus) => void;

let writeChain: Promise<void> = Promise.resolve();
let statusListeners = new Set<StatusListener>();
let lastStatus: PersistStatus = "idle";

export function getPersistStatus(): PersistStatus {
  return lastStatus;
}

export function subscribePersistStatus(listener: StatusListener): () => void {
  statusListeners.add(listener);
  listener(lastStatus);
  return () => {
    statusListeners.delete(listener);
  };
}

function setStatus(status: PersistStatus): void {
  lastStatus = status;
  for (const listener of statusListeners) {
    listener(status);
  }
}

/** Serializa un draft a JSON estable para copia exportable / reintento. */
export function exportDraftCopy(draft: Draft): string {
  return `${JSON.stringify(draft, null, 2)}\n`;
}

function classifyPersistError(err: unknown): PersistFailureReason {
  const name =
    err && typeof err === "object" && "name" in err
      ? String((err as { name: unknown }).name)
      : "";
  const message = err instanceof Error ? err.message : String(err ?? "");
  if (
    name === "QuotaExceededError" ||
    /quota/i.test(message) ||
    /NS_ERROR_DOM_QUOTA/i.test(message)
  ) {
    return "quota";
  }
  if (
    typeof indexedDB === "undefined" ||
    /indexedDB/i.test(message) ||
    /SecurityError/i.test(name) ||
    /private/i.test(message)
  ) {
    return "unavailable";
  }
  return "unknown";
}

function openDb(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    if (typeof indexedDB === "undefined") {
      reject(new Error("indexedDB unavailable"));
      return;
    }
    const req = indexedDB.open(DB_NAME, 1);
    req.onupgradeneeded = () => {
      const db = req.result;
      if (!db.objectStoreNames.contains(STORE)) {
        db.createObjectStore(STORE, { keyPath: "id" });
      }
    };
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(req.error ?? new Error("indexedDB open failed"));
  });
}

async function putDraft(draft: Draft): Promise<void> {
  const db = await openDb();
  await new Promise<void>((resolve, reject) => {
    const tx = db.transaction(STORE, "readwrite");
    tx.objectStore(STORE).put(draft);
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error ?? new Error("indexedDB put failed"));
  });
  window.localStorage.setItem(LAST_ID_KEY, draft.id);
}

async function runPersist(draft: Draft): Promise<PersistResult> {
  setStatus("saving");
  const exportable = exportDraftCopy(draft);
  try {
    await putDraft(draft);
    setStatus("saved");
    return { ok: true, status: "saved" };
  } catch (err) {
    const reason = classifyPersistError(err);
    setStatus("error");
    return {
      ok: false,
      status: "error",
      reason,
      message:
        reason === "unavailable"
          ? "Almacén local no disponible."
          : reason === "quota"
            ? "Sin espacio en el almacén local."
            : "No se pudo guardar el perfil localmente.",
      exportable,
    };
  }
}

/**
 * Encola una escritura. Las escrituras se serializan (una tras otra).
 * Emite estados guardando / guardado / error.
 */
export function persistDraftLocal(draft: Draft): Promise<PersistResult> {
  const job = writeChain.then(() => runPersist(draft));
  writeChain = job.then(
    () => undefined,
    () => undefined,
  );
  return job;
}

/** Reintento explícito cuando el almacén había fallado. */
export function retryPersistDraftLocal(draft: Draft): Promise<PersistResult> {
  return persistDraftLocal(draft);
}

export type LoadLastDraftResult =
  | { kind: "none" }
  | { kind: "found"; draft: Draft }
  | { kind: "error"; reason: PersistFailureReason; message: string };

/**
 * Distingue ausencia de borrador, borrador recuperado y fallo de almacén (H14).
 * No convierte un error de IndexedDB en «perfil nuevo» silencioso.
 */
export async function loadLastDraftLocal(): Promise<LoadLastDraftResult> {
  let id: string | null;
  try {
    id = window.localStorage.getItem(LAST_ID_KEY);
  } catch (err) {
    const reason = classifyPersistError(err);
    return {
      kind: "error",
      reason,
      message:
        reason === "unavailable"
          ? "Almacén local no disponible."
          : "No se pudo leer el identificador del último perfil.",
    };
  }
  if (!id) return { kind: "none" };
  try {
    const db = await openDb();
    const draft = await new Promise<Draft | null>((resolve, reject) => {
      const tx = db.transaction(STORE, "readonly");
      const req = tx.objectStore(STORE).get(id);
      req.onsuccess = () => resolve((req.result as Draft | undefined) ?? null);
      req.onerror = () =>
        reject(req.error ?? new Error("indexedDB get failed"));
    });
    if (!draft) return { kind: "none" };
    return { kind: "found", draft };
  } catch (err) {
    const reason = classifyPersistError(err);
    return {
      kind: "error",
      reason,
      message:
        reason === "unavailable"
          ? "Almacén local no disponible."
          : reason === "quota"
            ? "Sin espacio en el almacén local."
            : "No se pudo recuperar el perfil local.",
    };
  }
}

export function clearLastDraftId(): void {
  window.localStorage.removeItem(LAST_ID_KEY);
}

/** Solo tests: reinicia cola y estado. */
export function resetPersistQueueForTests(): void {
  writeChain = Promise.resolve();
  lastStatus = "idle";
  statusListeners = new Set();
}
