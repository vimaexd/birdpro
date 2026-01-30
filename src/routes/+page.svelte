<script lang="ts">
    import HistoryItem from "../components/HistoryItem.svelte";
    import SidebarOscStatus from "../components/SidebarOscStatus.svelte";
    import StepToggle from "../components/StepToggle.svelte";
    import Voicebank from "../components/Voicebank.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { speakTts } from "$lib/bird";
    import { onMount } from "svelte";

    let message = "";
    let selectedVoice = "";
    let voices: string[] = [];

    let selectedDevice = "";
    let devices: string[] = [];

    const onChangeVoice = async () => {
      await invoke("tts_set_voice", { voice: selectedVoice });
    }

    const onSubmit = () => {
      speakTts(message);
    }

    onMount(async () => {
      voices = await invoke("tts_get_voices");
      devices = await invoke("audio_get_devices");
      console.log(voices);
    })
</script>

<main class="app-container theme-dark">
    <div class="app-left">
        <textarea class="talkbox" placeholder="type something to say" bind:value={message}></textarea>
        <button class="btn-say" onclick={onSubmit}>
            ↵ say
        </button>
        <div class="history">
            <h2 class="history-title">History</h2>
            <div class="history-items">
                <HistoryItem>awawawa</HistoryItem>
                <HistoryItem>im bird</HistoryItem>
                <HistoryItem>Hey guys, did you know that in terms of bird and puppy breeding, Vaporeo</HistoryItem>
                <HistoryItem>year of the linux desktop</HistoryItem>
            </div>
        </div>

    </div>
    <div class="app-right">
        <StepToggle name="pitch"/>
        <StepToggle name="speed"/>

        <p>SUPER BETA DEBUG SHIT</p>
        <p>Voice</p>
        <select name="pets" id="pet-select" onchange={onChangeVoice} bind:value={selectedVoice}>
          {#each voices as voice}
           	<option value={voice}>{voice}</option>
          {/each}
        </select>

        <p>Device</p>
        <select name="pets" id="pet-select" bind:value={selectedDevice}>
          {#each devices as device}
           	<option value={device}>{device}</option>
          {/each}
        </select>
        <!-- <Voicebank/> -->
        <!-- <SidebarOscStatus>
            Connected to VRChat OSC
        </SidebarOscStatus>
        <SidebarOscStatus>
            Running Browser Source
        </SidebarOscStatus> -->
    </div>
</main>

<style>
    .app-container {
        display: grid;
        grid-template-columns: minmax(0, 3fr) minmax(0, 2fr);
        grid-template-rows: 1fr;
        min-height: 100vh;
        height: 100%;
        width: 100%;

        gap: 4px;
        padding: 12px;
        background: var(--color-bg);
    }

    /* set general text color */
    .app-container, textarea {
        color: var(--color-text);
    }

    .talkbox {
        font-size: 1.25rem;
        font-family: var(--font-family);
        border-radius: var(--rounding);
        padding: 12px;
        width: 100%;
        height: 400px;
        background: transparent;
        border-width: 2px;
        border-color: var(--color-surface0);
        resize: none;

        height: 100%;
    }

    .btn-say {
        width: 100%;
        background-color: #4744eb;
        color: #fff;
        border: none;
        padding: 24px 16px;
        font-size: 1.25rem;

        border-radius: var(--rounding);

        transition: filter, transform .15s var(--ease-out-expo);
        will-change: filter;

        box-shadow: 0px 4px color-mix(in srgb, #4744eb 70%, white 30%);

        cursor: pointer;

        &:hover {
            filter: brightness(1.35);
        }

        &:active {
            transform: translateY(4px);
            box-shadow: 0px 0px color-mix(in srgb, #4744eb 70%, white 30%);
        }
    }

    .app-left {
        display: flex;
        flex-direction: column;
        width: 100%;
        gap: 8px;
    }
    .app-right {
        padding: 0px 8px;

        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .history {
        height: 50%;
        display: flex;
        gap: 8px;

        .history-title {
            writing-mode: vertical-rl;
            text-orientation: sideways;
            transform: rotate(180deg);
            letter-spacing: 0.1px;
            font-size: 0.75rem;
            user-select: none;
        }

        .history-items {
            justify-content: end;
            display:flex;
            flex-direction: column;
            min-width: 0;
            gap: 8px;
            width: 100%;
        }
    }
</style>
