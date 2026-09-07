<script setup lang="ts">
import { ref, onMounted, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { usePopoverPosition } from "../../composables/usePopoverPosition";
import type { ProcessInfo } from "../../models/process_info";
import ProcessPopover from "./processPopover.vue";

const topProcesses = ref<ProcessInfo[]>([]);

onMounted(async () => {
  topProcesses.value = await invoke("get_top_processes");
});
setInterval(async () => {
  topProcesses.value = await invoke("get_top_processes");
}, 10000);
const hoveredProcess = ref<ProcessInfo | null>(null);
const popoverRef = ref<InstanceType<typeof ProcessPopover> | null>(null);
const { x, y, updatePosition } = usePopoverPosition();
async function showPopover(process: ProcessInfo, event: MouseEvent) {
  const element = event.currentTarget as HTMLElement;
  hoveredProcess.value = process;
  await nextTick();
  const el = (popoverRef.value as any)?.popoverRef as HTMLElement | undefined;
  const width = el?.offsetWidth;
  const height = el?.offsetHeight;
  updatePosition(element, width, height, 2);
}
function hidePopover(event: MouseEvent) {
  hoveredProcess.value = null;
  const element = event.currentTarget as HTMLElement;
  if (element) {
    element.style.backgroundColor = "";
  }
}
</script>
<template>
  <div class="accordion w-100">
    <div class="accordion-item">
      <h2 class="accordion-header">
        <button
          class="accordion-button collapsed"
          type="button"
          data-bs-toggle="collapse"
          data-bs-target="#processPanel-collapse"
          aria-expanded="false"
          aria-controls="processPanel-collapse"
        >
          Top Processes
        </button>
      </h2>
      <div id="processPanel-collapse" class="accordion-collapse collapse">
        <div class="accordion-body">
          <div
            v-for="process in topProcesses"
            :key="process.name"
            class="process-row"
            @mouseenter="showPopover(process, $event)"
            @mouseleave="hidePopover($event)"
          >
            <span>{{ process.name }} ({{ process.process_count }})</span>
          </div>
        </div>
      </div>
    </div>
  </div>
  <ProcessPopover
    v-if="hoveredProcess"
    ref="popoverRef"
    :process="hoveredProcess"
    :top="y"
    :left="x"
  />
</template>
<style scoped>
.accordion {
  background-color: transparent;
}
.accordion-button {
  padding: 0.5rem 1rem;
  &:not(.collapsed),
  &:focus {
    box-shadow: inset 0 -1px 0 rgba(255, 255, 255, 0.1);
  }
  &::after {
    background-image: none;
    content: "⏷";
  }
  &:not(.collapsed)::after {
    background-image: none;
    content: "⏶";
  }
}
.accordion-body {
  padding: 0.5rem 0;
}
.accordion-body li {
  list-style-type: none;
}
.process-row {
  display: flex;
  justify-content: space-between;
  padding: 0.3rem 0.5rem;
  cursor: pointer;
  span {
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
  }
  &:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }
}
</style>
