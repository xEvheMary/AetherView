<script setup lang="ts">
import { computed } from "vue";
import { useProfile, type Profile } from "../../composables/useProfile";
defineProps<{
  top: number;
  left: number;
}>();

const { activeProfile, switchProfile } = useProfile();
const isMinimal = computed(() => activeProfile.value === "minimal");
function switchProfileTo(profile: Profile) {
  switchProfile(profile);
}
</script>

<template>
  <Teleport to="body">
    <div
      class="hud-menu"
      :class="{ minimal: isMinimal }"
      :style="{ top: top + 'px', left: left + 'px' }"
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
      </div>
      <div class="menu-section">
        <button class="hud-menu-item">
          <i class="bi bi-gear"></i>
          <span>Settings</span>
        </button>
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
  grid-template-columns: repeat(2, auto);
  grid-auto-flow: column;
  grid-auto-columns: max-content;
  gap: 0.25rem;
  align-items: start;
  padding: 0.3rem;
  font-size: 0.8rem;
}
.hud-menu.minimal .menu-section {
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
</style>
