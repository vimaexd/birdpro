<script lang="ts">
    import HistoryItem from "../components/HistoryItem.svelte";
    import SidebarOscStatus from "../components/SidebarOscStatus.svelte";
    import StepToggle from "../components/StepToggle.svelte";
    import Voicebank from "../components/Voicebank.svelte";
    import SayButton from "../components/SayButton.svelte";

    import { invoke } from "@tauri-apps/api/core";
    import { speakTts } from "$lib/bird";
    import { onMount } from "svelte";

    let message = $state("");


    // TODO: refactor ALL of this
    interface Provider {
      id: number;
      name: string;
    }

    let selectedProvider = $state("");
    let providers: Provider[] = $state([]);

    let selectedVoice = $state("");
    let voices: string[] = $state([]);

    let selectedDevice = $state("");
    let devices: string[] = $state([]);

    let isLoading = $state(false);

    const onChangeVoice = async () => {
      await invoke("tts_set_voice", { voice: selectedVoice });
    }

    const onChangeDevice = async () => {
      await invoke("audio_set_device", { deviceName: selectedDevice });
    }

    const onSubmit = async () => {
      if(!message) return;

      let msg = message;
      message = ""

      // TODO: push to history

      isLoading = true;
      await speakTts(msg);
      isLoading = false;
    }

    onMount(async () => {
      providers = await invoke("tts_get_providerlist");
      selectedProvider = await invoke("tts_get_provider");

      console.log(providers);

      voices = await invoke("tts_get_voicelist");
      selectedVoice = await invoke("tts_get_voice");

      devices = await invoke("audio_get_devices");
      selectedDevice = await invoke("audio_get_device");
      console.log(voices);
    })
</script>

<main class="app-container theme-dark">
    <div class="app-left">
        <textarea class="talkbox" placeholder="type something to say"
            bind:value={message}
            onkeypress={(e) => {
              // enter key
              if(e.key == "Enter") {
                e.preventDefault();
                onSubmit();
              }
            }}
        ></textarea>
        <SayButton onclick={onSubmit} loading={isLoading}/>
        <div class="history">
            <h2 class="history-title">History</h2>
            <div class="history-items">
                <HistoryItem>tuah hawk</HistoryItem>
            </div>
        </div>

    </div>
    <div class="app-right">
        <!-- <StepToggle name="pitch"/> -->
        <!-- <StepToggle name="speed"/> -->

        <Voicebank voiceName={selectedVoice} provider="Microsoft Edge TTS [!!CHANGE ME!!]" />

        <div>
            <p>SUPER BETA DEBUG SHIT</p>
            <p>Provider</p>
            <select name="pets" id="pet-select" bind:value={selectedProvider}>
              {#each providers as provider}
               	<option value={provider.id}>{provider.name}</option>
              {/each}
            </select>

            <p>Voice</p>
            <select name="pets" id="pet-select" onchange={onChangeVoice} bind:value={selectedVoice}>
              {#each voices as voice}
               	<option value={voice}>{voice}</option>
              {/each}
            </select>

            <p>Device</p>
            <select name="pets" id="pet-select" onchange={onChangeDevice} bind:value={selectedDevice}>
              {#each devices as device}
               	<option value={device}>{device}</option>
              {/each}
            </select>
        </div>
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
