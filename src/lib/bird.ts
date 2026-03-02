import { invoke } from "@tauri-apps/api/core";
import { writable, get } from "svelte/store";
import { showError } from "./toast";
import { tryResurrectAudioConfig } from "./audio";
import { configStore } from "./config";
import { setTextFileContents, startClearTimeout, textTimeout } from "./txtoutput";
import { favouritesStore } from "./favourites";
import { info } from "@tauri-apps/plugin-log";

export interface Provider {
  id: string;
  name: string;
  cloud: boolean;
  supported_platforms: string[];
  supported_features: string[]
}

export interface Voice {
  provider: string;
  id: string;
  name: string;
}

export let audioDevices = writable([]);
export let ttsProviders = writable<Provider[]>([]);

export interface TTSStore {
  providerId: string;
  voice: Voice;
  pitch: number;
  rate: number;
}

export let ttsStore = writable<TTSStore>({
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
  info(`=== frontend initialisation ===`)
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

  // Restore last voice
  if (config["last"] != undefined) {
    ttsStore.set(config["last"])
    info(`Last voice restored`)
  } else {
    ttsStore.set({
      providerId: await invoke("tts_get_provider"),
      voice: await invoke("tts_get_voice"),
      pitch: 0,
      rate: 0.0
    });
  }

  // Restore favourites
  favouritesStore.set(config["favourites"]);
  info(`${get(favouritesStore).length} favourites restored`)

  // save updates to current voice to config
  ttsStore.subscribe(t => {
    let cs = get(configStore);
    cs["last"] = t;

    configStore.set(cs);
  })

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

export async function updateAudioDeviceList() {
  audioDevices.set(await invoke("audio_get_devices"));
}

export async function speakTts(text: string, preview: boolean = false) {
  let ttss = get(ttsStore);
  try {
    let message = text;

    // process replacements
    let cs = get(configStore);
    let replacements = Object.entries(cs['replacements']);
    for (let i = 0; i < replacements.length; i++) {
      message = message.replaceAll(replacements[i][0], replacements[i][1]);
    }

    await invoke("tts_say", {
      message,
      pitch: ttss.pitch,
      rate: ttss.rate,
      provider: ttss.providerId,
      voice: ttss.voice,
      preview
    });

    // txt file output
    if (cs.txtoutput && !preview) {
      await setTextFileContents(text);
      if (cs["txtoutput.clear"]) {
        if (textTimeout) {
          clearTimeout(textTimeout)
        }
        startClearTimeout(cs["txtoutput.clearTimeout"])
      }
    }
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
