<script setup lang="ts">
import { ref, watch } from "vue";
import type { MonitoringSettings } from "@/models/settings";
const props = defineProps<{ content: any }>();
const emit = defineEmits<{
  (e: "submit", value: any): void;
}>();
const target = ref<MonitoringSettings>({} as MonitoringSettings);
watch(
  () => props.content,
  (newVal) => {
    target.value = { ...newVal };
  },
  { immediate: true },
);
</script>
<template>
  <div class="settings-monitor">
    <h5>Monitoring</h5>
    <p class="text-secondary">Global monitoring configuration.</p>
    <div class="settings-area aether-scroll">
      <h6>
        Thread Settings
        <span
          ref="tooltipRef"
          class="ms-1 text-secondary"
          tabindex="0"
          title="How often AetherView checks this monitor."
        >
          <i class="bi bi-info-circle"></i>
        </span>
      </h6>
      <div class="settings-content">
        <!-- Thread Interval Setting -->
        <div class="input-group mb-2">
          <div class="form-floating">
            <input
              type="number"
              class="form-control"
              id="floatingInputInterval"
              placeholder="Thread Interval"
              autocomplete="off"
              v-model="target.engine_tick_seconds"
            />
            <label for="floatingInputInterval">Interval</label>
          </div>
          <span class="input-group-text">secs</span>
        </div>
        <!-- Timeout Threshold Setting -->
        <div class="input-group mb-2">
          <div class="form-floating">
            <input
              type="number"
              class="form-control"
              id="floatingInputTimeout"
              placeholder="Timeout Threshold"
              autocomplete="off"
              v-model="target.slow_response_threshold_ms"
            />
            <label for="floatingInputTimeout">Timeout</label>
          </div>
          <span class="input-group-text">ms</span>
        </div>
        <!-- Logging Enabled Setting -->
        <div class="d-flex align-items-center">
          <div class="form-check">
            <input
              class="form-check-input"
              type="checkbox"
              v-model="target.logging_enabled"
              id="checkLoggingEnable"
            />
            <label class="form-check-label" for="checkLoggingEnable">
              Enable Logging
            </label>
          </div>
        </div>
      </div>
    </div>
    <div class="settings-footer">
      <button class="btn btn-primary" @click="emit('submit', target)">
        Save
      </button>
    </div>
  </div>
</template>
<style scoped>
.text-secondary {
  line-height: 1;
}
.settings-monitor {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}
.settings-area {
  display: flex;
  flex: 1 1 auto;
  flex-direction: column;
  overflow-y: auto;
  min-height: 0;
  padding-right: 0.3rem;
}
.settings-content {
  display: grid;
  grid-template-columns: repeat(2, minmax(100px, 1fr));
  gap: 0.5rem;
}
.settings-footer {
  display: flex;
  flex-direction: row;
  justify-content: flex-end;
  flex-shrink: 0;
  margin-top: 0.5rem;
}
.form-floating-sm > .form-control,
.form-floating-sm > .form-select {
  height: calc(2.2rem + 2px);
  min-height: calc(2.2rem + 2px);
  padding: 0.5rem 0.5rem;
  font-size: 0.8rem;
  line-height: 1.1;
}
.form-floating {
  color: var(--av-text);
  & > label {
    color: var(--av-text);
  }
}
.form-floating .form-control:disabled {
  background-color: var(--av-input-bg-disabled);
}
.input-group-text {
  background-color: var(--av-background);
  color: var(--av-text);
}
@container settingsPane (max-width: 260px) {
  .settings-content {
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }
}
</style>
