import type { Draft } from "./coreport.js";
import { STEP_IDS, findOptionById } from "./neubat-face.js";

/**
 * Face-only selections (intent) and gap packages stay in the UI draft but must
 * not reach RealCore resolve/validate/manifest (AM-RES-001).
 */
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
