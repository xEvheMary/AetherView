<script setup lang="ts">
import { ref } from "vue";
const currentDate = ref(new Date().toLocaleDateString());
const currentTime = ref(new Date().toLocaleTimeString());

defineProps<{
  minimal?: boolean;
}>();
setInterval(() => {
  currentTime.value = new Date().toLocaleTimeString();
}, 1000);
</script>
<template>
  <div class="clock-widget" :class="{ 'w-100': !minimal }">
    <p v-if="!minimal" class="date">{{ currentDate }}</p>
    <p class="time">{{ currentTime }}</p>
  </div>
</template>
<style scoped>
.clock-widget {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  justify-content: space-between;
  background-color: transparent;
  padding-bottom: 0.5rem;
  margin-bottom: 0.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  p {
    margin: 0;
  }
}
.clock-widget .date {
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}
</style>
