import { path } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/core";
import { exists, mkdir, readFile, writeFile } from "@tauri-apps/plugin-fs";
import { writable, type Writable, get } from "svelte/store";
import { error, info } from "@tauri-apps/plugin-log";
import { showError } from "./toast";
import type { AudioDevice } from "./audio";
import type { TTSStore } from "./bird";
import type { Favourite } from "./favourites";

interface BirdProConfig {
    /* General app state */
    onboarded: boolean;
    last: TTSStore | undefined;
    favourites: Favourite[];
    replacements: {
        [from: string]: string;
    };

    /* Audio */
    audio: {
        usePreviewOutput: boolean;
        devices: {
            [idx: number]: string;
        };
    };
    volumes: number[];

    /* Behaviour */
    audioTypingIndicator: boolean;
    bypassCharLimit: boolean;
    checkForUpdates: boolean;

    /* Integrations */
    vrcOsc: boolean;

    txtoutput: boolean;
    "txtoutput.clear": boolean;
    "txtoutput.clearTimeout": number;
    "txtoutput.typingIndicator": boolean;
    "txtoutput.typingIndicatorText": string;

    "heartrate": boolean;
    "heartrate.widgetId": string | undefined;
    "heartrate.customMaxHeartrate": number | undefined;
    "heartrate.customConnectedParam": string | undefined;
    "heartrate.customPercentParam": string | undefined;

    /* Providers */
    "elevenlabs.apikey": string;
    "piper.voicesPath": string;

    /* UI */
    "ui.theme": "dark" | "light" | "auto";
    "ui.rounding": 6;
    "ui.accentColor": string;
    "ui.showHistory": boolean;
    "ui.showHrm": boolean;
}

export let configStore: Writable<BirdProConfig>;

async function getConfigPath() {
    const configDir = await path.appConfigDir();
    const configPath = await path.join(configDir, "config.json");
    return configPath;
}

export async function initialiseConfig() {
    let initialConfig: BirdProConfig = {
        onboarded: false,
        vrcOsc: false,
        audio: {
            usePreviewOutput: false,
            devices: {
                0: (
                    (await invoke("audio_get_device", {
                        setupIdx: 0,
                    })) as AudioDevice
                ).name,
            },
        },
        replacements: {
            omw: "On my way!",
        },
        favourites: [],
        volumes: [1.0, 1.0],
        last: undefined,
        txtoutput: false,
        "txtoutput.clear": false,
        "txtoutput.clearTimeout": 10,
        "txtoutput.typingIndicator": false,
        "txtoutput.typingIndicatorText": "[* typing *]",
        "elevenlabs.apikey": "",
        "piper.voicesPath": "/usr/share/piper-voices",
        audioTypingIndicator: false,
        bypassCharLimit: false,
        "ui.theme": "dark",
        "ui.rounding": 6,
        "ui.accentColor": "#4744eb",
        "ui.showHistory": true,
        "ui.showHrm": false,
        checkForUpdates: true,

        heartrate: false,
        "heartrate.widgetId": undefined,
        "heartrate.customConnectedParam": undefined,
        "heartrate.customMaxHeartrate": undefined,
        "heartrate.customPercentParam": undefined,
    };

    let cfgPath = await getConfigPath();
    let cfg;
    try {
        cfg = await readFile(cfgPath);
    } catch (_) {
        info("config file not found, starting with defaults");
        // no file i guess
    }

    if (cfg) {
        try {
            let readConfig = JSON.parse(new TextDecoder().decode(cfg));
            info(`Using config at ${cfgPath}`);

            // fill config keys that arent filled
            let keysEx = Object.keys(initialConfig);
            let keysActual = Object.keys(readConfig);

            keysEx.forEach((k) => {
                if (!keysActual.includes(k)) {
                    readConfig[k] = (initialConfig as any)[k];
                }
            });

            initialConfig = readConfig;
        } catch {
            error("failed to parse config file");
            showError("Failed to parse config file", "");
        }
    }

    configStore = writable<BirdProConfig>(initialConfig);

    configStore.subscribe(async (c) => {
        info("Saving config");
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
                { create: true },
            );
        } catch (e: any) {
            showError("Failed to save config", e);
        }

        await invoke("update_config", { config: c });
    });
}

let configPathFound = false;
