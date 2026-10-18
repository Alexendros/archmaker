/**
 * Lectura de catálogo canónico para etiquetas de presentación (H06).
 * forge-face conserva iconos/agrupación editorial; los títulos verificables
 * preferen la respuesta de loadCatalog cuando está disponible.
 *
 * Acepta forma canónica (steps[].sections[].options) y la cara mock
 * (steps[].options / steps[].layers[].options).
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

function asOption(raw: unknown): CatalogOptionView | null {
  if (!raw || typeof raw !== "object") return null;
  const o = raw as Record<string, unknown>;
  if (typeof o.id !== "string" || typeof o.title !== "string") return null;
  const out: CatalogOptionView = { id: o.id, title: o.title };
  if (typeof o.description === "string") out.description = o.description;
  if (o.provider && typeof o.provider === "object") {
    out.provider = o.provider as CatalogOptionView["provider"];
  }
  return out;
}

function optionsFromUnknown(raw: unknown): CatalogOptionView[] {
  if (!Array.isArray(raw)) return [];
  const out: CatalogOptionView[] = [];
  for (const item of raw) {
    const opt = asOption(item);
    if (opt) out.push(opt);
  }
  return out;
}

function sectionsFromStep(step: Record<string, unknown>): Array<{
  options: CatalogOptionView[];
}> {
  if (Array.isArray(step.sections)) {
    return step.sections.map((section) => {
      const s =
        section && typeof section === "object"
          ? (section as Record<string, unknown>)
          : {};
      return { options: optionsFromUnknown(s.options) };
    });
  }

  const sections: Array<{ options: CatalogOptionView[] }> = [];
  const direct = optionsFromUnknown(step.options);
  if (direct.length > 0) sections.push({ options: direct });

  if (Array.isArray(step.layers)) {
    for (const layer of step.layers) {
      const l =
        layer && typeof layer === "object"
          ? (layer as Record<string, unknown>)
          : {};
      const opts = optionsFromUnknown(l.options);
      if (opts.length > 0) sections.push({ options: opts });
    }
  }

  return sections;
}

export function parseCatalogView(raw: unknown): CatalogView | null {
  if (!raw || typeof raw !== "object") return null;
  const c = raw as Record<string, unknown>;
  if (typeof c.namespace !== "string" || typeof c.id !== "string") return null;
  if (typeof c.version !== "string") return null;
  if (!Array.isArray(c.steps)) return null;

  const steps: CatalogView["steps"] = [];
  for (const item of c.steps) {
    if (!item || typeof item !== "object") continue;
    const step = item as Record<string, unknown>;
    if (typeof step.id !== "string" || typeof step.title !== "string") continue;
    steps.push({
      id: step.id,
      title: step.title,
      sections: sectionsFromStep(step),
    });
  }
  if (steps.length === 0) return null;

  return {
    namespace: c.namespace,
    id: c.id,
    version: c.version,
    steps,
  };
}

export function catalogOptionTitle(
  catalog: CatalogView | null,
  optionId: string,
): string | undefined {
  if (!catalog) return undefined;
  for (const step of catalog.steps) {
    for (const section of step.sections ?? []) {
      const hit = (section.options ?? []).find((o) => o.id === optionId);
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
    for (const section of step.sections ?? []) {
      const hit = (section.options ?? []).find((o) => o.id === optionId);
      if (hit) return Boolean(hit.provider);
    }
  }
  return false;
}

/** Opciones canónicas de un paso (orden del catálogo). */
export function catalogOptionsForStep(
  catalog: CatalogView | null,
  stepId: string,
): CatalogOptionView[] {
  if (!catalog) return [];
  const step = catalog.steps.find((s) => s.id === stepId);
  if (!step) return [];
  const out: CatalogOptionView[] = [];
  for (const section of step.sections ?? []) {
    for (const opt of section.options ?? []) {
      out.push(opt);
    }
  }
  return out;
}

type FaceLike = {
  htmlId: string;
  optionId: string;
  stepId: string;
  title: string;
  description: string;
  tag?: string;
  version?: string;
  gap?: boolean;
  verificationStatus?: "verified" | "unverified";
};

/**
 * H06: ids y títulos desde el catálogo; copy editorial (descripción, tag, version)
 * desde la cara cuando existe. Sin catálogo, se conservan las constantes de cara.
 */
export function mergeFaceOptionsFromCatalog<T extends FaceLike>(
  catalog: CatalogView | null,
  stepId: string,
  faceDefaults: T[],
): T[] {
  const fromCatalog = catalogOptionsForStep(catalog, stepId);
  if (fromCatalog.length === 0) return faceDefaults;

  return fromCatalog.map((co) => {
    const face = faceDefaults.find((f) => f.optionId === co.id);
    if (face) {
      return { ...face, title: co.title };
    }
    return {
      ...(faceDefaults[0]
        ? {
            gap: faceDefaults[0].gap,
            verificationStatus: faceDefaults[0].verificationStatus,
          }
        : {}),
      htmlId: co.id.replace(/\./g, "-"),
      optionId: co.id,
      stepId,
      title: co.title,
      description: co.description ?? "",
    } as T;
  });
}
