<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { GeneralMetric } from "../../models/general_metric";
import GeneralMetrics from "../widget/generalMetrics.vue";
import Clock from "../widget/clock.vue";
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
}, 1000);
</script>
<template>
  <div class="aether-minimal">
    <GeneralMetrics
      :minimal="true"
      style="margin-bottom: 0; padding-bottom: 0"
    />
    <Clock
      :minimal="true"
      style="margin-bottom: 0; padding-bottom: 0; border: none"
    />
  </div>
</template>
<style scoped>
.aether-minimal {
  height: 100%;
  min-height: 0;
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  padding: 0 0.5rem;
  font-size: 0.8rem;
}
</style>
