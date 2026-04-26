import { invoke } from "@tauri-apps/api/core";
import { writable, get } from "svelte/store";
import { error, info } from "@tauri-apps/plugin-log";
import { configStore } from "./config";
import { showError } from "./toast";
import { show } from "@tauri-apps/api/app";

export interface AudioDevice {
    name: string;
    sample_rate: number;
    bit_depth: number;
}

export let audioDevices = writable([]);
export let audioStore = writable<{
    devices: { [idx: number]: string };
}>({
    devices: {},
});

/**
 * tries to resurrect audio devices from the configStore
 */
export async function tryResurrectAudioConfig() {
    info("Attempting to resurrect audio config");

    let cfg = get(configStore);
    let entr = Object.entries(cfg.audio.devices);

    for (let i = 0; i < entr.length; i++) {
        try {
            await _setAudioDeviceBackend(entr[i][1], +entr[i][0]);
        } catch (e: any) {
            error(e);
            showError(`Audio setup failure`, e);
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
    try {
        await invoke("audio_set_device", { setupIdx: idx, deviceName: device });
    } catch (err: any) {
        showError(
            "Audio device error",
            `Tried to set Output ${idx} to ${device}, got ${err}`,
        );
    }
}

export async function destroyAudioDevice(idx: number) {
    let as = get(audioStore);
    delete as.devices[idx];
    audioStore.set(as);
    await invoke("audio_destroy", { setupIdx: idx });

    let cs = get(configStore);
    delete cs.audio.devices[idx];
    configStore.set(cs);
}

export async function getAudioDeviceInfo(idx: number): Promise<AudioDevice> {
    let info: AudioDevice = await invoke("audio_get_device", { setupIdx: idx });
    return info;
}
