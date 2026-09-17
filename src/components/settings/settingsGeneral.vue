<script setup lang="ts">
import { ref, watch } from "vue";
import type { GeneralSettings } from "@/models/settings";
import { useTooltip } from "@/composables/useTooltip";

const props = defineProps<{ content: GeneralSettings }>();
const emit = defineEmits<{
  (e: "submit", value: any): void;
}>();
const targetMonitor = ref<GeneralSettings | null>(null);
const tooltipRef = ref<HTMLElement | null>(null);
useTooltip(tooltipRef, { placement: "right" });
watch(
  () => props.content,
  (newValue) => {
    targetMonitor.value = newValue;
    console.log("targetMonitor updated:", targetMonitor.value);
  },
  { immediate: true },
);
</script>
<template>
  <div class="settings-general">
    <h5>General</h5>
    <p class="text-secondary">
      General application settings saved upon closing.
    </p>
    <div v-if="targetMonitor" class="settings-area aether-scroll">
      <h6>
        Live Settings
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
        <div class="form-floating">
          <input
            type="text"
            class="form-control"
            id="floatingProfileDisabled"
            placeholder="-"
            v-model="targetMonitor.active_profile"
            disabled
          />
          <label for="floatingProfileDisabled">Active Profile</label>
        </div>
      </div>
      <h6 class="mt-2">App Configuration</h6>
      <div class="settings-content">
        <div class="form-check">
          <input
            class="form-check-input"
            type="checkbox"
            v-model="targetMonitor.run_on_startup"
            id="checkStartOnStartup"
          />
          <label class="form-check-label" for="checkStartOnStartup">
            Run on Startup
          </label>
        </div>
        <div class="form-check">
          <input
            class="form-check-input"
            type="checkbox"
            v-model="targetMonitor.close_on_exit"
            id="closeOnExit"
          />
          <label class="form-check-label" for="closeOnExit">
            Close on Exit
          </label>
        </div>
      </div>
    </div>
    <div v-else>
      <div class="d-flex justify-content-center">
        <div class="spinner-border" role="status">
          <span class="visually-hidden">Loading...</span>
        </div>
      </div>
    </div>
    <div class="settings-footer">
      <button class="btn btn-primary" @click="emit('submit', targetMonitor)">
        Save
      </button>
    </div>
  </div>
</template>
<style scoped>
.text-secondary {
  line-height: 1;
}
.settings-general {
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
.form-floating .form-control:disabled {
  background-color: var(--av-input-bg-disabled);
}
@container settingsPane (max-width: 260px) {
  .settings-content {
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }
}
</style>
