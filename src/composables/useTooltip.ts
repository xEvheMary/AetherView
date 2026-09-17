import { onMounted, onUnmounted, nextTick } from "vue";
import type { Ref } from "vue";
import { Tooltip } from "bootstrap";

type TooltipPlacement = "auto" | "top" | "bottom" | "left" | "right";
export function useTooltip(
  targetRef: Ref<HTMLElement | null>,
  opts: { placement?: TooltipPlacement } = {},
) {
  let inst: Tooltip | null = null;
  onMounted(async () => {
    await nextTick();
    const el = targetRef.value;
    if (!el) return;
    let targetPlacement = opts.placement ?? "auto";
    inst = new Tooltip(el, {
      trigger: "hover focus",
      placement: targetPlacement,
      customClass: "bs-dark",
      container: "body",
      html: false,
    });
  });
  onUnmounted(() => inst?.dispose());
  return { dispose: () => inst?.dispose() };
}
