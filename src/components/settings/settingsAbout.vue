<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app";
import { ref, onMounted } from "vue";

const version = ref("1.0.0");
onMounted(async () => {
  version.value = await getVersion();
});
</script>
<template>
  <div class="settings-about">
    <h5>About</h5>

    <div class="card about-content">
      <div class="card-body aether-scroll">
        <h6>AetherView</h6>

        <p class="mb-1">Lightweight desktop observability HUD<br />Tech :</p>

        <div class="tech-stacks">
          <span class="tech-badge">Tauri</span>
          <span class="tech-badge">Vue</span>
          <span class="tech-badge">Bootstrap</span>
          <span class="tech-badge">TypeScript</span>
          <span class="tech-badge">Rust</span>
        </div>
      </div>
    </div>
    <div class="card footer">
      <div class="card-body d-flex flex-row justify-content-between">
        <p class="mb-0">Version {{ version }}</p>
        <p class="mb-0 ms-auto text-end">© 2026 Alexander</p>
      </div>
    </div>
  </div>
</template>
<style scoped>
.settings-about {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}
.card {
  border-color: var(--av-border);
}
.card.about-content {
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  min-height: 0;
  overflow: hidden;
  & .card-body {
    flex: 1 1 auto;
    min-height: 0;
    overflow-y: auto;
  }
}
.card.footer {
  flex-shrink: 0;
  margin-top: 0.5rem;
  &.card-body {
    padding: 0.25rem 0.5rem;
  }
}
.tech-stacks {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.tech-badge {
  padding: 0.25rem 0.5rem;
  border: 1px solid var(--av-border);
  border-radius: 0.25rem;
}
.card-body.aether-scroll {
  overflow-y: scroll;
}
</style>
