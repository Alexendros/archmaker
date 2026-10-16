import type { Draft } from "./coreport.js";

const DB_NAME = "archmaker-profiles";
const STORE = "drafts";
const LAST_ID_KEY = "archmaker.lastDraftId";

function openDb(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
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

export async function persistDraftLocal(draft: Draft): Promise<void> {
  try {
    const db = await openDb();
    await new Promise<void>((resolve, reject) => {
      const tx = db.transaction(STORE, "readwrite");
      tx.objectStore(STORE).put(draft);
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(tx.error ?? new Error("indexedDB put failed"));
    });
    window.localStorage.setItem(LAST_ID_KEY, draft.id);
  } catch {
    // Quota / private mode: persistence is best-effort (documented limit).
  }
}

export async function loadLastDraftLocal(): Promise<Draft | null> {
  try {
    const id = window.localStorage.getItem(LAST_ID_KEY);
    if (!id) return null;
    const db = await openDb();
    return await new Promise((resolve, reject) => {
      const tx = db.transaction(STORE, "readonly");
      const req = tx.objectStore(STORE).get(id);
      req.onsuccess = () => resolve((req.result as Draft | undefined) ?? null);
      req.onerror = () =>
        reject(req.error ?? new Error("indexedDB get failed"));
    });
  } catch {
    return null;
  }
}

export function clearLastDraftId(): void {
  window.localStorage.removeItem(LAST_ID_KEY);
}
