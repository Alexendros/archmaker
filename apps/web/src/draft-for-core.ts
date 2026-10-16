import type { Draft } from "./coreport.js";
import { STEP_IDS, findOptionById } from "./neubat-face.js";

export type FaceExclusion = {
  stepId: string;
  reason: "face-only-intent" | "gap-option";
  optionIds: string[];
};

/**
 * Face-only selections (intent) and gap packages stay in the UI draft but must
 * not reach RealCore resolve/validate/manifest (AM-RES-001).
 * Hostname (text) and encrypt (boolean intention) are kept for core fidelity.
 */
export function listFaceExclusions(draft: Draft): FaceExclusion[] {
  const exclusions: FaceExclusion[] = [];
  for (const selection of draft.selections) {
    if (selection.stepId === STEP_IDS.intent) {
      const optionIds =
        selection.value.kind === "single"
          ? [selection.value.optionId]
          : selection.value.kind === "multiple"
            ? selection.value.optionIds
            : [];
      exclusions.push({
        stepId: selection.stepId,
        reason: "face-only-intent",
        optionIds,
      });
      continue;
    }
    const value = selection.value;
    if (value.kind === "single") {
      const opt = findOptionById(value.optionId);
      if (opt && "gap" in opt && opt.gap) {
        exclusions.push({
          stepId: selection.stepId,
          reason: "gap-option",
          optionIds: [value.optionId],
        });
      }
    } else if (value.kind === "multiple") {
      const gapIds = value.optionIds.filter((id) => {
        const opt = findOptionById(id);
        return Boolean(opt && "gap" in opt && opt.gap);
      });
      if (gapIds.length > 0) {
        exclusions.push({
          stepId: selection.stepId,
          reason: "gap-option",
          optionIds: gapIds,
        });
      }
    }
  }
  return exclusions;
}

export function draftForCore(draft: Draft): Draft {
  const selections = draft.selections.flatMap((selection) => {
    if (selection.stepId === STEP_IDS.intent) return [];
    const value = selection.value;
    if (value.kind === "single") {
      const opt = findOptionById(value.optionId);
      if (opt && "gap" in opt && opt.gap) return [];
      return [selection];
    }
    if (value.kind === "multiple") {
      const optionIds = value.optionIds.filter((id) => {
        const opt = findOptionById(id);
        return !(opt && "gap" in opt && opt.gap);
      });
      if (optionIds.length === 0) return [];
      return [
        { ...selection, value: { kind: "multiple" as const, optionIds } },
      ];
    }
    return [selection];
  });
  return { ...draft, selections };
}
