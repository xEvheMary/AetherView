<script setup lang="ts">
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useAppearance } from "./composables/useAppearance.ts";
import SettingsPage from "./components/settings/settingsPage.vue";
const { setTheme } = useAppearance();
onMounted(async () => {
  const theme = await invoke("get_theme");
  setTheme(theme as "dark" | "light");
});
</script>
<template>
  <div class="aether-view" style="min-height: 0">
    <SettingsPage />
  </div>
</template>
<style scoped>
.aether-view {
  --aether-opacity: 1;
  border-radius: 0 0 0.5rem 0.5rem;
}
</style>
