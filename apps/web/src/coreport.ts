/** Frontier types for CorePort v0 + importDraft. Rust owns semantics (ADR-0001). */

export type CoreError = {
  code: string;
  family: string;
  category: string;
  severity: "error" | "warning" | "info";
  messageKey: string;
  source: string;
};

export type Result<T> =
  { ok: true; value: T } | { ok: false; error: CoreError };

export type ContentDigest = {
  algorithm: "sha256";
  value: string;
  kind?: "content";
};

export type BinaryDigest = {
  algorithm: "sha256";
  value: string;
  kind?: "binary";
};

export type CatalogRef = {
  namespace: string;
  id: string;
  version: string;
  digest: ContentDigest;
};

export type TargetRef = { id: string; version: string };

export type SelectionValue =
  | { kind: "single"; optionId: string }
  | { kind: "multiple"; optionIds: string[] }
  | { kind: "boolean"; value: boolean }
  | { kind: "number"; value: number; unit?: string }
  | { kind: "text"; value: string }
  | { kind: "secretRef"; ref: string };

export type Selection = { stepId: string; value: SelectionValue };

/** Draft/manifiesto v2 — espejo de contracts/json-schema/draft-v2-defs.schema.json */
export type StorageModeV2 = "new-disk" | "replace-disk";

export type PlanCertifiabilityStatus =
  | "certifiable"
  | "not-certifiable"
  | "capacity-unverified"
  | "verification-pending";

export type StoragePlan = {
  mode: StorageModeV2;
  firmware: {
    kind: "uefi" | "bios" | "unspecified";
    provenance: "observed" | "declared" | "unknown" | "unavailable";
  };
  partitionTable: "gpt" | "mbr" | "unspecified";
  /** Alineado con partitionTableObservation del schema draft-v2-defs. */
  observedPartitionTable?: {
    status: "observed" | "declared" | "unknown" | "unavailable";
    kind?: "gpt" | "mbr";
  };
  logicalTargetRef?: { id: string };
  declaredCapacityBytes?: number;
  partitions: Array<{
    role: "esp" | "system";
    sizeBytes: number;
    layers: Array<
      | { kind: "fat32"; mountPoint: string }
      | { kind: "luks2"; unlock: "passphrase-requested-at-install" }
      | {
          kind: "filesystem";
          fs: "fat32" | "ext4" | "btrfs";
          mountPoint: string;
        }
    >;
    plannedAction: "create" | "replace" | "keep-declared";
  }>;
  bootArtifactStrategy: "uki-on-esp";
  intentAcknowledged: boolean;
  lossRiskAcknowledged?: boolean;
  planStatus: PlanCertifiabilityStatus;
};

export type EncryptionPolicy = {
  kind: "none" | "luks2";
  unlock?: "passphrase-requested-at-install";
  coversMountPoint?: string;
  initramfsNote?: string;
};

export type BrowserPolicy = {
  selectedBrowserIds: string[];
  defaultBrowserId?: string | null;
};

export type VerificationStatuses = {
  planValidationStatus: "not-validated" | "valid" | "invalid" | "pending";
  hardwareVerificationStatus:
    "not-verified" | "verified" | "pending" | "unavailable";
  bootCompatibilityStatus:
    "unknown" | "compatible" | "conditional" | "incompatible";
  capacityVerificationStatus:
    "unverified" | "sufficient" | "insufficient" | "unknown";
  executionStatus: "not-executed";
};

export type Draft = {
  id: string;
  schemaVersion: string;
  documentVersion: string;
  revision: number;
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  selections: Selection[];
  contentDigest?: ContentDigest;
  storagePlan?: StoragePlan;
  encryptionPolicy?: EncryptionPolicy;
  browserPolicy?: BrowserPolicy;
  verification?: VerificationStatuses;
  inheritedEncryptIntent?: boolean;
  legacyMetadata?: Record<string, unknown>;
};

export type Diagnostic = {
  code: string;
  path: string;
  messageKey: string;
  source: string;
  blocking?: boolean;
  severity?: "error" | "warning" | "info";
};

export type ResolvedSelection = {
  stepId: string;
  optionId: string;
  value: SelectionValue;
  origin: "manual" | "derived";
  locked: boolean;
};

export type ResolveResult = {
  resolutionDigest: ContentDigest;
  effectiveSelections: ResolvedSelection[];
  providedCapabilities: { id: string }[];
  requiredCapabilities: { id: string }[];
  conflicts: unknown[];
  diagnostics: Diagnostic[];
};

export type ValidationResult = {
  diagnostics: Diagnostic[];
  blocking: boolean;
  pipelineVersion: string;
};

export type Manifest = {
  documentVersion: string;
  schemaVersion: string;
  domainLabel: string;
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  producer: { id: string; version: string };
  resolutionDigest: ContentDigest;
  effectiveSelections: ResolvedSelection[];
  providedCapabilities: { id: string }[];
  requiredCapabilities: { id: string }[];
  conflicts: unknown[];
  contentDigest: ContentDigest;
  storagePlan?: StoragePlan;
  encryptionPolicy?: EncryptionPolicy;
  browserPolicy?: BrowserPolicy;
  verification?: VerificationStatuses;
};

export type Artifact = {
  documentVersion: string;
  schemaVersion: string;
  artifactRef: { binaryDigest: BinaryDigest; mediaType: string };
  targetRef: TargetRef;
  producer: { id: string; version: string };
  mediaType: string;
  binaryDigest: BinaryDigest;
  manifestRef: { contentDigest: ContentDigest };
  experimental?: boolean;
  content?: unknown;
  /** Exact renderer bytes; SHA-256 of these equals binaryDigest. */
  bytes?: number[];
};

export type CreateDraftInput = {
  draftId?: string;
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  requestId?: string;
};

export type LoadCatalogInput = {
  source: "embedded" | "file";
  catalogRef?: CatalogRef;
  requestId?: string;
};

export type SaveDraftInput = {
  draft: Draft;
  expectedRevision: number;
  requestId?: string;
};

export type OpenDraftInput = {
  draftId: string;
  seed?: Draft;
  requestId?: string;
};

export type ResolveDraftInput = {
  draft: Draft;
  catalogRef: CatalogRef;
  requestId?: string;
};

export type ValidateDraftInput = {
  draft: Draft;
  catalogRef: CatalogRef;
  targetRef?: TargetRef;
  requestId?: string;
};

export type BuildManifestInput = {
  draft: Draft;
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  expectedResolutionDigest?: ContentDigest;
  requestId?: string;
};

export type ExportArtifactInput = {
  manifest: Manifest;
  targetRef: TargetRef;
  /** Opaque destination handle token; never an arbitrary filesystem path from React. */
  destination: { token: string };
  requestId?: string;
};

export type ImportDraftInput = {
  sourceBytes: number[];
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  draftId?: string;
  confirmBlockingLosses: boolean;
  requestId?: string;
};

export interface CorePort {
  createDraft(input: CreateDraftInput): Promise<Result<Draft>>;
  loadCatalog(input: LoadCatalogInput): Promise<Result<unknown>>;
  saveDraft(input: SaveDraftInput): Promise<Result<Draft>>;
  openDraft(input: OpenDraftInput): Promise<Result<Draft>>;
  resolveDraft(input: ResolveDraftInput): Promise<Result<ResolveResult>>;
  validateDraft(input: ValidateDraftInput): Promise<Result<ValidationResult>>;
  buildManifest(input: BuildManifestInput): Promise<Result<Manifest>>;
  exportArtifact(input: ExportArtifactInput): Promise<Result<Artifact>>;
  importDraft(input: ImportDraftInput): Promise<Result<unknown>>;
}

export function typedError(
  code: string,
  source: string,
  messageKey: string,
): CoreError {
  return {
    code,
    family: code.split("-")[1] ?? "DOC",
    category: "contract",
    severity: "error",
    messageKey,
    source,
  };
}

/** Product catalog identity (local-catalog-mvp1). Digest is content digest from Rust. */
export const DESKTOP_CATALOG_REF: CatalogRef = {
  namespace: "archmaker.core",
  id: "desktop",
  version: "1.0.0",
  digest: {
    algorithm: "sha256",
    value: "dc40565079c99039a39e95063233337af58f0f3b80ee9d7c34d415db11983991",
  },
};

/** @deprecated Use DESKTOP_CATALOG_REF */
export const FIXED_CATALOG_REF = DESKTOP_CATALOG_REF;

export const DRAFT_TARGET: TargetRef = {
  id: "arch-x86_64",
  version: "2026.10.0",
};

export function exportTarget(id: string): TargetRef {
  return { id, version: "0.1.0" };
}

export function singleSelection(stepId: string, optionId: string): Selection {
  return { stepId, value: { kind: "single", optionId } };
}

export function withSelection(draft: Draft, sel: Selection): Draft {
  const rest = draft.selections.filter((s) => s.stepId !== sel.stepId);
  return { ...draft, selections: [...rest, sel] };
}
