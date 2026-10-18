import { RAIL_STEPS } from "./forge-face.js";

export type StepProgress =
  | "visited"
  | "complete"
  | "invalid"
  | "pending"
  | "current";

export type StepProgressInput = {
  current: number;
  visited: ReadonlySet<number>;
  invalid: ReadonlySet<number>;
  hasDefaults: boolean;
  reviewReady: boolean;
};

function stepComplete(index: number, input: StepProgressInput): boolean {
  const id = RAIL_STEPS[index]?.id;
  switch (id) {
    case "base":
    case "environment":
    case "apps":
      return input.hasDefaults;
    case "identity-security":
      return input.visited.has(index);
    case "review":
      return input.reviewReady;
    default:
      return false;
  }
}

export function progressForStep(
  index: number,
  input: StepProgressInput,
): StepProgress {
  if (index === input.current) return "current";
  if (input.invalid.has(index)) return "invalid";
  if (stepComplete(index, input)) return "complete";
  if (input.visited.has(index)) return "visited";
  return "pending";
}

export function allStepProgress(input: StepProgressInput): StepProgress[] {
  return RAIL_STEPS.map((_, i) => progressForStep(i, input));
}
