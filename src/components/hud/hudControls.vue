<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useProfile } from "@/composables/useProfile";
import HudMenu from "./hudMenu.vue";
import OpacityMenu from "./opacityMenu.vue";

const pinned = ref(false);
const appWindow = getCurrentWindow();
const isHover = ref(false);
const burgerButton = ref<HTMLElement | null>(null);
const { activeProfile } = useProfile();
const isMinimal = computed(() => activeProfile.value === "minimal");
const showHudMenu = ref(false);
const burgerPosition = computed(() => {
  const rect = burgerButton.value?.getBoundingClientRect();
  if (isMinimal.value) {
    // Appear beside the burger button when in minimal mode
    return rect ? { x: rect.right + 4, y: rect.top - 3 } : { x: 50, y: 10 };
  }
  return rect ? { x: rect.left, y: rect.bottom + 3 } : { x: 50, y: 10 };
});
const showOpacityMenu = ref(false);
const opacityButton = ref<HTMLElement | null>(null);
const opacityPosition = computed(() => {
  const rect = opacityButton.value?.getBoundingClientRect();
  return rect ? { x: rect.right, y: rect.bottom + 3 } : { x: 150, y: 10 };
});
async function togglePin() {
  pinned.value = !pinned.value;
  await appWindow.setAlwaysOnTop(pinned.value);
}
async function hideWindow() {
  await appWindow.hide();
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
onMounted(() => {
  document.body.addEventListener("mouseleave", handleMouseLeave);
});
onUnmounted(() => {
  document.body.removeEventListener("mouseleave", handleMouseLeave);
});
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
    <button class="btn btn-link" @click="hideWindow()">
      <i class="bi bi-dash"></i>
    </button>
  </div>
  <HudMenu
    v-if="showHudMenu"
    :top="burgerPosition.y"
    :left="burgerPosition.x"
  />
  <OpacityMenu
    v-if="showOpacityMenu"
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
