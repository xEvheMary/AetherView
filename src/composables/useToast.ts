import { ref } from "vue";

export type ToastVariant = "success" | "danger" | "warning" | "info";

const visible = ref(false);
const title = ref("");
const message = ref("");
const variant = ref<ToastVariant>("success");

let timeoutId: number | null = null;

export function useToast() {
  const showToast = (
    toastTitle: string,
    toastMessage: string,
    toastVariant: ToastVariant = "success",
    duration = 3000,
  ) => {
    title.value = toastTitle;
    message.value = toastMessage;
    variant.value = toastVariant;
    visible.value = true;

    if (timeoutId) {
      clearTimeout(timeoutId);
    }

    timeoutId = window.setTimeout(() => {
      visible.value = false;
      timeoutId = null;
    }, duration);
  };

  const hideToast = () => {
    visible.value = false;

    if (timeoutId) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
  };

  return {
    visible,
    title,
    message,
    variant,
    showToast,
    hideToast,
  };
}
