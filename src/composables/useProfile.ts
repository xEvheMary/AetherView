import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export type Profile = "default" | "minimal";

const activeProfile = ref<Profile>("default");

export function useProfile() {
  function switchProfile(profile: Profile) {
    activeProfile.value = profile;
    invoke("set_profile_mode", { mode: profile }).catch(console.error);
  }

  return {
    activeProfile,
    switchProfile,
  };
}
