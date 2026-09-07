import { ref } from "vue";

export function usePopoverPosition() {
  const x = ref(0);
  const y = ref(0);

  function updatePosition(
    triggerEl: HTMLElement,
    popoverWidth = 250,
    popoverHeight = 150,
    gap = 8,
  ) {
    const rect = triggerEl.getBoundingClientRect();

    let left = rect.left;
    let top = rect.bottom + gap;
    // Prefer above
    const spaceAbove = rect.top;
    const spaceBelow = window.innerHeight - rect.bottom;

    if (spaceAbove > popoverHeight + gap) {
      top = rect.top - popoverHeight - gap;
    } else if (spaceBelow > popoverHeight + gap) {
      top = rect.bottom + gap;
    }

    // Right overflow
    if (left + popoverWidth > window.innerWidth) {
      left = window.innerWidth - popoverWidth - 10;
    }

    // Left overflow
    if (left < 10) {
      left = 10;
    }

    x.value = left;
    y.value = top;
  }

  return {
    x,
    y,
    updatePosition,
  };
}
