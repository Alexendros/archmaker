import { RAIL_STEPS } from "./forge-face.js";
import type { StepProgress } from "./step-progress.js";

type Props = {
  step: number;
  progress: StepProgress[];
  onSelect: (index: number) => void;
  /** Sin draft listo: navegación inerte (CSP/boot gate). */
  disabled?: boolean;
};

/** Stepper horizontal cuando el rail lateral se oculta. */
export function ForgeStepper({ step, progress, onSelect, disabled }: Props) {
  return (
    <nav
      className="forge-stepper"
      aria-label="Pasos del configurador (compacto)"
      id="step-stepper"
      aria-disabled={disabled || undefined}
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
                disabled={disabled}
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
