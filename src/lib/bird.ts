import { invoke } from "@tauri-apps/api/core";
import { writable, get } from "svelte/store";

interface Provider {
  id: string;
  name: string;
}

export let audioDevices = writable([]);
export let ttsProviders = writable<Provider[]>([]);
export let ttsVoices = writable([]);

export let ttsStore = writable<{
  providerId: string;
  voice: string;
}>({
  providerId: "",
  voice: ""
});

export let audioStore = writable({
  device: ""
})

export async function initialiseStores() {
  // populate stores from backend
  ttsProviders.set(await invoke("tts_get_providerlist"));
  updateVoiceList();

  ttsStore.set({
    providerId: await invoke("tts_get_provider"),
    voice: await invoke("tts_get_voice")
  })

  updateDeviceList();
  audioStore.set({
    device: await invoke("audio_get_device")
  })

  console.log(get(ttsProviders))
}

export async function setProvider(providerId: string) {
  await invoke("tts_set_provider", { provider: providerId })
  ttsStore.set({
    ...get(ttsStore),
    providerId: providerId,
    voice: await invoke("tts_get_voice") // fetch new default voice
  });

  console.log(get(ttsStore))
  updateVoiceList();
}

export function resolveProvider(providerId: string): Provider {
  return get(ttsProviders).find(p => p.id == providerId)!
}

export async function setVoice(voice: string) {
  await invoke("tts_set_voice", { voice });
}

export async function setAudioDevice(device: string) {
  await invoke("audio_set_device", { deviceName: device });
}

export async function updateVoiceList() {
  ttsVoices.set(await invoke("tts_get_voicelist"));
}

export async function updateDeviceList() {
  audioDevices.set(await invoke("audio_get_devices"));
}

export async function speakTts(text: string) {
  await invoke("tts_say", { message: text });
}
