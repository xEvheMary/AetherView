<script setup lang="ts">
import { ref } from "vue";
const popoverRef = ref<HTMLElement | null>(null);
defineProps<{
  top: number;
  left: number;
  enabled?: boolean;
  editTarget?: () => void;
  toggleTarget?: () => void;
  deleteTarget?: () => void;
}>();
defineExpose({
  popoverRef,
});
</script>
<template>
  <Teleport to="body">
    <div
      class="custom-popover context-popover list-group"
      ref="popoverRef"
      :style="{ top: `${top}px`, left: `${left}px` }"
    >
      <button type="button" class="list-group-item" @click="editTarget">
        <i class="bi bi-pencil" style="margin-right: 1rem"></i>Edit
      </button>
      <div class="input-group list-group-item">
        <div class="input-group-text">
          <input
            class="form-check-input mt-0"
            type="checkbox"
            :checked="enabled"
            aria-label="Checkbox for following text input"
            disabled
          />
        </div>
        <button type="button" @click="toggleTarget">
          {{ enabled ? "Disable" : "Enable" }}
        </button>
      </div>

      <button type="button" class="list-group-item" @click="deleteTarget">
        <i class="bi bi-trash" style="margin-right: 1rem"></i>Delete
      </button>
    </div>
  </Teleport>
</template>
<style scoped>
.custom-popover {
  min-width: 0;
}
.context-popover {
  display: flex;
  flex-direction: column;
  background: rgba(20, 20, 20, 0.9);
  backdrop-filter: blur(20px);
  padding: 0;
  .list-group-item {
    border-color: rgba(255, 255, 255, 0.2);
    &.input-group {
      padding: var(--bs-list-group-item-padding-y)
        var(--bs-list-group-item-padding-x);
      display: grid;
      grid-template-columns: auto 1fr;
      align-items: center;
      & button {
        border: none;
        background-color: transparent;
        color: var(--av-color);
      }
    }
  }
}
.form-check-input:disabled {
  opacity: 1;
}
.input-group-text {
  padding: 0 0.3rem 0 0;
  border: none;
  background-color: transparent;
}
</style>
