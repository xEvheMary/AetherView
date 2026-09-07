<script setup lang="ts">
import Clock from "../widget/clock.vue";
import GeneralMetrics from "../widget/generalMetrics.vue";
import TopProcess from "../widget/topProcess.vue";
</script>
<template>
  <div class="aether-scroll aether-content">
    <Clock />
    <!-- <Placeholder /> -->
    <GeneralMetrics />
    <TopProcess />
  </div>
</template>
<style scoped>
.aether-content {
  height: 100%;
  min-height: 0;
  overflow-x: hidden;
  overflow-y: scroll;
  padding: 16px;
  box-sizing: border-box;
}
</style>
