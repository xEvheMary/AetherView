<script setup lang="ts">
import { useProfile } from "@/composables/useProfile";
import { useFormType } from "@/composables/useFormType";
import Default from "@/components/profile/default.vue";
import Minimal from "@/components/profile/minimal.vue";
import Monitor from "@/components/profile/monitor.vue";
import MonitorForm from "@/components/profile/monitorForm.vue";
const { activeProfile } = useProfile();
const { activeForm } = useFormType();
</script>
<template>
  <main
    class="aether-view"
    :class="{ 'aether-view--form': activeProfile === 'form' }"
  >
    <template v-if="activeProfile === 'default'">
      <Default />
    </template>
    <template v-else-if="activeProfile === 'minimal'">
      <Minimal />
    </template>
    <template v-else-if="activeProfile === 'monitor'">
      <Monitor />
    </template>
    <template v-else-if="activeProfile === 'form'">
      <template v-if="activeForm === 'monitor'">
        <MonitorForm />
      </template>
      <template v-else-if="activeForm === 'setting'">
        <!-- Add your setting form component here -->
      </template>
    </template>
  </main>
</template>
