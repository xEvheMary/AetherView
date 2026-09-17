<script setup lang="ts">
import { computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useProfile, type Profile } from "../../composables/useProfile";
import { useAppearance } from "../../composables/useAppearance";
const props = defineProps<{
  defaultTheme?: string;
  top: number;
  left: number;
}>();

const { activeProfile, switchProfile } = useProfile();
const { theme, setTheme } = useAppearance();
onMounted(() => {
  if (props.defaultTheme === "dark" || props.defaultTheme === "light")
    setTheme(props.defaultTheme);
});
const isDarkMode = computed({
  get: () => theme.value === "dark",
  set: (value: boolean) => setTheme(value ? "dark" : "light"),
});
const isMinimal = computed(() => activeProfile.value === "minimal");
function switchProfileTo(profile: Profile) {
  switchProfile(profile);
}
async function openSettings() {
  try {
    await invoke("open_settings");
  } catch (e) {
    console.error("open_settings failed", e);
  }
}
</script>

<template>
  <Teleport to="body">
    <div
      class="hud-menu"
      :class="{ minimal: isMinimal }"
      :style="{ top: props.top + 'px', left: props.left + 'px' }"
    >
      <div class="menu-section">
        <button class="hud-menu-item" @click="switchProfileTo('default')">
          <i class="bi bi-grid"></i>
          <span
            :style="{ 'font-weight': activeProfile == 'default' ? 'bold' : '' }"
            >Default</span
          >
        </button>
        <button class="hud-menu-item" @click="switchProfileTo('minimal')">
          <i class="bi bi-grid"></i>
          <span
            :style="{ 'font-weight': activeProfile == 'minimal' ? 'bold' : '' }"
            >Minimal</span
          >
        </button>
        <button class="hud-menu-item" @click="switchProfileTo('monitor')">
          <i class="bi bi-grid"></i>
          <span
            :style="{ 'font-weight': activeProfile == 'monitor' ? 'bold' : '' }"
            >Monitor</span
          >
        </button>
      </div>
      <div class="menu-section">
        <button class="hud-menu-item" @click="openSettings">
          <i class="bi bi-gear"></i>
          <span>Settings</span>
        </button>
      </div>
      <div class="menu-section theme-toggle">
        <div class="d-flex justify-content-center align-items-center">
          <i class="bi bi-sun"></i>
        </div>
        <div class="d-flex justify-content-center align-items-center">
          <div class="form-check form-switch">
            <input
              class="form-check-input"
              type="checkbox"
              role="switch"
              id="switchCheckChecked"
              v-model="isDarkMode"
            />
          </div>
        </div>
        <div class="d-flex justify-content-center align-items-center">
          <i class="bi bi-moon"></i>
        </div>
      </div>
    </div>
  </Teleport>
</template>
<style scoped>
/* HUD Menu Styles */
.hud-menu {
  position: fixed;
  min-width: 50px;
  padding: 0.5rem;
  background: rgba(25, 25, 25, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  font-size: 0.9rem;
  z-index: 9999;
}
.hud-menu.minimal {
  display: grid;
  grid-template-rows: repeat(2, auto);
  grid-auto-flow: column;
  grid-auto-columns: max-content;
  gap: 0.25rem;
  align-items: start;
  padding: 0.3rem;
  font-size: 0.8rem;
}
.hud-menu.minimal .menu-section {
  display: contents;
  border: none;
}
.hud-menu-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.3rem;

  border: none;
  background: transparent;
  color: white;
  text-align: left;
  border-radius: 5px;
}
.hud-menu-item:hover {
  background: rgba(255, 255, 255, 0.1);
}
.menu-section:not(:last-child) {
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}
.theme-toggle {
  display: grid;
  gap: 0;
  grid-template-columns: 3fr 6fr 3fr;
  align-items: center;
  justify-content: center;
  margin-top: 0.25rem;
}
.theme-toggle .form-check {
  padding: 0;
  margin: 0;
}
.theme-toggle .form-check-input {
  margin: 0;
}
.form-check {
  min-height: 0;
}
</style>
