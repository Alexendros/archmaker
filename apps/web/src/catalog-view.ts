/**
 * Lectura de catálogo canónico para etiquetas de presentación (H06).
 * forge-face conserva iconos/agrupación editorial; los títulos verificables
 * preferen la respuesta de loadCatalog cuando está disponible.
 */

export type CatalogOptionView = {
  id: string;
  title: string;
  description?: string;
  provider?: {
    status?: string;
    channel?: string;
    packageId?: string;
    revision?: string;
    checkedAt?: string;
  };
};

export type CatalogView = {
  namespace: string;
  id: string;
  version: string;
  steps: Array<{
    id: string;
    title: string;
    sections: Array<{ options: CatalogOptionView[] }>;
  }>;
};

export function parseCatalogView(raw: unknown): CatalogView | null {
  if (!raw || typeof raw !== "object") return null;
  const c = raw as Record<string, unknown>;
  if (typeof c.namespace !== "string" || typeof c.id !== "string") return null;
  if (!Array.isArray(c.steps)) return null;
  return raw as CatalogView;
}

export function catalogOptionTitle(
  catalog: CatalogView | null,
  optionId: string,
): string | undefined {
  if (!catalog) return undefined;
  for (const step of catalog.steps) {
    for (const section of step.sections) {
      const hit = section.options.find((o) => o.id === optionId);
      if (hit) return hit.title;
    }
  }
  return undefined;
}

export function catalogHasProvider(
  catalog: CatalogView | null,
  optionId: string,
): boolean {
  if (!catalog) return false;
  for (const step of catalog.steps) {
    for (const section of step.sections) {
      const hit = section.options.find((o) => o.id === optionId);
      if (hit) return Boolean(hit.provider);
    }
  }
  return false;
}
