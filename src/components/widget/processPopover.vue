<script setup lang="ts">
import { ref } from "vue";
import type { ProcessInfo } from "../../models/process_info";
const popoverRef = ref<HTMLElement | null>(null);
defineProps<{
  process: ProcessInfo;
  top: number;
  left: number;
}>();
defineExpose({
  popoverRef,
});
</script>
<template>
  <Teleport to="body">
    <div
      class="process-popover"
      ref="popoverRef"
      :style="{ top: `${top}px`, left: `${left}px` }"
    >
      <div class="process-popover-header">
        {{ process.name }}
      </div>
      <div class="process-popover-content">
        <p>Process Count: {{ process.process_count }}</p>
        <p>CPU Usage: {{ process.cpu_usage }}%</p>
        <p>Memory Usage: {{ process.memory_mb }} MB</p>
      </div>
    </div>
  </Teleport>
</template>
<style scoped>
.process-popover {
  background: rgba(20, 20, 20, 0.9);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  & .process-popover-header {
    font-weight: 600;
    font-size: 1rem;
    margin-bottom: 0.5rem;
  }
}
.process-popover-content p {
  margin: 0.2rem 0;
}
</style>
