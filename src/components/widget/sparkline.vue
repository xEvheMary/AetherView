<script setup lang="ts">
import { computed } from "vue";
const props = defineProps<{
  data: (number | null)[];
}>();
const avg = computed(() => {
  const values = props.data.filter((v): v is number => v !== null);

  if (values.length === 0) return 0;

  return values.reduce((a, b) => a + b, 0) / values.length;
});
function getBarHeight(value: number | null): string {
  if (value === null) {
    return "100%";
  }
  const validValues = props.data.filter((v): v is number => v !== null);
  const max = Math.max(...validValues, avg.value * 3);

  const height = Math.max((value / max) * 100, 10);

  return `${height}%`;
}
function getBarClass(value: number | null) {
  if (value === null) {
    return "bg-danger";
  }

  if (value <= avg.value) {
    return "bg-success";
  }

  if (value <= avg.value * 2) {
    return "bg-warning";
  }

  return "bg-danger";
}
</script>
<template>
  <div class="sparkline-container">
    <div class="history-bars">
      <div
        v-for="(value, index) in props.data"
        :key="index"
        class="history-bar"
        :class="getBarClass(value)"
        :style="{ height: getBarHeight(value) }"
      />
    </div>
  </div>
</template>
<style scoped>
.history-bars {
  height: 40px;
  display: flex;
  align-items: flex-end;
  gap: 0;
}
.history-bar {
  flex: 1;
  min-height: 2px;
  border-left: 1px solid var(--av-border);
}
.sparkline-container {
  width: 100%;
  border: 1px solid var(--av-border);
  border-radius: 4px;
}
</style>
