import { invoke } from "@tauri-apps/api/core";
import { writable, get } from "svelte/store";
import { showError } from "./toast";
import { tryResurrectAudioConfig } from "./audio";

interface Provider {
  id: string;
  name: string;
  cloud: boolean;
}

interface Voice {
  provider: string;
  id: string;
  name: string;
}

export let audioDevices = writable([]);
export let ttsProviders = writable<Provider[]>([]);
export let ttsVoices = writable<Voice[]>([]);

export let ttsStore = writable<{
  providerId: string;
  voice: Voice;
  pitch: number;
  rate: number;
}>({
  providerId: "",
  voice: {
    provider: "",
    id: "",
    name: ""
  },
  pitch: 0.0,
  rate: 1.0
});


export async function initialiseStores() {
  await tryResurrectAudioConfig();

  ttsProviders.set(await invoke("tts_get_providerlist"));
  updateVoiceList();
  updateAudioDeviceList();

  ttsStore.set({
    providerId: await invoke("tts_get_provider"),
    voice: await invoke("tts_get_voice"),
    pitch: 0,
    rate: 1.0
  })

  console.log("voices", get(ttsVoices));
  console.log("providers", get(ttsProviders));
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

export async function setVoice(voiceId: string) {
  let voice = get(ttsVoices).find(v => v.id == voiceId);
  await invoke("tts_set_voice", { voice });
}

export async function updateVoiceList() {
  ttsVoices.set(await invoke("tts_get_voicelist"));
}

export async function updateAudioDeviceList() {
  audioDevices.set(await invoke("audio_get_devices"));
}

export async function speakTts(text: string, preview: boolean = false) {
  let ttss = get(ttsStore);
  try {
    await invoke("tts_say", { message: text, pitch: ttss.pitch, rate: ttss.rate, preview });
  } catch (e: any) {
    showError(e, await getErrorText(e))
  }
}

export async function getErrorText(errorCode: string): Promise<string> {
  return await invoke("get_error_text", { errorCode })
}
