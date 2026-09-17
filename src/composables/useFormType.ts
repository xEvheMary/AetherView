import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";

export type Forms = "monitor" | "setting";

const activeForm = ref<Forms>("monitor");
const formData = ref<any>(null);

export function useFormType() {
  function switchForm(form: Forms, data?: any) {
    activeForm.value = form;
    formData.value = data;
  }

  function submitForm() {
    // Implement form submission logic here
  }

  function resetForm() {
    formData.value = null;
  }

  return {
    activeForm,
    formData,
    switchForm,
    submitForm,
    resetForm,
  };
}
