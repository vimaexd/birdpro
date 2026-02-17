<script lang="ts">
    import HistoryItem from "../components/HistoryItem.svelte";
    import StepToggle from "../components/StepToggle.svelte";
    import Voicebank from "../components/Voicebank.svelte";
    import ClickyButton from "../components/ClickyButton.svelte";

    import {
        speakTts,
        ttsStore,
        resolveProvider,

        type Provider

    } from "$lib/bird";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";
    import LoadingSpinner from "../components/LoadingSpinner.svelte";
    import { getLastMessage, historyStore, pushHistory } from "$lib/history";
    import { isSettingsOpen } from "@bird/lib/modal";

    import IconPitch from "../assets/icons/IconPitch.svelte";
    import IconRate from "../assets/icons/IconRate.svelte";
    import IconEnter from "@bird/assets/icons/IconEnter.svelte";

    import Settings from "./screens/settings.svelte";
    import { configStore, initialiseConfig } from "@bird/lib/config";
    import SplitMenus from "@bird/components/SplitMenus.svelte";
    import StatusBar from "@bird/components/StatusBar.svelte";
    import IconStop from "@bird/assets/icons/IconStop.svelte";
    import IconHeadphones from "@bird/assets/icons/IconHeadphones.svelte";
    import {getAllWindows, getCurrentWindow} from "@tauri-apps/api/window";
    import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";
    import { setTextFileContents, setTextTypingIndicator } from "@bird/lib/txtoutput";
    import { info } from "@tauri-apps/plugin-log";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";

    let provider: Provider = $derived.by(() => {
      return resolveProvider($ttsStore.providerId)
    });

    let talkboxRef: HTMLTextAreaElement;
    let buttonIsDown = $state(false);
    let buttonIsDownPreview = $state(false);
    let message = $state("");

    let isLoadingTts = $state(false);
    let isLoadingPreview = $state(false);

    let barSize = $state(380);
    let resizeBar = $state(false);

    let typingIndicatorLastLength = 0;
    let typingIndicatorShowing = $state(false);
    let typingIndicatorTimeout: number;

    const onTyping = async () => {
        // regardless of whever this is already showing, send typing over OSC
        if($configStore["vrcOsc"]) {
          await invoke("osc_typing_indicator", { typing: true });
        }

        // if the indicator is already showing, extend the timeout by 4s
        if (typingIndicatorShowing) {
            clearTimeout(typingIndicatorTimeout);
            typingIndicatorTimeout = setTimeout(onTypingTimeout, 4000);
            return;
        }
        // prevent this from being called until we time out

        typingIndicatorShowing = true;

        info("Showing typing indicator");

        // set osc and txt typing indicators
        if($configStore["txtoutput"] && $configStore["txtoutput.typingIndicator"]) {
          await setTextTypingIndicator($configStore["txtoutput.typingIndicatorText"]);
        }

        // set audio typing indicator
        if($configStore["audioTypingIndicator"]) {
          await invoke("audio_typingindicator_start");
        }

        typingIndicatorTimeout = setTimeout(onTypingTimeout, 4000);
    };

    // clear after the timeout from not typing has run
    const onTypingTimeout = async () => {
        info("Hiding typing indicator");
        await invoke("osc_typing_indicator", { typing: false });
        await invoke("audio_typingindicator_stop");
        await setTextTypingIndicator("");
        typingIndicatorShowing = false;
    };

    const sendMessage = async () => {
        if (!message) return;

        let msg = message;
        message = "";

        pushHistory(msg);

        isLoadingTts = true;

        await speakTts(msg);

        // clear typing indicators
        if(typingIndicatorTimeout) {
          clearTimeout(typingIndicatorTimeout);
          onTypingTimeout();
        }

        isLoadingTts = false;
    };

    const sendPreviewMessage = async () => {
        if (!message) return;

        isLoadingPreview = true;
        await speakTts(message, true);
        isLoadingPreview = false;
    };

    const focusTextbox = () => {
        if (talkboxRef) {
            talkboxRef.focus();
        }
    };

    const trackMouseAndResizeBar = (e: MouseEvent) => {
      if(!resizeBar) return;
      console.log("rsz")
      let newWidth = (window.innerWidth - e.pageX);
      barSize = Math.min(Math.max(newWidth, 400), 800) - 18
    }

    onMount(async () => {
        await getCurrentWindow().listen("tauri://focus", () => {
            if (!$isSettingsOpen) {
                focusTextbox()
            }
        })

        document.body.addEventListener("keydown", (e) => {
            // dont capture strokes if settings is shown
            // the user needs that to type stuff!!!
            if($isSettingsOpen) {
              return;
            }

            switch (e.key) {
                case ",":
                    if(e.ctrlKey) {
                      $isSettingsOpen = true;
                    }
                    break;

                case "ArrowUp":
                    e.preventDefault();
                    if (message == "") {
                        message = getLastMessage();
                    }
                    focusTextbox();
                    break;

                case "Enter":
                    e.preventDefault();
                    if(e.altKey && $configStore.audio.usePreviewOutput) {
                      // preview mode!
                      if (buttonIsDownPreview) return;
                      sendPreviewMessage();
                      buttonIsDownPreview = true;
                    } else {
                      if (buttonIsDown) return;
                      sendMessage();
                      buttonIsDown = true;
                    }
                    break;

                default:
                    focusTextbox();
            }
        })

        document.body.addEventListener("keyup", (e) => {
            switch (e.key) {
                case "Enter":
                    e.preventDefault();
                    buttonIsDownPreview = false;
                    buttonIsDown = false;
                    break;
            }
        });
    });
</script>

<!--svelte-ignore a11y_no_noninteractive_element_interactions -->
<main class="app-container theme-dark"
    style="--sidebar-width: {barSize}px"
    onmousemove={trackMouseAndResizeBar}
    onmouseup={() => { resizeBar = false }}
    in:fade={{ duration: 300 }}>
    {#if $isSettingsOpen}
        <Settings onClose={() => ($isSettingsOpen = false)} />
    {/if}

    <div class="app-left">
        <div class="talkbox-container">
            <textarea
                id="talkbox"
                placeholder="type something to say"
                bind:value={message}
                bind:this={talkboxRef}
                oninput={(e: any) => {
                    // don't count deleting as typing
                    if (typingIndicatorLastLength < e.target.value.length + 1) {
                        onTyping();
                    }
                    typingIndicatorLastLength = e.target.value.length;
                }}
                maxlength="144"
            >
            </textarea>
            {#if typingIndicatorShowing}
                <div class="typingindicator-floating-container">
                    <p class="typingindicator-floating">typing</p>
                </div>
            {/if}
        </div>
        <div class="buttons">
            {#if $configStore.audio.usePreviewOutput}
                <ClickyButton
                    onclick={sendPreviewMessage}
                    loading={isLoadingPreview}
                    active={buttonIsDownPreview}
                    color="var(--color-surface2)"
                >
                    <IconHeadphones height={24} width={24}/>
                    <span class="action">preview</span>
                </ClickyButton>
            {/if}

            <div
                style={!$configStore.audio.usePreviewOutput
                    ? "grid-column-start: 1; grid-column-end: 3;"
                    : ""}
            >
                <ClickyButton
                    onclick={sendMessage}
                    loading={isLoadingTts}
                    active={buttonIsDown}
                >
                    <IconEnter height={24} width={24} />
                    <span class="action">say</span>
                </ClickyButton>
            </div>
            <ClickyButton
                onclick={async () => {
                  await invoke("audio_stop_all")
                }}
                color="var(--color-danger)"
            >
                <IconStop height={24} width={24}/>
                <span class="action">stop</span>
            </ClickyButton>
        </div>

        <div class="history">
            <div class="history-side">
                <span class="vr"></span>
                <h2 class="history-title">History</h2>
                <span class="vr"></span>
            </div>
            <div class="history-items">
                {#if $historyStore.length < 1}
                    <span class="history-empty"
                        >Say something, and it'll show up here!</span
                    >
                {/if}
                {#each $historyStore as item}
                    <HistoryItem
                        onclick={() => {
                            message = item;
                        }}>{item}</HistoryItem
                    >
                {/each}
            </div>
        </div>
    </div>

    <!--svelte-ignore a11y_no_static_element_interactions -->
    <div class="divider"
        onmousedown={() => { resizeBar = true }}
        >
        <span class="vr"></span>
    </div>
    <div class="app-right">
        {#if $ttsStore.providerId}
            <Voicebank
                voiceName={$ttsStore.voice.name}
                provider={resolveProvider($ttsStore.providerId).name}
                cloud={resolveProvider($ttsStore.providerId).cloud}
            />

            <StepToggle
                majStep={5}
                minStep={1}
                initial={0}
                min={-64}
                max={100}
                bind:value={$ttsStore.pitch}
            >
                <IconPitch width={24} height={24} />
                <h2>Pitch</h2>
            </StepToggle>

            {#if provider.supported_features.includes("Rate")}
                <StepToggle
                    initial={0}
                    majStep={1}
                    minStep={0.5}
                    min={-8}
                    max={8}
                    bind:value={$ttsStore.rate}
                >
                    <IconRate width={24} height={24} />
                    <h2>Rate</h2>
                </StepToggle>
            {/if}

            <Checkbox bind:checked={$configStore.audioTypingIndicator}>
                Audible typing indicator
            </Checkbox>
        {:else}
            <LoadingSpinner />
        {/if}

        <hr/>

        <SplitMenus/>

        <!-- <SidebarItem title="Debug">
             -->

        <div class="bottom">
            <StatusBar/>
        </div>
    </div>
</main>

<style>
    .app-container {
        display: grid;
        grid-template-columns: minmax(0, 3fr) auto auto;
        grid-template-rows: 1fr;
        min-height: 100vh;
        max-height: 100vh;

        min-width: 100vw;
        height: 100%;
        width: 100%;

        gap: 4px;
        padding: 12px;
        background: var(--color-bg);

        overflow: hidden;
    }

    .divider {
        padding: 0 4px;
        display: flex;

        &:hover {
            cursor: w-resize;
            .vr {
                border-right-color: var(--color-surface2);
            }
        }

        .vr {
            width: 1px;
            height: 100%;

            border-right: 1px transparent solid;
        }
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
            outline-color: color-mix(
                in srgb,
                var(--color-surface0) 80%,
                #fff 20%
            );
        }
        100% {
            outline-color: var(--color-surface0);
        }
    }div

    .talkbox-container {
        width: 100%;
        height: 100%;
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
        transition: outline-width 0.1s var(--ease-out-expo);
    }

    #talkbox:focus {
        outline-offset: 0;
        animation: flash 0.4s;
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

        max-height: 100vh;

        width: var(--sidebar-width);

        overflow-y: auto;

        .bottom {
            margin-top: auto;
        }

    }

    /* compact mode */
    @media screen and (max-width: 720px) {
        .app-left {
            grid-column-start: 1;
            grid-column-end: 3;
        }
        .app-right {
            display: none;
        }
        .divider {
            display: none;
        }
    }

    .buttons {
        display: grid;
        grid-template-columns: 128px 1fr 128px;
        gap: 8px;
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

            .vr {
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
            opacity: 0.35;
            font-style: italic;
        }
    }

    .typingindicator-floating-container {
        position:relative;
    }

    @keyframes typingindicator-anim {
        0% {
            --gradient-alpha: 0.5;
        }
        15% {
            --gradient-percent: 0%;
        }

        50% {
            --gradient-alpha: 1.0;
        }

        100% {
            --gradient-percent: 150%;
            --gradient-alpha: 0.5;
        }
    }

    @property --gradient-percent {
      syntax: "<percentage>";
      inherits: false;
      initial-value: 0%;
    }


    @property --gradient-alpha {
      syntax: "<number>";
      inherits: false;
      initial-value: 0;
    }

    .typingindicator-floating {
        --gradient-percent: 0%;
        position: absolute;
        right: 16px;
        bottom: 16px;
        font-size: .9rem;

        background: linear-gradient(90deg,
            rgba(255,255,255,0.5),
            rgba(255,255,255,var(--gradient-alpha)) var(--gradient-percent),
            rgba(255,255,255,0.5)
        );
        background-size: 200%;
        background-position: center;
        background-clip: text;
        -webkit-text-fill-color: transparent;

        animation: 1.4s typingindicator-anim infinite linear;
    }

    .quicksettings {
        padding: 8px;
        border: 1px var(--color-surface0) solid;
        border-radius: var(--rounding);
    }
</style>
