<script setup lang="ts">
import { useFormType } from "@/composables/useFormType";
import { useProfile } from "@/composables/useProfile.ts";
import MonitorList from "../widget/monitorList.vue";

const { switchForm } = useFormType();
const { switchProfile } = useProfile();
function openMonitorForm() {
  switchForm("monitor");
  switchProfile("form");
}
</script>
<template>
  <div class="aether-scroll aether-content">
    <div class="monitor-head">
      <p>Monitor</p>
      <button class="btn btn-sm btn-outline-light" @click="openMonitorForm">
        <i class="bi bi-plus"></i>
      </button>
    </div>
    <MonitorList />
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
.monitor-head {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  justify-content: space-between;
  width: 100%;
  margin-bottom: 0.5rem;
  & > p {
    margin: 0;
    padding: 0;
  }
  .btn {
    height: 100%;
  }
}
</style>
