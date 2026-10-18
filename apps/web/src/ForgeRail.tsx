import { RAIL_STEPS } from "./forge-face.js";
import type { StepProgress } from "./step-progress.js";

type Props = {
  step: number;
  progress: StepProgress[];
  onSelect: (index: number) => void;
  /** Sin draft listo: navegación inerte (CSP/boot gate). */
  disabled?: boolean;
};

function progressLabel(p: StepProgress): string {
  switch (p) {
    case "current":
      return "actual";
    case "complete":
      return "completo";
    case "invalid":
      return "inválido";
    case "visited":
      return "visitado";
    default:
      return "pendiente";
  }
}

function progressMark(p: StepProgress, index: number): string {
  if (p === "complete") return "✓";
  if (p === "invalid") return "!";
  return String(index + 1);
}

export function ForgeRail({ step, progress, onSelect, disabled }: Props) {
  return (
    <nav
      className="forge-rail"
      aria-label="Pasos del configurador"
      id="step-rail"
      aria-disabled={disabled || undefined}
    >
      <ol>
        {RAIL_STEPS.map((s, i) => {
          const status = progress[i] ?? "pending";
          const active = step === i;
          return (
            <li key={s.id}>
              <button
                type="button"
                className={`forge-rail-item is-${status}${active ? " is-current" : ""}`}
                aria-current={active ? "step" : undefined}
                data-step-progress={status}
                disabled={disabled}
                onClick={() => onSelect(i)}
              >
                <span className="forge-rail-num" aria-hidden="true">
                  {progressMark(status, i)}
                </span>
                <span>
                  <span className="forge-rail-title">{s.title}</span>
                  <span className="forge-rail-hint">
                    {s.hint} · {progressLabel(status)}
                  </span>
                </span>
              </button>
            </li>
          );
        })}
      </ol>
    </nav>
  );
}
