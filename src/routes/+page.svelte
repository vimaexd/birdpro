<script lang="ts">
    import HistoryItem from "../components/HistoryItem.svelte";
    import SidebarOscStatus from "../components/SidebarOscStatus.svelte";
    import StepToggle from "../components/StepToggle.svelte";
    import Voicebank from "../components/Voicebank.svelte";
    import ClickyButton from "../components/ClickyButton.svelte";
    import SidebarItem from "../components/SidebarItem.svelte";
    import SelectList from "../components/ui/SelectList.svelte";
    import SelectListOption from "../components/ui/SelectListOption.svelte";

    import {
        speakTts,
        ttsStore,
        initialiseStores,
        setVoice,
        setProvider,
        ttsVoices,
        ttsProviders,
        resolveProvider,
        audioStore
    } from "$lib/bird";
    import { onMount } from "svelte";
    import Button from '@bird/components/ui/Button.svelte';
    import LoadingSpinner from "../components/LoadingSpinner.svelte";
    import IconCloud from "../assets/icons/IconCloud.svelte";
    import { getLastMessage, historyStore, pushHistory } from "$lib/history";
    import IconPitch from "../assets/icons/IconPitch.svelte";
    import IconRate from "../assets/icons/IconRate.svelte";
    import IconEnter from "@bird/assets/icons/IconEnter.svelte";

    import Settings from "./screens/settings.svelte";

    let talkboxRef: HTMLTextAreaElement;
    let buttonIsDown = $state(false);
    let buttonIsDownPreview = $state(false);
    let message = $state("");

    // TODO: refactor ALL of this
    interface Provider {
        id: number;
        name: string;
    }

    let isLoading = $state(false);
    let isLoadingPreview = $state(false);
    let showSettings = $state(false);

    const sendMessage = async () => {
        if (!message) return;

        let msg = message;
        message = "";

        pushHistory(msg);

        isLoading = true;
        await speakTts(msg);
        isLoading = false;
    };

    const sendPreviewMessage = async () => {
        if (!message) return;

        isLoadingPreview = true;
        await speakTts(message, true);
        isLoadingPreview = false;
    }


    const focusTextbox = () => {
      if(talkboxRef) {
        talkboxRef.focus();
      }
    }

    onMount(async () => {
        await initialiseStores();

        // focus talk box if not focussed
        document.body.addEventListener('keydown', (e) => {
          switch(e.key) {
            case "ArrowUp":
              e.preventDefault();
              if(message == "") {
                message = getLastMessage()
              }
              focusTextbox();
              break;

            // handled by events on text area,
            // but
            case "Enter":
              e.preventDefault();
              if(buttonIsDown) return;
              sendMessage();
              buttonIsDown = true;
              break;

            default:
              focusTextbox();
          }
        });

        document.body.addEventListener('keyup', (e) => {
          switch(e.key) {
            case "Enter":
              e.preventDefault();
              buttonIsDown = false;
              break;
          }
        });
    });
</script>

<main class="app-container theme-dark">
    {#if showSettings}
        <Settings onClose={() => showSettings = false}/>
    {/if}

    <div class="app-left">
        <textarea
            id="talkbox"
            placeholder="type something to say"
            bind:value={message}
            bind:this={talkboxRef}
        ></textarea>
        <div class="buttons">
            {#if $audioStore.devices[1] !== undefined}
                <ClickyButton onclick={sendPreviewMessage} loading={isLoadingPreview} active={buttonIsDownPreview} color="var(--color-surface2)">
                    preview
                </ClickyButton>
            {/if}

            <div style={($audioStore.devices[1] === undefined) ? "grid-column-start: 1; grid-column-end: 3;": ''}>
                <ClickyButton
                        onclick={sendMessage}
                        loading={isLoading}
                        active={buttonIsDown}>
                    <IconEnter height={24} width={24}/> <span class="action">say</span>
                </ClickyButton>
            </div>
        </div>

        <div class="history">
            <div class="history-side">
                <vr/>
                <h2 class="history-title">History</h2>
                <vr/>
            </div>
            <div class="history-items">
                {#if $historyStore.length < 1}
                    <span class="history-empty">Say something, and it'll show up here!</span>
                {/if}
                {#each $historyStore as item}
                    <HistoryItem onclick={() => {
                      message = item
                    }}>{item}</HistoryItem>
                {/each}
            </div>
        </div>
    </div>
    <div class="app-right">
        {#if $ttsStore.providerId}
            <Voicebank
                voiceName={$ttsStore.voice.name}
                provider={resolveProvider($ttsStore.providerId).name}
                cloud={resolveProvider($ttsStore.providerId).cloud}
            />
        {:else}
            <LoadingSpinner/>
        {/if}

        <StepToggle
            majStep={5}
            minStep={1}
            initial={0}
            min={-48} max={48}
            bind:value={$ttsStore.pitch}
        >
            <IconPitch width={24} height={24}/>
            <h2>Pitch</h2>
        </StepToggle>

        <StepToggle
            initial={0}
            majStep={1}
            minStep={0.5}
            min={-8} max={8}
            bind:value={$ttsStore.rate}
        >
            <IconRate width={24} height={24}/>
            <h2>Rate</h2>

        </StepToggle>

        <Button onclick={() => showSettings = true}>Settings</Button>

        <SidebarItem title="Debug">
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
            </SelectList>

            <p>Voice</p>
            <SelectList
                bind:value={$ttsStore.voice.id}
                onChange={() => setVoice($ttsStore.voice.id)}
                height="200px">
                {#each $ttsVoices as voice}
                    <SelectListOption value={voice.id}>
                        {voice.name}
                    </SelectListOption>
                {/each}
            </SelectList>
        </SidebarItem>
        <SidebarOscStatus>
            Placeholder Status
        </SidebarOscStatus>
    </div>
</main>

<style>
    .app-container {
        display: grid;
        grid-template-columns: minmax(0, 3fr) 420px;
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

    @keyframes flash {
        0% {
            outline-color: var(--color-surface0);
        }
        20% {
            outline-color: color-mix(in srgb, var(--color-surface0) 80%, #fff 20%);
        }
        100% {
            outline-color: var(--color-surface0);
        }
    }

    #talkbox {
        font-size: 1.25rem;
        font-family: var(--font-family);
        border-radius: var(--rounding);
        padding: 12px;
        width: 100%;
        height: 100%;
        background: transparent;
        border: none;
        outline: 1px var(--color-surface0) solid;
        resize: none;

        will-change: outline-width;
        transition: outline-width .1s var(--ease-out-expo);
    }

    #talkbox:focus {
        outline-offset: 0;
        animation: flash .4s;
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
        flex-grow: 0;
        gap: 8px;

        max-height: calc(100vh - 24px);

        overflow-y: auto;
    }

    .buttons {
        display: grid;
        grid-template-columns: 128px 1fr;
        gap: 8px;

        & .preview {

        }
    }

    .history {
        margin-top: 4px;
        height: 50%;
        display: flex;
        gap: 8px;

        .history-side {
            writing-mode: vertical-rl;
            text-orientation: sideways;
            transform: rotate(180deg);
            display: flex;
            flex-direction: row;
            align-items: center;

            gap: 16px;

            vr {
                height: 100%;
                border-right: 1px var(--color-surface0) solid;
            }
        }

        .history-title {
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

        .history-empty {
            opacity: .35;
            font-style: italic;
        }
    }
</style>
