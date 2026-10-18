/**
 * Transporte de cara para storagePlan v2.
 * La estructura canónica la rematerializa Rust (`declared_storage_plan_from_intent`)
 * en saveDraft/openDraft; aquí solo se envían modo, confirmaciones y el esqueleto
 * alineado con manual_reference_storage_plan.
 */

import type { Draft, StoragePlan } from "./coreport.js";
import { DEFAULT_BROWSER_ID, STEP_IDS } from "./forge-face.js";
import type { StorageIntent, StorageMode } from "./storage-intent.js";
import { INITIAL_STORAGE_INTENT } from "./storage-intent.js";

const ESP_BYTES = 2 * 1024 * 1024 * 1024;

export function storagePlanFromIntent(
  intent: StorageIntent,
): StoragePlan | undefined {
  if (intent.mode === null) return undefined;
  const incomplete =
    !intent.intentAcknowledged ||
    (intent.mode === "replace-disk" && !intent.lossRiskAcknowledged);
  const action = intent.mode === "replace-disk" ? "replace" : "create";
  return {
    mode: intent.mode,
    firmware: { kind: "uefi", provenance: "declared" },
    partitionTable: "gpt",
    observedPartitionTable: { status: "unknown" },
    partitions: [
      {
        role: "esp",
        sizeBytes: ESP_BYTES,
        layers: [{ kind: "fat32", mountPoint: "/efi" }],
        plannedAction: action,
      },
      {
        role: "system",
        sizeBytes: 0,
        layers: [{ kind: "filesystem", fs: "ext4", mountPoint: "/" }],
        plannedAction: action,
      },
    ],
    bootArtifactStrategy: "uki-on-esp",
    intentAcknowledged: intent.intentAcknowledged,
    lossRiskAcknowledged:
      intent.mode === "replace-disk" ? intent.lossRiskAcknowledged : undefined,
    planStatus: incomplete ? "not-certifiable" : "verification-pending",
  };
}

export function intentFromStoragePlan(
  plan: StoragePlan | undefined,
): StorageIntent {
  if (!plan) return { ...INITIAL_STORAGE_INTENT };
  const mode = plan.mode as StorageMode;
  const intentAcknowledged = plan.intentAcknowledged;
  const lossRiskAcknowledged = plan.lossRiskAcknowledged === true;
  let uiState: StorageIntent["uiState"] = "unspecified";
  if (plan.planStatus === "certifiable") {
    uiState = "validated";
  } else if (plan.planStatus === "verification-pending") {
    uiState = "proposed";
  } else if (mode !== null) {
    uiState = "declared";
  }
  return {
    mode,
    intentAcknowledged,
    lossRiskAcknowledged,
    uiState,
  };
}

export function uiStateForIntent(
  intent: StorageIntent,
): StorageIntent["uiState"] {
  if (intent.mode === null) return "unspecified";
  const incomplete =
    !intent.intentAcknowledged ||
    (intent.mode === "replace-disk" && !intent.lossRiskAcknowledged);
  if (incomplete) return "declared";
  return "proposed";
}

/** Alinea schema v2. No autora plan de disco ni cifrado. */
export function withFaceV2Fields(draft: Draft): Draft {
  const apps = draft.selections.find((s) => s.stepId === STEP_IDS.apps);
  const browserIds =
    apps?.value.kind === "multiple"
      ? apps.value.optionIds.filter((id) => id.startsWith("archmaker.browser."))
      : [];

  const defaultBrowser =
    draft.browserPolicy?.defaultBrowserId &&
    browserIds.includes(draft.browserPolicy.defaultBrowserId)
      ? draft.browserPolicy.defaultBrowserId
      : browserIds.includes(DEFAULT_BROWSER_ID)
        ? DEFAULT_BROWSER_ID
        : (browserIds[0] ?? null);

  const {
    storagePlan: _storagePlan,
    encryptionPolicy: _encryptionPolicy,
    inheritedEncryptIntent: _inheritedEncryptIntent,
    ...rest
  } = draft;

  return {
    ...rest,
    selections: rest.selections.filter(
      (selection) =>
        selection.stepId !== STEP_IDS.filesystem &&
        selection.stepId !== STEP_IDS.encrypt,
    ),
    schemaVersion: "2",
    browserPolicy: {
      selectedBrowserIds: browserIds,
      defaultBrowserId: defaultBrowser,
    },
    verification: {
      planValidationStatus: "not-validated",
      hardwareVerificationStatus: "not-verified",
      bootCompatibilityStatus: "unknown",
      capacityVerificationStatus: "unverified",
      executionStatus: "not-executed",
    },
  };
}
