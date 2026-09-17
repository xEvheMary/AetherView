<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type {
  Settings,
  GeneralSettings,
  MonitoringSettings,
  AppearanceSettings,
} from "../../models/settings";
import { useToast } from "../../composables/useToast";
import SettingsGeneral from "./settingsGeneral.vue";
import SettingsMonitor from "./settingsMonitor.vue";
import SettingsAppearance from "./settingsAppearance.vue";
import SettingsAbout from "./settingsAbout.vue";
const { visible, message, variant, showToast, hideToast } = useToast();
const activeTab = ref("general");
const tabs = [
  { name: "general", label: "General" },
  { name: "monitoring", label: "Monitoring" },
  { name: "appearance", label: "Appearance" },
  { name: "about", label: "About" },
];
const settings = ref<Settings | null>(null);
onMounted(async () => {
  settings.value = await invoke("get_settings");
  console.log(settings.value);
});
async function saveSettings(
  update: GeneralSettings | MonitoringSettings | AppearanceSettings,
) {
  if (update.type === "general") {
    settings.value!.general = update;
  } else if (update.type === "monitoring") {
    settings.value!.monitoring = update;
  } else if (update.type === "appearance") {
    settings.value!.appearance = update;
  }
  if (settings.value) {
    try {
      await invoke("store_settings", { settings: settings.value });
      showToast("", "Settings saved successfully", "success", 20000);
    } catch (error) {
      showToast("", "Failed to save settings: ${error}", "danger", 20000);
    }
  }
}
</script>
<template>
  <div class="container-fluid p-3 settings-page">
    <h4 class="mb-3">Settings</h4>
    <div class="row g-3">
      <div class="col-auto">
        <div class="list-group settings-sidebar">
          <button
            v-for="tab in tabs"
            :key="tab.name"
            type="button"
            class="list-group-item list-group-item-action"
            :class="{ active: activeTab === tab.name }"
            @click="activeTab = tab.name"
          >
            {{ tab.label }}
          </button>
        </div>
      </div>

      <div class="col">
        <div class="tab-content">
          <settings-general
            v-if="activeTab === 'general'"
            :content="settings?.general as GeneralSettings"
            @submit="(value) => saveSettings(value)"
          />
          <settings-monitor
            v-else-if="activeTab === 'monitoring'"
            :content="settings?.monitoring"
            @submit="(value) => saveSettings(value)"
          />
          <settings-appearance
            v-else-if="activeTab === 'appearance'"
            :content="settings?.appearance as AppearanceSettings"
          />
          <settings-about v-else-if="activeTab === 'about'" />
        </div>
      </div>
    </div>
  </div>
  <Teleport to="body">
    <div
      v-if="visible"
      class="toast-container position-fixed top-0 end-0 p-3"
      style="z-index: 9999"
    >
      <div
        class="toast show border-0 aether-toast"
        :class="`bg-${variant}`"
        role="alert"
      >
        <div class="d-flex align-items-center px-2">
          <div class="toast-body">
            {{ message }}
          </div>
          <button type="button" class="btn-close" @click="hideToast"></button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
<style scoped>
.settings-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}
.settings-page .row {
  flex: 1 1 auto;
  flex-wrap: nowrap;
  min-height: 0;
  overflow: hidden;
}
.settings-content {
  min-height: 400px;
}
.settings-sidebar .list-group-item {
  color: var(--av-text);
  text-align: left;
  &:hover {
    background-color: transparent;
    border-color: var(--bs-border-color);
    color: var(--bs-primary);
  }
  &.active {
    background-color: var(--av-button-bg-active);
    border-color: var(--bs-border-color);
    color: var(--av-button-text-active);
  }
}
.tab-content {
  min-width: 20%;
  height: 100%;
  container-type: inline-size;
  container-name: settingsPane;
}
.aether-toast {
  --bs-toast-max-width: none;
  width: max-content;
  max-width: 50vw;
}
.aether-toast .toast-body {
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}
</style>
