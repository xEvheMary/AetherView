<script setup lang="ts">
defineProps<{
  content: any;
}>();
</script>
<template>
  <div>
    <h5>Appearance</h5>
    <p class="text-secondary">Appearance settings will appear here.</p>
    <h6></h6>
    <div class="settings-content">
      <div class="form-floating">
        <input
          type="text"
          class="form-control"
          id="floatingThemeDisabled"
          placeholder="-"
          :value="content?.theme"
          disabled
        />
        <label for="floatingThemeDisabled">Active Theme</label>
      </div>
    </div>
  </div>
</template>
<style scoped>
.text-secondary {
  line-height: 1;
}
.settings-content {
  display: grid;
  grid-template-columns: repeat(2, minmax(100px, 1fr));
  gap: 0.5rem;
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
