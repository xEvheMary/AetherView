<script setup lang="ts">
import { useProfile } from "../../composables/useProfile";
import Default from "../profile/default.vue";
import Minimal from "../profile/minimal.vue";
const { activeProfile } = useProfile();
</script>
<template>
  <main class="aether-view">
    <template v-if="activeProfile === 'default'">
      <Default />
    </template>
    <template v-else-if="activeProfile === 'minimal'">
      <Minimal />
    </template>
  </main>
</template>
