import { useEffect, type RefObject } from "react";

const FOCUSABLE =
  'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

function listFocusable(container: HTMLElement): HTMLElement[] {
  return Array.from(container.querySelectorAll<HTMLElement>(FOCUSABLE)).filter(
    (el) => el.offsetParent !== null || el === document.activeElement,
  );
}

/**
 * Foco inicial, Tab cíclico, Escape, fondo inert y restauración al disparador (H15).
 */
export function useDialogFocusTrap(
  containerRef: RefObject<HTMLElement | null>,
  onClose: () => void,
): void {
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    const previouslyFocused =
      document.activeElement instanceof HTMLElement
        ? document.activeElement
        : null;

    const inertTargets = [
      document.getElementById("main"),
      document.getElementById("inspector"),
      document.querySelector<HTMLElement>(".forge-header"),
      document.querySelector<HTMLElement>(".forge-rail"),
      document.querySelector<HTMLElement>(".forge-stepper"),
    ].filter((el): el is HTMLElement => Boolean(el));

    for (const el of inertTargets) {
      el.setAttribute("inert", "");
      el.setAttribute("aria-hidden", "true");
    }

    const focusables = listFocusable(container);
    (focusables[0] ?? container).focus();

    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        onClose();
        return;
      }
      if (event.key !== "Tab") return;
      const list = listFocusable(container);
      if (list.length === 0) {
        event.preventDefault();
        return;
      }
      const first = list[0]!;
      const last = list[list.length - 1]!;
      if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last.focus();
      } else if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first.focus();
      }
    };

    document.addEventListener("keydown", onKeyDown, true);
    return () => {
      document.removeEventListener("keydown", onKeyDown, true);
      for (const el of inertTargets) {
        el.removeAttribute("inert");
        el.removeAttribute("aria-hidden");
      }
      previouslyFocused?.focus();
    };
  }, [containerRef, onClose]);
}
