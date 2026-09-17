<script setup lang="ts">
import { computed, ref, watch, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Settings } from "@/models/settings";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useProfile } from "@/composables/useProfile";
import { useAppearance } from "@/composables/useAppearance.ts";
import HudMenu from "./hudMenu.vue";
import OpacityMenu from "./opacityMenu.vue";

const appWindow = getCurrentWindow();
const pinned = ref(false);
const isHover = ref(false);
const showHudMenu = ref(false);
const showOpacityMenu = ref(false);
const burgerButton = ref<HTMLElement | null>(null);
const opacityButton = ref<HTMLElement | null>(null);
const settings = ref<Settings | null>(null);
let unsubscribe: [(() => void) | null] = [null];
const { activeProfile } = useProfile();
const { opacity, theme } = useAppearance();
const isMinimal = computed(() => activeProfile.value === "minimal");
const burgerPosition = computed(() => {
  const rect = burgerButton.value?.getBoundingClientRect();
  if (isMinimal.value) {
    // Appear beside the burger button when in minimal mode
    return rect ? { x: rect.right + 4, y: rect.top - 3 } : { x: 50, y: 10 };
  }
  return rect ? { x: rect.left, y: rect.bottom + 3 } : { x: 50, y: 10 };
});
const opacityPosition = computed(() => {
  const rect = opacityButton.value?.getBoundingClientRect();
  return rect ? { x: rect.right, y: rect.bottom + 3 } : { x: 150, y: 10 };
});
async function togglePin() {
  pinned.value = !pinned.value;
  settings.value!.general.pinned = pinned.value;
  await appWindow.setAlwaysOnTop(pinned.value);
}
async function closeWindow() {
  if (settings.value?.general.close_on_exit) {
    await saveSettings();
    console.log("Settings saved before closing");
    await appWindow.close();
  } else {
    await appWindow.hide();
  }
}
async function startDragging() {
  if (pinned.value) return;
  await appWindow.startDragging();
}
function handleMouseLeave() {
  showHudMenu.value = false;
  showOpacityMenu.value = false;
}
function toggleBurgerMenu() {
  showHudMenu.value = !showHudMenu.value;
  if (showHudMenu.value) {
    showOpacityMenu.value = false;
  }
}
function toggleOpacityMenu() {
  showOpacityMenu.value = !showOpacityMenu.value;
  if (showOpacityMenu.value) {
    showHudMenu.value = false;
  }
}
async function saveSettings() {
  settings.value!.appearance.opacity = opacity.value;
  settings.value!.appearance.theme = theme.value;
  await invoke("store_settings", { settings: settings.value });
}
onMounted(async () => {
  document.body.addEventListener("mouseleave", handleMouseLeave);
  settings.value = await invoke("get_settings");
  for (const item of ["save_settings_before_close", "setting_saved"]) {
    const unsub = await listen(item, async () => {
      if (item === "save_settings_before_close") {
        await saveSettings();
      } else if (item === "setting_saved") {
        settings.value = await invoke("get_settings");
      }
    });
    unsubscribe.push(unsub);
  }
});
onUnmounted(() => {
  document.body.removeEventListener("mouseleave", handleMouseLeave);
  unsubscribe.forEach((unsub) => unsub?.());
});
watch(
  () => settings.value,
  (newVal) => {
    if (newVal !== undefined) {
      pinned.value = newVal?.general.pinned ?? false;
    }
  },
  { immediate: true },
);
</script>
<template>
  <div
    class="d-flex align-items-stretch justify-content-end aether-view"
    @mouseenter="isHover = true"
    @mouseleave="isHover = false"
  >
    <button class="btn btn-link" @click="toggleBurgerMenu()" ref="burgerButton">
      <i class="bi bi-list"></i>
    </button>
    <div
      class="drag-handle flex-grow-1"
      @mousedown.stop="startDragging()"
    ></div>
    <button
      class="btn btn-link"
      @click="togglePin()"
      :class="{ active: pinned }"
    >
      <i class="bi bi-pin"></i>
    </button>
    <button
      class="btn btn-link"
      @click="toggleOpacityMenu()"
      ref="opacityButton"
    >
      <i class="bi bi-eye-slash"></i>
    </button>
    <button class="btn btn-link" @click="closeWindow()">
      <i
        class="bi"
        :class="settings?.general.close_on_exit ? 'bi-x-lg' : 'bi-dash'"
      ></i>
    </button>
  </div>
  <HudMenu
    v-if="showHudMenu"
    :default-theme="settings?.appearance.theme"
    :top="burgerPosition.y"
    :left="burgerPosition.x"
  />
  <OpacityMenu
    v-if="showOpacityMenu"
    :default-opacity="settings?.appearance.opacity"
    :top="opacityPosition.y"
    :left="opacityPosition.x"
  />
</template>
<style scoped>
.aether-view {
  height: auto;
  min-height: 0;
  flex-shrink: 0;
  border-radius: 0.5rem;
  margin-bottom: 2px;
  overflow: visible;
}
.aether-view::before {
  display: none;
}
.aether-view::after {
  display: none;
}
.drag-handle:hover {
  cursor: move;
}
button {
  padding: 0 0.3rem;
  font-size: 1rem;
  text-decoration: none;
  color: inherit;
}
</style>
