import type { Draft } from "./coreport.js";
import { STEP_IDS, findOptionById, isUnverifiedOption } from "./forge-face.js";

export type FaceExclusion = {
  stepId: string;
  reason: "face-only-intent" | "unverified-option";
  optionIds: string[];
};

/**
 * Face-only selections (intent) and unverified requests stay in the UI draft
 * but must not reach a fully-supported RealCore resolve/validate/manifest.
 * Hostname (text) is kept. Disk, filesystem and encrypt selections do not
 * reach the engine: the face no longer authors them.
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
      if (isUnverifiedOption(opt)) {
        exclusions.push({
          stepId: selection.stepId,
          reason: "unverified-option",
          optionIds: [value.optionId],
        });
      }
    } else if (value.kind === "multiple") {
      const unverifiedIds = value.optionIds.filter((id) =>
        isUnverifiedOption(findOptionById(id)),
      );
      if (unverifiedIds.length > 0) {
        exclusions.push({
          stepId: selection.stepId,
          reason: "unverified-option",
          optionIds: unverifiedIds,
        });
      }
    }
  }
  return exclusions;
}

const DISK_STEPS = new Set<string>([
  STEP_IDS.filesystem,
  STEP_IDS.encrypt,
]);

/** Drop disk plan and disk selections. Imported drafts do not reopen that step. */
export function withoutDiskPlan(draft: Draft): Draft {
  const selections = draft.selections.filter(
    (selection) => !DISK_STEPS.has(selection.stepId),
  );
  const next: Draft = { ...draft, selections };
  delete next.storagePlan;
  delete next.encryptionPolicy;
  delete next.inheritedEncryptIntent;
  return next;
}

export function draftForCore(draft: Draft): Draft {
  const stripped = withoutDiskPlan(draft);
  const selections = stripped.selections.flatMap((selection) => {
    if (selection.stepId === STEP_IDS.intent) return [];
    const value = selection.value;
    if (value.kind === "single") {
      const opt = findOptionById(value.optionId);
      if (isUnverifiedOption(opt)) return [];
      return [selection];
    }
    if (value.kind === "multiple") {
      const optionIds = value.optionIds.filter(
        (id) => !isUnverifiedOption(findOptionById(id)),
      );
      if (optionIds.length === 0) return [];
      return [
        { ...selection, value: { kind: "multiple" as const, optionIds } },
      ];
    }
    return [selection];
  });
  return { ...stripped, selections };
}
