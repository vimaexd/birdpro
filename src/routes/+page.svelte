<script lang="ts">
    import {
        speakTts,
        ttsStore,
        resolveProvider,
        type Provider,
    } from "@bird/lib/bird";
    import { configStore } from "@bird/lib/config";
    import {
        getLastMessage,
        historyStore,
        pushHistory,
    } from "@bird/lib/history";
    import { disableInputCapture, isSettingsOpen } from "@bird/lib/modal";
    import { setTextTypingIndicator } from "@bird/lib/txtoutput";

    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { info } from "@tauri-apps/plugin-log";
    import { platform } from "@tauri-apps/plugin-os"
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";
    import { _ } from "svelte-i18n";

    import AddFavourite from "./screens/add-favourite.svelte";
    import Settings from "./screens/settings.svelte";

    import SplitMenus from "@bird/components/feat/splitmenu/SplitMenus.svelte";
    import StatusBar from "@bird/components/StatusBar.svelte";
    import ClickyButton from "@bird/components/ClickyButton.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import LoadingSpinner from "@bird/components/LoadingSpinner.svelte";

    import IconStop from "@bird/assets/icons/IconStop.svelte";
    import IconHeadphones from "@bird/assets/icons/IconHeadphones.svelte";
    import IconEnter from "@bird/assets/icons/IconEnter.svelte";
    import VoiceEditor from "@bird/components/feat/sidebar/VoiceEditor.svelte";
    import HistoryContainer from "@bird/components/feat/history/HistoryContainer.svelte";
    import Onboarding from "./screens/onboarding.svelte";
    import { get } from "svelte/store";

    // talk box and messaging
    let talkboxRef: HTMLTextAreaElement;
    let buttonIsDown = $state(false);
    let buttonIsDownPreview = $state(false);
    let message = $state("");

    // loading states for main buttons
    let isLoadingTts = $state(false);
    let isLoadingPreview = $state(false);

    // app sizebar sizing
    let barSize = $state(380);
    let resizeBar = $state(false);

    // typing indicator
    const typingIndicatorTimeoutSeconds = 2500;
    let typingIndicatorLastLength = 0;
    let typingIndicatorShowing = $state(false);
    let typingIndicatorTimeout: number;

    const onTyping = async () => {
        // regardless of whever this is already showing, send typing over OSC
        if ($configStore["vrcOsc"]) {
            await invoke("osc_typing_indicator", { typing: true });
        }

        // if the indicator is already showing, extend the timeout by 4s
        if (typingIndicatorShowing) {
            clearTimeout(typingIndicatorTimeout);
            typingIndicatorTimeout = setTimeout(
                onTypingTimeout,
                typingIndicatorTimeoutSeconds,
            );
            return;
        }
        // prevent this from being called until we time out

        typingIndicatorShowing = true;

        info("Showing typing indicator");

        // set osc and txt typing indicators
        if (
            $configStore["txtoutput"] &&
            $configStore["txtoutput.typingIndicator"]
        ) {
            await setTextTypingIndicator(
                $configStore["txtoutput.typingIndicatorText"],
            );
        }

        // set audio typing indicator
        if ($configStore["audioTypingIndicator"]) {
            await invoke("audio_typingindicator_start");
        }

        typingIndicatorTimeout = setTimeout(
            onTypingTimeout,
            typingIndicatorTimeoutSeconds,
        );
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
        if (typingIndicatorTimeout) {
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
        if (!resizeBar) return;
        console.log("rsz");
        let newWidth = window.innerWidth - e.pageX;
        barSize = Math.min(Math.max(newWidth, 300), 700) - 18;
    };

    onMount(async () => {
        await getCurrentWindow().listen("tauri://focus", () => {
            if (!$isSettingsOpen) {
                focusTextbox();
            }
        });

        // process hotkeys
        document.body.addEventListener("keydown", async (e) => {
            // dont capture strokes if a modal is shown
            // the user needs that to type stuff!!!

            if ($disableInputCapture) {
                return;
            }


            // Checks for CMD on macOS, and CTRL for Linux and Windows
            let platAwareControl =
                platform() == "macos" ? e.metaKey : e.ctrlKey;

            switch (e.key) {

                /*
                    Ctrl ,
                    Open settings
                */
                case ",":
                    if (platAwareControl) {
                        $isSettingsOpen = true;
                    }
                    break;

                /*
                    Ctrl =
                    Increase font size
                */
                case "=":
                    if (platAwareControl) {
                        console.log("add")

                        let cs = get(configStore);
                        if(cs['ui.textboxTextSize'] > 77) return;

                        cs['ui.textboxTextSize'] += 2;
                        configStore.set(cs);
                        console.log(cs['ui.textboxTextSize'])
                    }
                    break;

                /*
                    Ctrl -
                    Decrease font size
                */
                case "-":
                    if (platAwareControl) {
                        console.log("minus")

                        let cs = get(configStore);
                        if(cs['ui.textboxTextSize'] < 13) return;

                        cs['ui.textboxTextSize'] -= 2;
                        configStore.set(cs);
                    }
                    break;


                /*
                    Up arrow
                    Fill textbox with last history item
                */
                case "ArrowUp":
                    e.preventDefault();
                    if (message == "") {
                        message = getLastMessage();
                    }
                    focusTextbox();
                    break;

                /*
                    Enter
                    Speak message
                */
                case "Enter":
                    e.preventDefault();
                    if (e.altKey && $configStore.audio.usePreviewOutput) {
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

                /*
                    Del
                    Stop all speech
                */
                case "Delete":
                    await invoke("audio_stop_all");
                    break;

                default:
                    focusTextbox();
            }
        });

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
<main
    class="app-container"
    style="
    --sidebar-width: {barSize}px;
    --rounding: {$configStore['ui.rounding']}px;
    --color-accent: {$configStore['ui.accentColor']};"
    onmousemove={trackMouseAndResizeBar}
    onmouseup={() => {
        resizeBar = false;
    }}
    in:fade={{ duration: 300 }}
>
    {#if !$configStore.onboarded}
        <Onboarding />
    {/if}

    {#if $isSettingsOpen}
        <Settings onClose={() => ($isSettingsOpen = false)} />
    {/if}

    <div class="app-left">
        <div class="talkbox-container">
            <textarea
                id="talkbox"
                placeholder={$_("talkbox.placeholder")}
                bind:value={message}
                bind:this={talkboxRef}
                oninput={(e: any) => {
                    // don't count deleting as typing
                    if (typingIndicatorLastLength < e.target.value.length + 1) {
                        onTyping();
                    }
                    typingIndicatorLastLength = e.target.value.length;
                }}
                onclick={() => {
                    $disableInputCapture = false;
                }}
                maxlength={$configStore["bypassCharLimit"] ? 99999 : 144}
                style="--talkbox-text-size: {$configStore['ui.textboxTextSize']}px;"
            >
            </textarea>
            <div class="talkbox-corner">
                <div class="talkbox-corner-inner">
                    {#if typingIndicatorShowing}
                        <p
                            class="typingindicator-floating"
                            out:fade={{ duration: 150 }}
                        >
                            {$_("talkbox.typingIndicator")}
                        </p>
                    {/if}
                    <p class="char-count">
                        {message.length}/{$configStore["bypassCharLimit"]
                            ? "∞"
                            : "144"}
                    </p>
                </div>
                <div class="talkbox-corner-inner-bl">
                    {#if message.length > 0}
                        <div
                            in:fade={{ duration: 100 }}
                            out:fade={{ duration: 100 }}
                        >
                            <Button
                                onclick={() => {
                                    message = "";
                                }}>{$_("talkbox.clear")}</Button
                            >
                        </div>
                    {/if}
                </div>
            </div>
        </div>
        <div class="buttons">
            {#if $configStore.audio.usePreviewOutput}
                <ClickyButton
                    onclick={sendPreviewMessage}
                    loading={isLoadingPreview}
                    active={buttonIsDownPreview}
                    color="var(--color-surface2)"
                >
                    <IconHeadphones height={24} width={24} />
                    <span class="action">{$_("talkbox.actionPreview")}</span>
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
                    <span class="action">{$_("talkbox.actionSay")}</span>
                </ClickyButton>
            </div>
            <ClickyButton
                onclick={async () => {
                    await invoke("audio_stop_all");
                }}
                color="var(--color-danger)"
            >
                <IconStop height={24} width={24} />
                <span class="action">{$_("talkbox.actionStop")}</span>
            </ClickyButton>
        </div>

        {#if $configStore["ui.showHistory"]}
            <HistoryContainer
                onSelectMessage={(msg: string) => {
                    message = msg;
                }}
            />
        {/if}
    </div>

    <!--svelte-ignore a11y_no_static_element_interactions -->
    <div
        class="divider"
        onmousedown={() => {
            resizeBar = true;
        }}
    >
        <span class="vr"></span>
    </div>
    <div class="app-right">
        {#if $ttsStore.voice.provider}
            <VoiceEditor />
        {:else}
            <LoadingSpinner />
        {/if}

        <SplitMenus />

        <div class="bottom">
            <StatusBar />
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

        gap: 2px;
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
    }
    div .talkbox-container {
        width: 100%;
        height: 100%;
    }

    #talkbox {
        font-size: var(--talkbox-text-size);
        font-family: var(--font-family);
        border-radius: var(--rounding);
        padding: 12px;
        width: 100%;
        height: 100%;
        background: transparent;
        border: none;
        outline: 1px var(--color-surface1) solid;
        resize: none;

        will-change: outline-width;
        transition:
            outline-width 0.1s var(--ease-out-expo),
            font-size 0.2s var(--ease-out-expo);
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

    .talkbox-corner {
        position: relative;

        .talkbox-corner-inner {
            position: absolute;
            right: 16px;
            bottom: 16px;
            font-size: 0.9rem;

            display: flex;
            flex-direction: row;
            align-items: end;
            gap: 2px;

            text-align: right;
        }

        .talkbox-corner-inner-bl {
            position: absolute;
            left: 16px;
            bottom: 16px;
            font-size: 0.9rem;

            display: flex;
            flex-direction: row;
            align-items: end;
            gap: 2px;

            text-align: right;

            :global(button) {
                opacity: 0.8;
                font-size: 0.8rem;
                background-color: var(--color-surface0);
                color: var(--color-text);
                padding: 2px 4px;

                &:hover {
                    background-color: var(--color-surface0);
                }
            }
        }
    }

    .char-count {
        opacity: 0.5;
        font-weight: 300;
        width: 54px;
    }

    @keyframes typingindicator-anim {
        0% {
            --gradient-alpha: 50%;
        }
        15% {
            --gradient-percent: 0%;
        }

        50% {
            --gradient-alpha: 100%;
        }

        100% {
            --gradient-percent: 150%;
            --gradient-alpha: 50%;
        }
    }

    @property --gradient-percent {
        syntax: "<percentage>";
        inherits: false;
        initial-value: 0%;
    }

    @property --gradient-alpha {
        syntax: "<percentage>";
        inherits: false;
        initial-value: 0%;
    }

    .typingindicator-floating {
        --gradient-percent: 0%;

        background: linear-gradient(
            90deg,
            color-mix(in srgb, var(--color-text), transparent 50%),
            color-mix(
                    in srgb,
                    var(--color-text) calc(var(--gradient-alpha)),
                    transparent calc(100% - var(--gradient-alpha))
                )
                var(--gradient-percent),
            color-mix(in srgb, var(--color-text), transparent 50%)
        );
        background-size: 200%;
        background-position: center;
        background-clip: text;
        -webkit-text-fill-color: transparent;

        animation: 1.4s typingindicator-anim infinite linear;
    }
</style>
