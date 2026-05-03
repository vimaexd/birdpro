import { invoke } from "@tauri-apps/api/core";
import { writable, get } from "svelte/store";
import { showError } from "./toast";
import {
  audioDevices,
  audioStore,
  tryResurrectAudioConfig,
  type AudioDevice,
} from "./audio";
import { configStore } from "./config";
import {
  setTextFileContents,
  startClearTimeout,
  textTimeout,
} from "./txtoutput";
import { favouritesStore } from "./favourites";
import { info } from "@tauri-apps/plugin-log";
import { startUpdateCheck } from "./updates";
import { dev } from "$app/environment";

export interface Provider {
  id: string;
  name: string;
  cloud: boolean;
  supported_platforms: string[];
  supported_features: string[];
}

export interface Voice {
  provider: string;
  id: string;
  name: string;
}

export let ttsProviders = writable<Provider[]>([]);

export interface TTSStore {
  voice: Voice;
  pitch: number;
  rate: number;
}

export let ttsStore = writable<TTSStore>({
  voice: {
    provider: "",
    id: "",
    name: "",
  },
  pitch: 0.0,
  rate: 0.0,
});

export let devmode = writable(dev);

/**
 * Initialise the app
 * Runs after config initialisation
 */
export async function initialiseApp() {
  info(`=== Frontend initialisation ===`);
  const config = get(configStore);

  // Resurrect audio config from loaded config
  await tryResurrectAudioConfig();

  // Download provider list
  let prv: Provider[] = await invoke("tts_get_providerlist");
  if (prv.length < 1) {
    throw "No providers available, possibly an unsupported platform";
  }
  ttsProviders.set(prv);
  console.log("providers", get(ttsProviders));

  updateAudioDeviceList();

  // populate audiostore with default device
  let defaultDevice: AudioDevice = await invoke("audio_get_device", {
    setupIdx: 0,
  });
  if (defaultDevice) {
    info(`Starting with default device (${defaultDevice.name})`);
    let as = get(audioStore);
    as.devices[0] = defaultDevice.name;
  }

  // Restore last voice unless that provider is no longer available
  if (
    config["last"] != undefined &&
    get(ttsProviders)
      .map((p) => p.id)
      .includes(config["last"].voice.provider)
  ) {
    ttsStore.set(config["last"]);
    info(`Last voice restored`);
  } else {
    let defaultProvider: Provider = await invoke("tts_get_default_provider");
    ttsStore.set({
      voice: await invoke("tts_get_default_voice", {
        provider: defaultProvider.id,
      }),
      pitch: 0,
      rate: 0.0,
    });
  }

  // Restore favourites
  favouritesStore.set(config["favourites"]);
  info(`${get(favouritesStore).length} favourites restored`);

  // save updates to current voice to config
  ttsStore.subscribe((t) => {
    let cs = get(configStore);
    cs["last"] = t;

    configStore.set(cs);
  });

  // Start OSC if configured to do so
  if (config.vrcOsc) {
    await invoke("osc_start");
  }

  // Start heart rate monitoring if configured to do so
  if (config.heartrate) {
    try {
      await invoke("hrm_svc_start");
    } catch (err: any) {
      showError("Heart rate error", err);
    }
  }

  // check for updates in the background
  if (config["checkForUpdates"]) {
    setTimeout(startUpdateCheck, 1000);
  }
}

export function resolveProvider(providerId: string): Provider {
  return get(ttsProviders).find((p) => p.id == providerId)!;
}

export async function updateAudioDeviceList() {
  audioDevices.set(await invoke("audio_get_devices"));
  console.log(get(audioDevices));
}

export async function speakTts(text: string, preview: boolean = false) {
  let ttss = get(ttsStore);
  try {
    let message = text;

    // process replacements
    let cs = get(configStore);

    let words = message.split(" ");
    let replacements = Object.entries(cs["replacements"]);

    for (let i = 0; i < words.length; i++) {
      // go through all the replacements
      for (let j = 0; j < replacements.length; j++) {
        if (words[i] == replacements[j][0]) {
          words[i] = replacements[j][1];

          // stop on first replacement found
          break;
        }
      }
    }

    message = words.join(" ");

    await invoke("tts_say", {
      message,
      pitch: ttss.pitch,
      rate: ttss.rate,
      provider: ttss.voice.provider,
      voice: ttss.voice,
      preview,
    });

    // txt file output
    if (cs.txtoutput && !preview) {
      await setTextFileContents(text);
      if (cs["txtoutput.clear"]) {
        if (textTimeout) {
          clearTimeout(textTimeout);
        }
        startClearTimeout(cs["txtoutput.clearTimeout"]);
      }
    }
  } catch (e: any) {
    showError(e, await getErrorText(e));
  }
}

/**
 * Get error text for an error code provided by the backend
 * @param errorCode The error code
 * @returns A description of the error
 */
export async function getErrorText(errorCode: string): Promise<string> {
  return await invoke("get_error_text", { errorCode });
}
