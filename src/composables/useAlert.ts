import { ref } from "vue";

export function useAlert() {
  const visible = ref(false);
  const message = ref("");
  const variant = ref("success");

  let timeoutId: number | null = null;

  const showAlert = (
    text: string,
    type: "success" | "danger" | "warning" | "info" = "success",
    duration = 3000,
  ) => {
    message.value = text;
    variant.value = type;
    visible.value = true;

    if (timeoutId) {
      clearTimeout(timeoutId);
    }

    timeoutId = window.setTimeout(() => {
      visible.value = false;
      timeoutId = null;
    }, duration);
  };

  const hideAlert = () => {
    visible.value = false;

    if (timeoutId) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
  };

  return {
    visible,
    message,
    variant,
    showAlert,
    hideAlert,
  };
}
