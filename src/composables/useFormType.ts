import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";

export type Forms = "monitor" | "setting";

const activeForm = ref<Forms>("monitor");

export function useFormType() {
  function switchForm(form: Forms) {
    activeForm.value = form;
  }

  function submitForm() {
    // Implement form submission logic here
  }

  return {
    activeForm,
    switchForm,
    submitForm,
  };
}
