import { invoke } from "@tauri-apps/api/core";
import { writable, get } from "svelte/store";
import { showError } from "./toast";
import { tryResurrectAudioConfig } from "./audio";
import { configStore } from "./config";

export interface Provider {
  id: string;
  name: string;
  cloud: boolean;
}

export interface Voice {
  provider: string;
  id: string;
  name: string;
}

export let audioDevices = writable([]);
export let ttsProviders = writable<Provider[]>([]);

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
  rate: 0.0
});

/**
 * Initialise the app
 * Runs after config initialisation
 */
export async function initialiseApp() {
  const config = get(configStore);

  // Resurrect audio config from loaded config
  await tryResurrectAudioConfig();

  // Start OSC if configured to do so
  if (config.vrcOsc) {
    invoke("osc_start");
  }

  // Download provider list
  ttsProviders.set(await invoke("tts_get_providerlist"));
  updateAudioDeviceList();

  // Initialise TTS store with defaults from the backend
  ttsStore.set({
    providerId: await invoke("tts_get_provider"),
    voice: await invoke("tts_get_voice"),
    pitch: 0,
    rate: 0.0
  });

  console.log("providers", get(ttsProviders));
}

export async function setProvider(providerId: string) {
  await invoke("tts_set_provider", { provider: providerId })
  ttsStore.set({
    ...get(ttsStore),
    providerId: providerId,
    voice: await invoke("tts_get_voice") // fetch new default voice
  });
}

export function resolveProvider(providerId: string): Provider {
  return get(ttsProviders).find(p => p.id == providerId)!
}

export async function setVoice(voice: Voice) {
  await invoke("tts_set_voice", { voice });
}

export async function updateAudioDeviceList() {
  audioDevices.set(await invoke("audio_get_devices"));
}

export async function speakTts(text: string, preview: boolean = false) {
  let ttss = get(ttsStore);
  try {
    await invoke("tts_say", {
      message: text,
      pitch: ttss.pitch,
      rate: ttss.rate,
      provider: ttss.providerId,
      voice: ttss.voice,
      preview
    });
  } catch (e: any) {
    showError(e, await getErrorText(e))
  }
}

/**
 * Get error text for an error code provided by the backend
 * @param errorCode The error code
 * @returns A description of the error
 */
export async function getErrorText(errorCode: string): Promise<string> {
  return await invoke("get_error_text", { errorCode })
}
