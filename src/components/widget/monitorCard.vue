<script setup lang="ts">
import type {
  TargetMonitor as Monitor,
  MonitorStatus as Status,
} from "../../models/monitors";
const props = defineProps<{
  target: Monitor;
  status: Status | null;
}>();
</script>
<template>
  <div class="target-card mb-2">
    <div class="left card-section">
      <span>{{ props.target.name }}</span>
      <span class="text-below ellipsis">{{ props.target.endpoint }}</span>
    </div>
    <div class="right card-section">
      <div class="d-flex flex-row align-items-center justify-content-end">
        <span
          class="dot me-1"
          :style="{ backgroundColor: props.status?.healthy ? 'green' : 'red' }"
        ></span>
        <span :style="{ color: props.status?.healthy ? 'green' : 'red' }">{{
          props.status?.healthy ? "Up" : "Down"
        }}</span>
      </div>
      <div
        v-if="props.status?.response_time_ms"
        class="text-below"
        style="align-self: flex-end"
      >
        {{ props.status?.response_time_ms }} ms
      </div>
    </div>
  </div>
</template>
<style scoped>
.target-card {
  width: 100%;
  display: flex;
  flex-direction: row;
  padding: 0.3rem 0.5rem;
  margin-bottom: 0.5rem;
  border: 1px solid #cccccc6b;
  border-radius: 0.3rem;
}
.card-section {
  display: flex;
  flex-direction: column;
  &.left {
    flex: 2;
    max-width: 65%;
  }
  &.right {
    flex: 1;
    font-size: 0.9rem; /* Adjust the font size as needed */
  }
}
.text-below {
  font-size: 0.8rem; /* Adjust the font size as needed */
  color: #c0c0c0; /* Adjust the color as needed */
}
.ellipsis {
  direction: rtl;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.dot {
  height: 0.9rem;
  aspect-ratio: 1 / 1;
  border-radius: 50%;
  display: inline-block;
}
</style>
