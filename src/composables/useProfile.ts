import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export type Profile = "default" | "minimal" | "monitor" | "form";

const activeProfile = ref<Profile>("default");
const previousProfile = ref<Profile | null>(null);

export function useProfile() {
  function switchProfile(profile?: Profile) {
    if (!profile) return;
    if (activeProfile.value !== "form") {
      previousProfile.value = activeProfile.value;
    }
    activeProfile.value = profile ?? "default";
    invoke("set_profile_mode", { mode: activeProfile.value }).catch(
      console.error,
    );
  }

  return {
    activeProfile,
    previousProfile,
    switchProfile,
  };
}
