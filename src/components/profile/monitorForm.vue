<script setup lang="ts">
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useProfile } from "@/composables/useProfile";
import { useFormType } from "@/composables/useFormType";
import type { TargetMonitor } from "@/models/monitors";

const { previousProfile, switchProfile } = useProfile();
const { formData, resetForm } = useFormType();
const methodOptions = ["GET"];
const target = ref<TargetMonitor>({
  id: "",
  name: "",
  method: "GET",
  endpoint: "",
  interval_seconds: 0,
  enabled: true,
});
const validation = ref({
  name: true,
  method: true,
  endpoint: true,
  interval: true,
  enabled: true,
});
function validate() {
  validation.value.name = target.value.name.trim() !== "";
  validation.value.method = target.value.method.trim() !== "";
  validation.value.endpoint = target.value.endpoint.trim() !== "";
  validation.value.interval = target.value.interval_seconds > 0;
  validation.value.enabled = typeof target.value.enabled === "boolean";
  return Object.values(validation.value).every((v) => v);
}
function cancel() {
  // Reset the target to its initial state or previous profile if needed
  target.value = {
    id: "",
    name: "",
    method: "",
    endpoint: "",
    interval_seconds: 0,
    enabled: true,
  };
  if (previousProfile.value) {
    resetForm();
    switchProfile(previousProfile.value);
  }
}
function submit() {
  if (!validate()) {
    return;
  }
  // Handle form submission logic here
  console.log("Submitting target:", target.value);
  invoke("add_monitor_targets", { target: target.value })
    .catch((err) => {
      console.error("Failed to add target:", err);
    })
    .finally(() => {
      cancel();
    });
}
watch(
  formData,
  (newData) => {
    if (newData) {
      target.value = { ...newData };
      console.log("Form data updated:", newData);
    }
  },
  { immediate: true },
);
</script>
<template>
  <div class="monitor-form aether-scroll">
    <div class="d-flex flex-row justify-content-between">
      <i class="bi bi-chevron-left clickable" @click="cancel"></i>
      <p class="px-2">
        <span v-if="target.id === ''">Add</span><span v-else>Edit</span> Monitor
        Target
      </p>
    </div>
    <form class="px-2" @submit.prevent="submit">
      <div class="form-floating mb-2">
        <input
          type="text"
          class="form-control"
          :class="{ 'is-invalid': !validation.name }"
          id="floatingInputName"
          placeholder="Target Name"
          autocomplete="off"
          v-model="target.name"
        />
        <label for="floatingInputName">Name</label>
      </div>
      <div class="form-floating mb-2">
        <select
          class="form-select"
          id="floatingSelect"
          aria-label="Floating label select example"
          :class="{ 'is-invalid': !validation.method }"
          autocomplete="off"
          v-model="target.method"
        >
          <option v-for="option in methodOptions" :key="option" :value="option">
            {{ option }}
          </option>
        </select>
        <label for="floatingSelect">Method</label>
      </div>
      <div class="form-floating mb-2">
        <input
          type="text"
          class="form-control"
          id="floatingInputEndpoint"
          placeholder="Target Endpoint"
          autocomplete="off"
          :class="{ 'is-invalid': !validation.endpoint }"
          v-model="target.endpoint"
        />
        <label for="floatingInputEndpoint">Endpoint</label>
      </div>
      <div class="input-group mb-2">
        <div class="form-floating">
          <input
            type="number"
            class="form-control"
            id="floatingInputInterval"
            placeholder="Target Interval"
            :class="{ 'is-invalid': !validation.interval }"
            autocomplete="off"
            v-model="target.interval_seconds"
          />
          <label for="floatingInputInterval">Interval</label>
        </div>
        <span class="input-group-text">secs</span>
      </div>
      <div class="form-check mb-1">
        <input
          class="form-check-input"
          type="checkbox"
          :class="{ 'is-invalid': !validation.enabled }"
          v-model="target.enabled"
          id="checkChecked"
        />
        <label class="form-check-label" for="checkChecked"> Enabled </label>
      </div>
      <div class="w-100 d-flex justify-content-end">
        <button
          type="button"
          class="btn btn-sm btn-secondary me-2"
          @click="cancel"
        >
          Cancel
        </button>
        <button type="submit" class="btn btn-sm btn-primary">
          <span v-if="target.id === ''">Add</span><span v-else>Edit</span>
        </button>
      </div>
    </form>
  </div>
</template>
<style scoped>
.clickable {
  cursor: pointer;
}
.monitor-form {
  display: flex;
  height: 100%;
  min-height: 0;
  flex-direction: column;
  padding: 16px;
  overflow-y: scroll;
}
.form-floating-sm > .form-control,
.form-floating-sm > .form-select {
  height: calc(2.2rem + 2px);
  min-height: calc(2.2rem + 2px);
  padding: 0.5rem 0.5rem;
  font-size: 0.8rem;
  line-height: 1.1;
}
.form-floating {
  color: var(--av-text);
  & > label {
    color: var(--av-text);
  }
}
.form-select {
  background-color: var(--av-form-bg) !important;
  color: var(--av-text);
}
.form-select option {
  background-color: var(--av-form-bg-solid) !important;
  color: var(--av-text);
}
:root[data-theme="light"] .form-select {
  background: rgb(210, 210, 210);
}
.input-group-text {
  background-color: var(--av-background);
  color: var(--av-text);
}
</style>
