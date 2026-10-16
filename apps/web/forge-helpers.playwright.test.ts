import { test, expect } from "@playwright/test";
import type { Draft } from "./src/coreport.js";
import { DRAFT_TARGET, FIXED_CATALOG_REF } from "./src/coreport.js";
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
import { STEP_IDS } from "./src/neubat-face.js";

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
    draft = setText(draft, STEP_IDS.hostname, "arch-neubat");
    draft = setBool(draft, STEP_IDS.encrypt, true);
    expect(textValue(draft, STEP_IDS.hostname)).toBe("arch-neubat");
    expect(boolValue(draft, STEP_IDS.encrypt)).toBe(true);
  });
});
