import { test, expect } from "@playwright/test";
import type { Draft } from "./src/coreport.js";
import { DRAFT_TARGET, FIXED_CATALOG_REF } from "./src/coreport.js";
import { draftForCore, listFaceExclusions } from "./src/draft-for-core.js";
import { mockAllowed } from "./src/create-core-port.js";
import {
  bumpGeneration,
  isPreparedCurrent,
  keepResultForGeneration,
  markPrepared,
  shouldIgnoreReentrantAction,
} from "./src/export-generation.js";
import {
  appIds,
  boolValue,
  chooseSingle,
  setBool,
  setText,
  singleId,
  textValue,
  toggleApp,
} from "./src/forge-helpers.js";
import { STEP_IDS } from "./src/forge-face.js";

function emptyDraft(): Draft {
  return {
    id: "draft-test",
    schemaVersion: "0",
    documentVersion: "0",
    revision: 0,
    catalogRef: FIXED_CATALOG_REF,
    targetRef: DRAFT_TARGET,
    selections: [],
  };
}

test.describe("forge-helpers", () => {
  test("chooseSingle replaces the previous option for the same step", () => {
    let draft = emptyDraft();
    draft = chooseSingle(draft, STEP_IDS.boot, "archmaker.boot.grub");
    draft = chooseSingle(draft, STEP_IDS.boot, "archmaker.boot.systemd-boot");
    expect(singleId(draft, STEP_IDS.boot)).toBe("archmaker.boot.systemd-boot");
    expect(
      draft.selections.filter((s) => s.stepId === STEP_IDS.boot),
    ).toHaveLength(1);
  });

  test("toggleApp adds then removes without duplicating", () => {
    let draft = emptyDraft();
    const first = toggleApp(draft, "archmaker.browser.firefox");
    expect(first.added).toBe(true);
    expect(appIds(first.draft)).toEqual(["archmaker.browser.firefox"]);

    const second = toggleApp(first.draft, "archmaker.browser.firefox");
    expect(second.added).toBe(false);
    expect(appIds(second.draft)).toEqual([]);
  });

  test("setText and setBool round-trip face values", () => {
    let draft = emptyDraft();
    draft = setText(draft, STEP_IDS.hostname, "arch-forge");
    draft = setBool(draft, STEP_IDS.encrypt, true);
    expect(textValue(draft, STEP_IDS.hostname)).toBe("arch-forge");
    expect(boolValue(draft, STEP_IDS.encrypt)).toBe(true);
  });
});

test.describe("draftForCore Prove-It", () => {
  test("strips face intent before core resolve (AM-RES-001)", () => {
    let draft = emptyDraft();
    draft = chooseSingle(draft, STEP_IDS.intent, "archmaker.intent.desarrollo");
    draft = chooseSingle(draft, STEP_IDS.boot, "archmaker.boot.systemd-boot");
    expect(singleId(draft, STEP_IDS.intent)).toBe(
      "archmaker.intent.desarrollo",
    );

    const forCore = draftForCore(draft);
    expect(singleId(forCore, STEP_IDS.intent)).toBeUndefined();
    expect(singleId(forCore, STEP_IDS.boot)).toBe(
      "archmaker.boot.systemd-boot",
    );
    expect(forCore.selections.some((s) => s.stepId === STEP_IDS.intent)).toBe(
      false,
    );
  });

  test("strips unverified packages from multiple selections", () => {
    let draft = emptyDraft();
    draft = {
      ...draft,
      selections: [
        {
          stepId: STEP_IDS.apps,
          value: {
            kind: "multiple",
            optionIds: [
              "archmaker.browser.firefox",
              "archmaker.app.grokbot", // unverified in face catalog
            ],
          },
        },
      ],
    };
    const forCore = draftForCore(draft);
    const apps = forCore.selections.find((s) => s.stepId === STEP_IDS.apps);
    expect(apps?.value).toEqual({
      kind: "multiple",
      optionIds: ["archmaker.browser.firefox"],
    });
  });

  test("keeps hostname and encrypt; lists face exclusions", () => {
    let draft = emptyDraft();
    draft = chooseSingle(draft, STEP_IDS.intent, "archmaker.intent.desarrollo");
    draft = setText(draft, STEP_IDS.hostname, "arch-forge");
    draft = setBool(draft, STEP_IDS.encrypt, true);
    draft = {
      ...draft,
      selections: [
        ...draft.selections,
        {
          stepId: STEP_IDS.apps,
          value: {
            kind: "multiple",
            optionIds: ["archmaker.browser.firefox", "archmaker.app.grokbot"],
          },
        },
      ],
    };

    const exclusions = listFaceExclusions(draft);
    expect(exclusions.some((e) => e.reason === "face-only-intent")).toBe(true);
    expect(exclusions.some((e) => e.reason === "unverified-option")).toBe(true);

    const forCore = draftForCore(draft);
    expect(textValue(forCore, STEP_IDS.hostname)).toBe("arch-forge");
    expect(boolValue(forCore, STEP_IDS.encrypt)).toBe(true);
    expect(forCore.selections.some((s) => s.stepId === STEP_IDS.intent)).toBe(
      false,
    );
  });
});

test.describe("export-generation Prove-It", () => {
  test("edit after prepare invalidates reuse of prepared snapshot", () => {
    let gate = { generation: 0, preparedGeneration: null as number | null };
    gate = markPrepared(gate, gate.generation);
    expect(isPreparedCurrent(gate)).toBe(true);

    gate = bumpGeneration(gate);
    expect(isPreparedCurrent(gate)).toBe(false);
    expect(gate.preparedGeneration).toBeNull();
    expect(gate.generation).toBe(1);
  });

  test("F7: generar A, editar B invalida resultado A; exportar usa B", () => {
    let gate = { generation: 1, preparedGeneration: null as number | null };
    gate = markPrepared(gate, gate.generation);
    const generationA = gate.generation;
    expect(isPreparedCurrent(gate)).toBe(true);

    gate = bumpGeneration(gate); // edición B
    expect(keepResultForGeneration(generationA, gate.generation)).toBe(false);
    expect(isPreparedCurrent(gate)).toBe(false);

    gate = markPrepared(gate, gate.generation); // prepare B
    expect(isPreparedCurrent(gate)).toBe(true);
    expect(keepResultForGeneration(gate.generation, gate.generation)).toBe(
      true,
    );
  });

  test("F7: doble clic / reentrada se ignora mientras busy", () => {
    expect(shouldIgnoreReentrantAction(false)).toBe(false);
    expect(shouldIgnoreReentrantAction(true)).toBe(true);
  });

  test("P5: respuesta async antigua no aplica si la generación avanzó", () => {
    // Simula prepare A en vuelo; el usuario edita → generación 2; llega A.
    let currentGeneration = 1;
    const capturedA = currentGeneration;
    currentGeneration = 2; // edición B
    expect(keepResultForGeneration(capturedA, currentGeneration)).toBe(false);
    const capturedB = currentGeneration;
    expect(keepResultForGeneration(capturedB, currentGeneration)).toBe(true);
  });
});

test.describe("create-core-port F7 motor real", () => {
  test("producción nunca admite mock; sin opt-in el mock está cerrado", () => {
    expect(
      mockAllowed({ isProd: true, search: "?mock=1", localStorageMock: "1" }),
    ).toBe(false);
    expect(
      mockAllowed({ isProd: false, search: "", localStorageMock: null }),
    ).toBe(false);
    expect(
      mockAllowed({ isProd: false, search: "?mock=1", localStorageMock: null }),
    ).toBe(true);
    expect(
      mockAllowed({ isProd: false, search: "", localStorageMock: "1" }),
    ).toBe(true);
  });
});

test.describe("storage-plan-face H01/H02", () => {
  test("modo con confirmaciones queda verification-pending, nunca certifiable", async () => {
    const { storagePlanFromIntent, uiStateForIntent, withFaceV2Fields } =
      await import("./src/storage-plan-face.js");
    const intent = {
      mode: "new-disk" as const,
      intentAcknowledged: true,
      lossRiskAcknowledged: false,
      uiState: "proposed" as const,
    };
    expect(uiStateForIntent(intent)).toBe("proposed");
    const plan = storagePlanFromIntent(intent);
    expect(plan?.planStatus).toBe("verification-pending");
    expect(plan?.planStatus).not.toBe("certifiable");

    const draft = withFaceV2Fields(emptyDraft(), intent);
    expect(draft.schemaVersion).toBe("2");
    expect(draft.storagePlan?.mode).toBe("new-disk");
    expect(draft.verification?.executionStatus).toBe("not-executed");
    expect(draft.verification?.planValidationStatus).toBe("not-validated");
  });

  test("reemplazo sin lossRisk queda not-certifiable", async () => {
    const { storagePlanFromIntent } =
      await import("./src/storage-plan-face.js");
    const plan = storagePlanFromIntent({
      mode: "replace-disk",
      intentAcknowledged: true,
      lossRiskAcknowledged: false,
      uiState: "declared",
    });
    expect(plan?.planStatus).toBe("not-certifiable");
  });
});

test.describe("catalog-view H06", () => {
  test("título del catálogo prevalece sobre la constante de cara", async () => {
    const { mergeFaceOptionsFromCatalog, parseCatalogView } =
      await import("./src/catalog-view.js");
    const { KERNELS, STEP_IDS: ids } = await import("./src/forge-face.js");
    const faceLinux = KERNELS.find(
      (k) => k.optionId === "archmaker.kernel.linux",
    );
    expect(faceLinux?.title).toBeTruthy();

    const catalog = parseCatalogView({
      namespace: "archmaker.core",
      id: "desktop",
      version: "test",
      steps: [
        {
          id: ids.kernel,
          title: "Kernel",
          options: [
            {
              id: "archmaker.kernel.linux",
              title: "Linux-FROM-CATALOG-FIXTURE",
              description: "desde catálogo",
            },
            {
              id: "archmaker.kernel.linux-lts",
              title: "LTS-FROM-CATALOG",
            },
          ],
        },
      ],
    });
    expect(catalog).not.toBeNull();
    const merged = mergeFaceOptionsFromCatalog(catalog, ids.kernel, KERNELS);
    expect(merged.map((o) => o.optionId)).toEqual([
      "archmaker.kernel.linux",
      "archmaker.kernel.linux-lts",
    ]);
    expect(merged[0]?.title).toBe("Linux-FROM-CATALOG-FIXTURE");
    expect(merged[0]?.title).not.toBe(faceLinux!.title);
    expect(merged[0]?.description).toBe(faceLinux!.description);
  });
});
