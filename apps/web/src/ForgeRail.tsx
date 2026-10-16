import { RAIL_STEPS } from "./neubat-face.js";

type Props = {
  step: number;
  simpleMode: boolean;
  onSelect: (index: number) => void;
};

export function ForgeRail({ step, simpleMode, onSelect }: Props) {
  return (
    <nav
      className="forge-rail"
      aria-label="Pasos del configurador"
      id="step-rail"
    >
      <p className="forge-rail-meta">
        Modo: {simpleMode ? "Sencillo" : "Avanzado"}
      </p>
      <ol>
        {RAIL_STEPS.map((s, i) => {
          const active = step === i;
          const done = step > i;
          const hint = simpleMode ? s.hintSimple : s.hintAdvanced;
          return (
            <li key={s.id}>
              <button
                type="button"
                className={`forge-rail-item${active ? " is-current" : ""}${done ? " is-done" : ""}`}
                aria-current={active ? "step" : undefined}
                onClick={() => onSelect(i)}
              >
                <span className="forge-rail-num" aria-hidden="true">
                  {done ? "✓" : i + 1}
                </span>
                <span>
                  <span className="forge-rail-title">{s.title}</span>
                  <span className="forge-rail-hint">{hint}</span>
                </span>
              </button>
            </li>
          );
        })}
      </ol>
    </nav>
  );
}
