/**
 * Deterministic CorePort for UI development. No rules, no domain digests, no disk.
 */
import type {
  Artifact,
  BuildManifestInput,
  CorePort,
  CreateDraftInput,
  Draft,
  ExportArtifactInput,
  ImportDraftInput,
  LoadCatalogInput,
  Manifest,
  ResolveDraftInput,
  ResolveResult,
  ResolvedSelection,
  Result,
  SaveDraftInput,
  Selection,
  ValidateDraftInput,
  ValidationResult,
} from "./coreport.js";
import { typedError } from "./coreport.js";
import {
  BOOTLOADERS,
  ENVIRONMENTS,
  FILESYSTEMS,
  GLOSSARY,
  INITIAL_APP_OPTION_IDS,
  INITIAL_SINGLE,
  INTENTS,
  KERNELS,
  LAYERS,
  STEP_IDS,
} from "./neubat-face.js";

const FIXED_DRAFT_ID = "0d5e6b1a-1111-4a2b-9c3d-111111111111";
const FIXED_DIGEST =
  "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

function digest(value = FIXED_DIGEST) {
  return { algorithm: "sha256" as const, value, kind: "content" as const };
}

function initialSelections(): Selection[] {
  return [
    {
      stepId: STEP_IDS.intent,
      value: { kind: "single", optionId: INITIAL_SINGLE.intent },
    },
    {
      stepId: STEP_IDS.kernel,
      value: { kind: "single", optionId: INITIAL_SINGLE.kernel },
    },
    {
      stepId: STEP_IDS.boot,
      value: { kind: "single", optionId: INITIAL_SINGLE.boot },
    },
    {
      stepId: STEP_IDS.filesystem,
      value: { kind: "single", optionId: INITIAL_SINGLE.filesystem },
    },
    {
      stepId: STEP_IDS.compositor,
      value: { kind: "single", optionId: INITIAL_SINGLE.compositor },
    },
    {
      stepId: STEP_IDS.apps,
      value: {
        kind: "multiple",
        optionIds: [...INITIAL_APP_OPTION_IDS],
      },
    },
  ];
}

function faceCatalog() {
  return {
    namespace: "archmaker.core",
    id: "minimal",
    version: "0.1.0",
    documentVersion: "vNext",
    schemaVersion: "1",
    face: "neubat-forge",
    glossary: GLOSSARY,
    steps: [
      {
        id: STEP_IDS.intent,
        title: "Intención",
        mode: "single",
        options: INTENTS.map((o) => ({
          id: o.optionId,
          title: o.title,
          description: o.description,
        })),
      },
      {
        id: STEP_IDS.kernel,
        title: "Kernel",
        mode: "single",
        options: KERNELS.map((o) => ({
          id: o.optionId,
          title: o.title,
          description: o.description,
          gap: o.gap ?? false,
        })),
      },
      {
        id: STEP_IDS.boot,
        title: "Bootloader",
        mode: "single",
        options: BOOTLOADERS.map((o) => ({
          id: o.optionId,
          title: o.title,
          description: o.description,
          gap: o.gap ?? false,
        })),
      },
      {
        id: STEP_IDS.filesystem,
        title: "Filesystem",
        mode: "single",
        options: FILESYSTEMS.map((o) => ({
          id: o.optionId,
          title: o.title,
          description: o.description,
        })),
      },
      {
        id: STEP_IDS.compositor,
        title: "Entorno",
        mode: "single",
        options: ENVIRONMENTS.map((o) => ({
          id: o.optionId,
          title: o.title,
          description: o.description,
          kind: o.kind,
          gap: o.gap ?? false,
        })),
      },
      {
        id: STEP_IDS.apps,
        title: "Apps",
        mode: "multiple",
        layers: LAYERS.map((l) => ({
          id: l.id,
          title: l.title,
          simple: l.simple,
          options: l.packages.map((p) => ({
            id: p.optionId,
            title: p.title,
            description: p.description,
            gap: p.gap ?? false,
          })),
        })),
      },
    ],
  };
}

function effectiveFrom(draft: Draft): ResolvedSelection[] {
  const out: ResolvedSelection[] = [];
  for (const s of draft.selections) {
    if (s.value.kind === "single") {
      out.push({
        stepId: s.stepId,
        optionId: s.value.optionId,
        value: s.value,
        origin: "manual",
        locked: false,
      });
    } else if (s.value.kind === "multiple") {
      for (const optionId of s.value.optionIds) {
        out.push({
          stepId: s.stepId,
          optionId,
          value: { kind: "single", optionId },
          origin: "manual",
          locked: false,
        });
      }
    } else if (s.value.kind === "text") {
      out.push({
        stepId: s.stepId,
        optionId: `${s.stepId}:text`,
        value: s.value,
        origin: "manual",
        locked: false,
      });
    } else if (s.value.kind === "boolean") {
      out.push({
        stepId: s.stepId,
        optionId: `${s.stepId}:${s.value.value}`,
        value: s.value,
        origin: "manual",
        locked: false,
      });
    }
  }
  return out.sort((a, b) =>
    a.stepId === b.stepId
      ? a.optionId.localeCompare(b.optionId)
      : a.stepId.localeCompare(b.stepId),
  );
}

export class MockCorePort implements CorePort {
  private drafts = new Map<string, Draft>();

  async createDraft(input: CreateDraftInput): Promise<Result<Draft>> {
    if (input.catalogRef.id !== "minimal") {
      return {
        ok: false,
        error: typedError(
          "AM-CAT-001",
          "mock-core",
          "error.catalog.unresolvable",
        ),
      };
    }
    if (input.targetRef.id !== "arch-x86_64") {
      return {
        ok: false,
        error: typedError(
          "AM-TGT-001",
          "mock-core",
          "error.target.unsupported",
        ),
      };
    }
    const id = input.draftId ?? FIXED_DRAFT_ID;
    const draft: Draft = {
      id,
      schemaVersion: "1",
      documentVersion: "vNext",
      revision: 0,
      catalogRef: input.catalogRef,
      targetRef: input.targetRef,
      selections: initialSelections(),
    };
    this.drafts.set(id, draft);
    return { ok: true, value: { ...draft } };
  }

  async loadCatalog(_input: LoadCatalogInput): Promise<Result<unknown>> {
    return { ok: true, value: faceCatalog() };
  }

  async saveDraft(input: SaveDraftInput): Promise<Result<Draft>> {
    const stored = this.drafts.get(input.draft.id);
    if (!stored || stored.revision !== input.expectedRevision) {
      return {
        ok: false,
        error: typedError(
          "AM-DOC-002",
          "mock-core",
          "error.draft.revisionConflict",
        ),
      };
    }
    const next = { ...input.draft, revision: stored.revision + 1 };
    this.drafts.set(next.id, next);
    return { ok: true, value: { ...next } };
  }

  async resolveDraft(input: ResolveDraftInput): Promise<Result<ResolveResult>> {
    const effective = effectiveFrom(input.draft);
    return {
      ok: true,
      value: {
        resolutionDigest: digest(),
        effectiveSelections: effective,
        providedCapabilities: effective.length ? [{ id: "fs.ext4" }] : [],
        requiredCapabilities: [],
        conflicts: [],
        diagnostics: [],
      },
    };
  }

  async validateDraft(
    input: ValidateDraftInput,
  ): Promise<Result<ValidationResult>> {
    const diagnostics =
      input.draft.selections.length === 0
        ? [
            {
              code: "AM-RES-001",
              path: "/selections",
              messageKey: "error.resolve.empty",
              source: "mock-core",
              blocking: true,
              severity: "error" as const,
            },
          ]
        : [];
    return {
      ok: true,
      value: {
        diagnostics,
        blocking: diagnostics.some((d) => d.blocking),
        pipelineVersion: "0.1.0",
      },
    };
  }

  async buildManifest(input: BuildManifestInput): Promise<Result<Manifest>> {
    const allowed = ["archmaker-profile", "archinstall-profile", "report"];
    if (!allowed.includes(input.targetRef.id)) {
      return {
        ok: false,
        error: typedError(
          "AM-TGT-001",
          "mock-core",
          "error.target.unsupported",
        ),
      };
    }
    const effective = effectiveFrom(input.draft);
    if (effective.length === 0) {
      return {
        ok: false,
        error: typedError(
          "AM-RES-003",
          "mock-core",
          "error.resolve.unresolved",
        ),
      };
    }
    const manifest: Manifest = {
      documentVersion: input.draft.documentVersion,
      schemaVersion: input.draft.schemaVersion,
      domainLabel: "archmaker:manifest:v1",
      catalogRef: input.catalogRef,
      targetRef: input.targetRef,
      producer: { id: "archmaker-mock", version: "0.1.0" },
      resolutionDigest: digest(),
      effectiveSelections: effective,
      providedCapabilities: [{ id: "fs.ext4" }],
      requiredCapabilities: [],
      conflicts: [],
      contentDigest: digest(
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
      ),
    };
    return { ok: true, value: manifest };
  }

  async exportArtifact(input: ExportArtifactInput): Promise<Result<Artifact>> {
    if (input.manifest.targetRef.id !== input.targetRef.id) {
      return {
        ok: false,
        error: typedError(
          "AM-TGT-001",
          "mock-core",
          "error.target.unsupported",
        ),
      };
    }
    const experimental =
      input.targetRef.id === "archinstall-profile" ? true : undefined;
    let content: unknown;
    let mediaType = "application/json";
    if (input.targetRef.id === "report") {
      mediaType = "text/markdown";
      content = `# ArchMaker report\n\ndomain: archmaker:manifest:v1\n`;
    } else if (input.targetRef.id === "archinstall-profile") {
      content = {
        adapter: "archinstall-profile",
        version: input.targetRef.version,
        manifestRef: { contentDigest: input.manifest.contentDigest },
        effectiveSelections: input.manifest.effectiveSelections,
      };
    } else {
      content = input.manifest;
    }
    const binaryValue =
      input.targetRef.id === "archmaker-profile"
        ? "1111111111111111111111111111111111111111111111111111111111111111"
        : input.targetRef.id === "archinstall-profile"
          ? "2222222222222222222222222222222222222222222222222222222222222222"
          : "3333333333333333333333333333333333333333333333333333333333333333";
    const artifact: Artifact = {
      documentVersion: input.manifest.documentVersion,
      schemaVersion: input.manifest.schemaVersion,
      artifactRef: {
        binaryDigest: {
          algorithm: "sha256",
          value: binaryValue,
          kind: "binary",
        },
        mediaType,
      },
      targetRef: input.targetRef,
      producer: { id: "archmaker-mock", version: "0.1.0" },
      mediaType,
      binaryDigest: {
        algorithm: "sha256",
        value: binaryValue,
        kind: "binary",
      },
      manifestRef: { contentDigest: input.manifest.contentDigest },
      experimental,
      content,
    };
    return { ok: true, value: artifact };
  }

  async importDraft(input: ImportDraftInput): Promise<Result<unknown>> {
    if (input.sourceBytes.length === 0) {
      return {
        ok: false,
        error: typedError("AM-DOC-001", "mock-core", "error.doc.limit"),
      };
    }
    const draft: Draft = {
      id: input.draftId ?? FIXED_DRAFT_ID,
      schemaVersion: "1",
      documentVersion: "vNext",
      revision: 0,
      catalogRef: input.catalogRef,
      targetRef: input.targetRef,
      selections: initialSelections(),
    };
    return {
      ok: true,
      value: {
        draft,
        report: { entries: [], blockingLosses: false },
        changeset: { schemaVersion: "1", operations: [] },
      },
    };
  }
}
