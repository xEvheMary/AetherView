<script setup lang="ts">
import { onMounted, onUnmounted, ref, nextTick } from "vue";
import { useFormType } from "@/composables/useFormType.ts";
import { useProfile } from "@/composables/useProfile.ts";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { MonitorState, MonitorStatus } from "../../models/monitors";
import MonitorCard from "./monitorCard.vue";
import MonitorPopover from "./monitorPopover.vue";
import { usePopoverPosition } from "@/composables/usePopoverPosition.ts";
import ContextPopover from "./contextPopover.vue";

const { switchForm } = useFormType();
const { switchProfile } = useProfile();
const monitorState = ref<MonitorState | null>(null);
let unsubscribe: (() => void) | undefined;
onMounted(async () => {
  monitorState.value = await invoke("get_monitor_state");
  unsubscribe = await listen("monitor-status-update", (event) => {
    const update = event.payload;
    if (monitorState.value) {
      if (Array.isArray(update)) {
        update.forEach((status: MonitorStatus) => {
          monitorState.value!.statuses[status.id] = status;
        });
      }
    }
  });
});
onUnmounted(() => {
  unsubscribe?.();
});

const hoveredTarget = ref<String | null>(null);
const clickedTarget = ref<String | null>(null);
const monitorPopoverRef = ref<InstanceType<typeof MonitorPopover> | null>(null);
const contextPopoverRef = ref<InstanceType<typeof ContextPopover> | null>(null);
const { x, y, updatePosition } = usePopoverPosition();
const {
  x: x2,
  y: y2,
  updatePosition: updateContextMenuPosition,
} = usePopoverPosition();
async function showPopover(target: String, event: MouseEvent) {
  // clear other popovers
  clickedTarget.value = null;
  //
  const element = event.currentTarget as HTMLElement;
  hoveredTarget.value = target;
  await nextTick();
  const el = (monitorPopoverRef.value as any)?.monitorPopoverRef as
    | HTMLElement
    | undefined;
  const width = el?.offsetWidth;
  const height = el?.offsetHeight;
  updatePosition(element, width, height, 2);
}
function hidePopover(event: MouseEvent) {
  hoveredTarget.value = null;
  const el = event.currentTarget as HTMLElement;
  if (el) {
    el.style.backgroundColor = "";
  }
}
async function openContextMenu(target: String, event: MouseEvent) {
  // Clear other context menus
  hoveredTarget.value = null;
  //
  const element = event.currentTarget as HTMLElement;
  clickedTarget.value = target;
  await nextTick();
  const el = (contextPopoverRef.value as any)?.popoverRef as
    | HTMLElement
    | undefined;
  const width = el?.offsetWidth;
  const height = el?.offsetHeight;
  updateContextMenuPosition(element, width, height, 2, "right");
}
onMounted(() => {
  document.addEventListener("click", () => {
    clickedTarget.value = null;
  });
});
function contextEdit(target?: String | null) {
  const targetData =
    monitorState?.value?.targets.find((t) => t.id === target) ?? null;
  if (targetData) {
    switchForm("monitor", targetData);
    switchProfile("form");
  }
}
</script>

<template>
  <div class="monitor-widget">
    <MonitorCard
      v-for="target in monitorState?.targets"
      :key="target.id"
      :target="target"
      :status="monitorState?.statuses[target.id] ?? null"
      @mouseenter="showPopover(target.id, $event)"
      @mouseleave="hidePopover($event)"
      @contextmenu.prevent="openContextMenu(target.id, $event)"
    />
  </div>
  <MonitorPopover
    v-if="hoveredTarget"
    ref="monitorPopoverRef"
    :target="monitorState?.targets.find((t) => t.id === hoveredTarget) ?? null"
    :status="
      hoveredTarget
        ? (monitorState?.statuses[hoveredTarget?.toString()] ?? null)
        : null
    "
    :top="y"
    :left="x"
  />
  <ContextPopover
    v-if="clickedTarget"
    ref="contextPopoverRef"
    :top="y2"
    :left="x2"
    :enabled="true"
    :editTarget="() => contextEdit(clickedTarget)"
    :toggleTarget="() => console.log(`toggle ${clickedTarget}`)"
    :deleteTarget="() => console.log(`delete ${clickedTarget}`)"
  />
</template>

<style scoped>
.monitor-widget {
  width: 100%;
  height: auto;
  padding-bottom: 0.5rem;
  margin-bottom: 0.5rem;
}
</style>
