import { RAIL_STEPS } from "./forge-face.js";
import type { StepProgress } from "./step-progress.js";

type Props = {
  step: number;
  progress: StepProgress[];
  onSelect: (index: number) => void;
};

/** Stepper horizontal cuando el rail lateral se oculta. */
export function ForgeStepper({ step, progress, onSelect }: Props) {
  return (
    <nav
      className="forge-stepper"
      aria-label="Pasos del configurador (compacto)"
      id="step-stepper"
    >
      <ol className="forge-stepper-list">
        {RAIL_STEPS.map((s, i) => {
          const status = progress[i] ?? "pending";
          const active = step === i;
          return (
            <li key={s.id}>
              <button
                type="button"
                className={`forge-stepper-item is-${status}${active ? " is-current" : ""}`}
                aria-current={active ? "step" : undefined}
                data-step-progress={status}
                title={s.title}
                onClick={() => onSelect(i)}
              >
                <span className="forge-stepper-num" aria-hidden="true">
                  {status === "complete" ? "✓" : i + 1}
                </span>
                <span className="forge-stepper-title">{s.title}</span>
              </button>
            </li>
          );
        })}
      </ol>
    </nav>
  );
}
