<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { GeneralMetric } from "../../models/general_metric";
import { formatSpeed } from "../../utils/format";
const props = defineProps<{
  minimal?: boolean;
}>();
const metrics = ref<GeneralMetric>({
  cpu_usage: 0,
  memory_usage: 0,
  disk_usage: 0,
  download_speed: 0,
  upload_speed: 0,
});
onMounted(async () => {
  metrics.value = await invoke("get_system_metrics");
});
setInterval(async () => {
  metrics.value = await invoke("get_system_metrics");
}, 5000);
function formatMetrics(metrics: number): string {
  if (props.minimal) {
    return metrics.toFixed(1);
  }
  return metrics.toFixed(2);
}
</script>
<template>
  <div class="general-metrics-widget">
    <div
      v-if="metrics"
      :class="{
        'metrics-container': !props.minimal,
        'metrics-container-minimal': props.minimal,
      }"
    >
      <div>
        <span v-if="!props.minimal">CPU Usage:</span><span v-else>CPU: </span
        ><span>{{ formatMetrics(metrics.cpu_usage) }} %</span>
      </div>
      <div>
        <span v-if="!props.minimal">Memory Usage:</span><span v-else>Mem: </span
        ><span>{{ formatMetrics(metrics.memory_usage) }} %</span>
      </div>
      <div>
        <span v-if="!props.minimal">Disk Activity:</span
        ><span v-else>Disk: </span
        ><span>{{ formatMetrics(metrics.disk_usage) }} %</span>
      </div>
      <div v-if="!!props.minimal">
        <i class="bi bi-globe mx-1"></i>
        <span style="color: red">{{
          formatSpeed(metrics.download_speed)
        }}</span>
        <span style="color: red">↓</span>
        <span style="color: green">↑</span>
        <span style="color: green">{{
          formatSpeed(metrics.upload_speed)
        }}</span>
      </div>
    </div>
    <p v-else>Loading metrics...</p>
  </div>
</template>

<style scoped>
.general-metrics-widget {
  width: 100%;
  background-color: transparent;
  padding-bottom: 0.5rem;
  margin-bottom: 0.5rem;
}
.metrics-container div {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.3rem;
  span {
    white-space: nowrap;
    overflow: hidden;
  }
  span:first-child {
    text-overflow: ellipsis;
  }
}
.metrics-container-minimal {
  display: flex;
  justify-content: space-between;
  span {
    white-space: nowrap;
    overflow: hidden;
  }
  span:first-child {
    text-overflow: ellipsis;
  }
}
</style>
