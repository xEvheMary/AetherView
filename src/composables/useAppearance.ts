import { ref } from "vue";

const opacity = ref(0.35);
const theme = ref<"dark" | "light">("dark");

export function useAppearance() {
  function setOpacity(value: number) {
    opacity.value = value;
    document.documentElement.style.setProperty(
      "--aether-opacity",
      value.toString(),
    );
  }
  function toggleTheme() {
    setTheme(theme.value === "dark" ? "light" : "dark");
  }
  function setTheme(value: "dark" | "light") {
    theme.value = value;
    document.documentElement.setAttribute("data-theme", value);
  }

  return {
    opacity,
    theme,
    setOpacity,
    setTheme,
    toggleTheme,
  };
}
