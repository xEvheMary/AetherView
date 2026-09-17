<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useAppearance } from "@/composables/useAppearance";

const props = defineProps<{
  defaultOpacity?: number;
  top: number;
  left: number;
}>();

const { opacity, setOpacity } = useAppearance();
const opacityMenu = ref<HTMLElement | null>(null);
const menuOffset = computed(() => {
  const rect = opacityMenu.value?.getBoundingClientRect();
  return rect ? { x: rect.width, y: 0 } : { x: 0, y: 0 };
});
const handleSliderInput = (e: Event) => {
  const v = Number((e.currentTarget as HTMLInputElement).value); // use currentTarget not target
  if (!Number.isNaN(v)) setOpacity(v);
};
watch(
  () => props.defaultOpacity,
  (newVal) => {
    if (newVal !== undefined) setOpacity(newVal);
  },
  { immediate: true },
);
</script>
<template>
  <Teleport to="body">
    <div
      class="opac-menu"
      ref="opacityMenu"
      :style="{
        top: props.top + 'px',
        left: props.left - menuOffset.x + 'px',
        position: 'absolute',
      }"
    >
      <input
        type="range"
        min="0"
        max="1"
        step="0.05"
        :value="opacity"
        @input="handleSliderInput"
      />
      <span>{{ opacity }}</span>
    </div>
  </Teleport>
</template>
<style scoped>
.opac-menu {
  position: fixed;
  display: flex;
  align-items: center;
  min-width: 50px;
  padding: 0.5rem;
  background: rgba(25, 25, 25, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  font-size: 0.9rem;
  z-index: 9999;
}
</style>
