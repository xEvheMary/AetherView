<script setup lang="ts">
import { onMounted, onUnmounted, ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { MonitorState, MonitorStatus } from "../../models/monitors";
import MonitorCard from "./monitorCard.vue";
import MonitorPopover from "./monitorPopover.vue";
import { usePopoverPosition } from "@/composables/usePopoverPosition.ts";

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
const monitorPopoverRef = ref<InstanceType<typeof MonitorPopover> | null>(null);
const { x, y, updatePosition } = usePopoverPosition();
async function showPopover(target: String, event: MouseEvent) {
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
</template>

<style scoped>
.monitor-widget {
  width: 100%;
  height: 100%;
  padding-bottom: 0.5rem;
  margin-bottom: 0.5rem;
}
</style>
