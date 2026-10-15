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
  ValidateDraftInput,
  ValidationResult,
} from "./coreport.js";
import { typedError } from "./coreport.js";

const FIXED_DRAFT_ID = "0d5e6b1a-1111-4a2b-9c3d-111111111111";
const FIXED_DIGEST =
  "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

function digest(value = FIXED_DIGEST) {
  return { algorithm: "sha256" as const, value, kind: "content" as const };
}

function optionIdOf(draft: Draft, stepId: string): string | undefined {
  const sel = draft.selections.find((s) => s.stepId === stepId);
  if (!sel) return undefined;
  if (sel.value.kind === "single") return sel.value.optionId;
  return undefined;
}

function effectiveFrom(draft: Draft): ResolvedSelection[] {
  return draft.selections
    .filter((s) => s.value.kind === "single")
    .map((s) => ({
      stepId: s.stepId,
      optionId: (s.value as { kind: "single"; optionId: string }).optionId,
      value: s.value,
      origin: "manual" as const,
      locked: false,
    }))
    .sort((a, b) =>
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
      selections: [
        {
          stepId: "archmaker.base.boot",
          value: {
            kind: "single",
            optionId: "archmaker.boot.systemd-boot",
          },
        },
      ],
    };
    this.drafts.set(id, draft);
    return { ok: true, value: { ...draft } };
  }

  async loadCatalog(_input: LoadCatalogInput): Promise<Result<unknown>> {
    return {
      ok: true,
      value: {
        namespace: "archmaker.core",
        id: "minimal",
        version: "0.1.0",
        documentVersion: "vNext",
        schemaVersion: "1",
        steps: [
          {
            id: "archmaker.sys.base",
            title: "Base system",
            mode: "single",
            sections: [
              {
                id: "archmaker.sys.kernel",
                title: "Kernel",
                cardinality: "one",
                options: [{ id: "archmaker.kernel.linux", title: "Linux" }],
              },
            ],
          },
          {
            id: "archmaker.sys.session",
            title: "Session",
            mode: "single",
            sections: [
              {
                id: "archmaker.sys.desktop",
                title: "Desktop",
                cardinality: "one",
                options: [{ id: "archmaker.desktop.gnome", title: "GNOME" }],
              },
            ],
          },
        ],
      },
    };
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
    if (
      optionIdOf(input.draft, "archmaker.sys.base") === undefined &&
      input.draft.selections.length > 0
    ) {
      /* still resolve whatever was selected */
    }
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
      selections: [],
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
