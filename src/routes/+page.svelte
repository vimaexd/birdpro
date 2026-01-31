<script lang="ts">
    import HistoryItem from "../components/HistoryItem.svelte";
    import SidebarOscStatus from "../components/SidebarOscStatus.svelte";
    import StepToggle from "../components/StepToggle.svelte";
    import Voicebank from "../components/Voicebank.svelte";
    import SayButton from "../components/SayButton.svelte";
    import SidebarItem from "../components/SidebarItem.svelte";
    import SelectList from "../components/ui/SelectList.svelte";
    import SelectListOption from "../components/ui/SelectListOption.svelte";

    import {
        speakTts,
        ttsStore,
        audioStore,
        initialiseStores,
        setVoice,
        setProvider,
        setAudioDevice,
        ttsVoices,
        audioDevices,
        ttsProviders,
        resolveProvider,
    } from "$lib/bird";
    import { onMount } from "svelte";
    import LoadingSpinner from "../components/LoadingSpinner.svelte";
    import IconCloud from "../assets/icons/IconCloud.svelte";


    let buttonIsDown = $state(false);
    let message = $state("");

    // TODO: refactor ALL of this
    interface Provider {
        id: number;
        name: string;
    }

    let isLoading = $state(false);

    const onSubmit = async () => {
        if (!message) return;

        let msg = message;
        message = "";

        // TODO: push to history

        isLoading = true;
        await speakTts(msg);
        isLoading = false;
    };

    onMount(async () => {
        await initialiseStores();
    });
</script>

<main class="app-container theme-dark">
    <div class="app-left">
        <textarea
            class="talkbox"
            placeholder="type something to say"
            bind:value={message}
            onkeydown={(e) => {
                if (e.key == "Enter") {
                    e.preventDefault();
                    buttonIsDown = true
                }
            }}
            onkeyup={(e) => {
                if (e.key == "Enter") {
                    e.preventDefault();
                    buttonIsDown = false;
                    onSubmit();
                }
            }}
        ></textarea>
        <SayButton onclick={onSubmit} loading={isLoading} active={buttonIsDown}/>
        <div class="history">
            <h2 class="history-title">History</h2>
            <div class="history-items">
                <HistoryItem>woof</HistoryItem>
            </div>
        </div>
    </div>
    <div class="app-right">
        <!-- <StepToggle name="pitch"/> -->
        <!-- <StepToggle name="speed"/> -->

        {#if $ttsStore.providerId}
            <Voicebank
                voiceName={$ttsStore.voice}
                provider={resolveProvider($ttsStore.providerId).name}
                cloud={resolveProvider($ttsStore.providerId).cloud}
            />
        {:else}
            <LoadingSpinner/>
        {/if}

        <SidebarItem title="Debug">
            <div class="debug">
                <p>Provider</p>
                <SelectList bind:value={$ttsStore.providerId} onChange={() => setProvider($ttsStore.providerId)}>
                    {#each $ttsProviders as provider}
                        <SelectListOption value={provider.id}>
                            {provider.name}
                            {#if provider.cloud}
                                <IconCloud width="16px" height="16px"/>
                            {/if}
                        </SelectListOption>
                    {/each}
                    <SelectListOption value={0}>
                        Dummy Provider 1
                    </SelectListOption>
                    <SelectListOption value={0}>
                        Dummy Provider 2
                    </SelectListOption>
                </SelectList>

                <p>Voice</p>
                <SelectList bind:value={$ttsStore.value} onChange={() => setVoice($ttsStore.value)}>
                    {#each $ttsVoices as voice}
                        <SelectListOption value={voice}>
                            {voice}
                        </SelectListOption>
                    {/each}
                </SelectList>

                <p>Output Device</p>
                <SelectList onChange={() => setAudioDevice($audioStore.device)} bind:value={$audioStore.device}>
                    {#each $audioDevices as device}
                        <SelectListOption value={device}>
                            {device}
                        </SelectListOption>
                    {/each}
                </SelectList>
            </div>
        </SidebarItem>
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
        grid-template-columns: minmax(0, 3fr) 360px;
        grid-template-rows: 1fr;
        min-height: 100vh;
        max-height: 100vh;

        min-width: 100vw;
        height: 100%;
        width: 100%;

        gap: 16px;
        padding: 12px;
        background: var(--color-bg);

        overflow: hidden;
    }

    /* set general text color */
    .app-container,
    textarea {
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
        display: flex;
        flex-direction: column;
        gap: 8px;

        max-height: calc(100vh - 24px);
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
            display: flex;
            flex-direction: column;
            min-width: 0;
            gap: 8px;
            width: 100%;
        }
    }

    .debug {
        height: 100%;
    }
</style>
