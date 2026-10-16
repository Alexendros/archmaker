import type { Draft, Selection } from "./coreport.js";
import { withSelection } from "./coreport.js";
import { STEP_IDS } from "./forge-face.js";

export function singleId(
  draft: Draft | null,
  stepId: string,
): string | undefined {
  if (!draft) return undefined;
  const sel = draft.selections.find((s) => s.stepId === stepId);
  if (!sel || sel.value.kind !== "single") return undefined;
  return sel.value.optionId;
}

export function appIds(draft: Draft | null): string[] {
  if (!draft) return [];
  const sel = draft.selections.find((s) => s.stepId === STEP_IDS.apps);
  if (!sel || sel.value.kind !== "multiple") return [];
  return sel.value.optionIds;
}

export function textValue(draft: Draft | null, stepId: string): string {
  if (!draft) return "";
  const sel = draft.selections.find((s) => s.stepId === stepId);
  if (!sel || sel.value.kind !== "text") return "";
  return sel.value.value;
}

export function boolValue(draft: Draft | null, stepId: string): boolean {
  if (!draft) return false;
  const sel = draft.selections.find((s) => s.stepId === stepId);
  if (!sel || sel.value.kind !== "boolean") return false;
  return sel.value.value;
}

export function chooseSingle(
  draft: Draft,
  stepId: string,
  optionId: string,
): Draft {
  return withSelection(draft, {
    stepId,
    value: { kind: "single", optionId },
  });
}

export function toggleApp(
  draft: Draft,
  optionId: string,
): {
  draft: Draft;
  added: boolean;
} {
  const current = appIds(draft);
  const has = current.includes(optionId);
  const optionIds = has
    ? current.filter((id) => id !== optionId)
    : [...current, optionId];
  const sel: Selection = {
    stepId: STEP_IDS.apps,
    value: { kind: "multiple", optionIds },
  };
  return { draft: withSelection(draft, sel), added: !has };
}

export function setText(draft: Draft, stepId: string, value: string): Draft {
  return withSelection(draft, {
    stepId,
    value: { kind: "text", value },
  });
}

export function setBool(draft: Draft, stepId: string, value: boolean): Draft {
  return withSelection(draft, {
    stepId,
    value: { kind: "boolean", value },
  });
}
