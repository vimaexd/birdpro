import { invoke } from "@tauri-apps/api/core";
import { writable, get } from "svelte/store";
import { error, info } from "@tauri-apps/plugin-log";
import { configStore } from "./config";
import { showError } from "./toast";

export interface AudioDevice {
  name: string;
  sample_rate: number;
  bit_depth: number;
}

export let audioStore = writable<{
  devices: { [idx: number]: string };
}>({
  devices: {},
});

/**
 * tries to resurrect audio devices from the configStore
 */
export async function tryResurrectAudioConfig() {
  info("attempting to resurrect audio config");

  let cfg = get(configStore);
  let entr = Object.entries(cfg.audio.devices);

  for (let i = 0; i < entr.length; i++) {
    try {
      await setAudioDevice(entr[i][1], +entr[i][0]);
      audioStore.set({
        ...get(audioStore),
        devices: {
          ...get(audioStore).devices,
          [+entr[i][0]]: entr[i][1]
        }
      })
    } catch (e: any) {
      error(e);
      showError(
        `Failed to set audiosetup ${entr[i][0]} (${entr[i][1]}) from config`,
        e,
      );
    }
  }
}

export async function setAudioDevice(device: string, idx: number = 0) {
  configStore.set({
    ...get(configStore),
    audio: {
      ...get(configStore).audio,
      devices: {
        ...get(configStore).audio.devices,
        [idx]: device,
      },
    },
  });
  await _setAudioDeviceBackend(device, idx);
}

export async function _setAudioDeviceBackend(device: string, idx: number = 0) {
  await invoke("audio_set_device", { setupIdx: idx, deviceName: device });
}

export async function destroyAudioDevice(idx: number) {
  let as = get(audioStore);
  delete as.devices[1];
  audioStore.set(as);
  await invoke("audio_destroy", { setupIdx: idx });
}

export async function getAudioDeviceInfo(idx: number): Promise<AudioDevice> {
  let info: AudioDevice = await invoke("audio_get_device", { setupIdx: idx });
  return info;
}
