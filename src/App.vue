<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useProfile, type Profile } from "./composables/useProfile";

const appWindow = getCurrentWindow();
const { switchProfile } = useProfile();
const profileLoaded = ref(false);
import hudControls from "./components/hud/hudControls.vue";
import dashboard from "./components/hud/dashboard.vue";

onMounted(async () => {
  // invoke("position_window_bottom_right");
  const profileMode = await invoke("get_profile_mode");
  switchProfile(profileMode as Profile);
  profileLoaded.value = true;
});
async function startResizing() {
  await appWindow.startResizeDragging("SouthEast");
}
</script>

<template>
  <div class="hud">
    <!-- Controls -->
    <div class="hud-controls">
      <hud-controls />
    </div>
    <div class="resize-handle" @mousedown="startResizing()">⌟</div>
    <!-- Contents -->
    <div v-if="!profileLoaded">Loading...</div>
    <div v-else class="h-100">
      <dashboard />
    </div>
  </div>
</template>
<style scoped>
.hud {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 0;
}
.hud > :last-child {
  flex-grow: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
.hud-controls {
  opacity: 0;
  transition: opacity 0.3s ease-in-out;
}
.hud:hover .hud-controls,
body:hover .hud .hud-controls {
  opacity: 1;
}
.resize-handle {
  cursor: se-resize;
  position: absolute;
  bottom: 0;
  right: 0;
  font-size: 1.2rem;
}
</style>
