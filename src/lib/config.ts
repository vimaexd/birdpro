import { path } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/core";
import { exists, mkdir, readFile, writeFile } from "@tauri-apps/plugin-fs";
import { writable, type Writable, get } from "svelte/store";
import { error, info } from '@tauri-apps/plugin-log';
import { showError } from "./toast";
import type { AudioDevice } from "./audio";

interface BirdProConfig {
  vrcOsc: boolean;
  audio: {
    usePreviewOutput: boolean;
    devices: {
      [idx: number]: string;
    }
  }
}

export let configStore: Writable<BirdProConfig>;

async function getConfigPath() {
  const configDir = await path.appConfigDir();
  const configPath = await path.join(configDir, "config.json");
  return configPath;
}

export async function initialiseConfig() {
  let initialConfig: BirdProConfig = {
    vrcOsc: false,
    audio: {
      usePreviewOutput: false,
      devices: {
        0: (await invoke("audio_get_device", { setupIdx: 0 }) as AudioDevice).name
      }
    }
  }

  let cfgPath = await getConfigPath();
  let cfg;
  try { cfg = await readFile(cfgPath); }
  catch (_) {
    info("config file not found, starting with defaults")
    // no file i guess
  }

  if (cfg) {
    try {
      initialConfig = JSON.parse(new TextDecoder().decode(cfg));
      info(`using config at ${cfgPath}`)
    } catch {
      error("failed to parse config file");
      showError("Failed to parse config file", "");
    }

  }

  configStore = writable<BirdProConfig>(initialConfig);

  configStore.subscribe(async c => {
    const configDir = await path.appConfigDir();
    const configPath = await getConfigPath();

    if (!configPathFound) {
      let doesExist = await exists(configDir);
      if (!doesExist) {
        await mkdir(configDir);
      }
      configPathFound = true;
    }

    try {
      await writeFile(
        configPath,
        new TextEncoder().encode(JSON.stringify(c)),
        { create: true })
    } catch (e: any) {
      showError("Failed to save config", e);
    }
  })
}

let configPathFound = false;
