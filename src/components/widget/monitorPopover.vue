<script setup lang="ts">
import { ref } from "vue";
import Sparkline from "@/components/widget/sparkline.vue";
import type {
  TargetMonitor as Monitor,
  MonitorStatus as Status,
} from "@/models/monitors";
const monitorPopoverRef = ref<HTMLElement | null>(null);
defineProps<{
  target: Monitor | null;
  status: Status | null;
  top: number;
  left: number;
}>();
defineExpose({
  monitorPopoverRef,
});
</script>
<template>
  <Teleport to="body">
    <div
      class="custom-popover monitor-popover"
      ref="monitorPopoverRef"
      :style="{ top: `${top}px`, left: `${left}px` }"
    >
      <div class="monitor-popover-header">
        {{ target?.name }} ({{ status?.response_time_ms }}ms)
      </div>
      <div class="monitor-popover-content">
        <p>Method: {{ target?.method }}</p>
        <p class="line-clamp-2 mb-1">
          Last Error: {{ status?.last_error ?? "N/A" }}
        </p>
        <sparkline :data="status?.history ?? []" />
      </div>
    </div>
  </Teleport>
</template>
<style scoped>
.monitor-popover {
  max-width: 85%;
  background: rgba(20, 20, 20, 0.9);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  & .monitor-popover-header {
    font-weight: 600;
    font-size: 1rem;
    margin-bottom: 0.5rem;
  }
}
.monitor-popover-content p {
  margin: 0.2rem 0;
  text-wrap: calc(100% - 2rem);
}
</style>
