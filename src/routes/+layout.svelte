<script lang="ts">
    import "../assets/main.css";
    import "../assets/switzer.css";

    import ToastContainer from "@bird/components/alert/ToastContainer.svelte";
    const { children } = $props();

    import { onMount } from "svelte";
    import { fade } from "svelte/transition";
    import { configStore, initialiseConfig } from "@bird/lib/config";
    import { initialiseApp } from "@bird/lib/bird";
    import LoadingSpinner from "@bird/components/LoadingSpinner.svelte";
    import logo from "@bird/assets/img/birdpro-logo.png";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { _ } from "svelte-i18n";

    import { register as registerLang, init as initLang, init, getLocaleFromNavigator } from 'svelte-i18n';
    import WarningWithOutline from "@bird/assets/icons/twemoji/WarningWithOutline.svelte";
    registerLang('en', () => import('@bird/lang/en.json'));

    let ready = $state(false);
    let error = $state("");

    // used to set actual theme - can NOT have auto
    let theme = $state("dark");

    // used to compare - can have auto
    let previousTheme = "dark";

    onMount(async () => {
        // avoid white flash on startup
        // by showing the window after we're ready
        getCurrentWindow().show();

        try {
            initLang({
                fallbackLocale: 'en',
                initialLocale: 'en'
            })

            await initialiseConfig();
            await initialiseApp();
            ready = true;

            // change theme when theme store changes
            configStore.subscribe(async (c) => {
                if (previousTheme !== c["ui.theme"]) {
                    if(c["ui.theme"] == "auto") {
                        theme = await getCurrentWindow().theme() || "dark";
                    } else {
                        theme = c["ui.theme"];
                    }
                    previousTheme = c["ui.theme"]
                }
            });

            // update auto theme
            getCurrentWindow().onThemeChanged((newTheme) => {
                if($configStore["ui.theme"] == "auto") {
                    theme = newTheme.payload;
                }
            })

        } catch (e: any) {
            setTimeout(() => {
                // honestly it looks nicer if it looks like the app
                // *tried* to load, launching straight into an error is
                // very frustrating
                error = e;
            }, 500)
        }
    });
</script>

<div class="wrapper theme-{theme}">
    <ToastContainer />

    {#if ready}
        {@render children()}
    {:else}
        <div
            class="loader"
            in:fade={{ duration: 300 }}
            out:fade={{ duration: 100 }}
        >
            <img src={logo} height="128px" width="128px" alt="Bird Pro logo" />
            {#if error != ""}
                <div class="loader-erroricon-container" in:fade={{ duration: 50 }}>
                    <WarningWithOutline/>
                </div>
            {/if}
            <div class="loader-bottom">
                {#if !error}
                    <div class="loader-spinner">
                        <LoadingSpinner />
                    </div>
                {:else}
                    <div class="loader-crashhandler" in:fade={{ duration: 100 }}>
                        <h2>{$_("error.startupError")}</h2>
                        <p>
                            {$_("error.startupErrorExplainer")}
                        </p>

                        <code>
                            {error}
                        </code>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .wrapper {
        background-color: var(--color-bg);
        min-height: 100vh;
    }

    .loader {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100vw;
        height: 100vh;

        background-color: var(--color-bg);
        color: var(--color-text);
    }

    .loader-erroricon-container {
        position: absolute;
        :global(svg) {
            width: 64px;

            position: relative;
            left: 32px;
            top: 32px;

            animation: loader-erroricon 0.5s alternate infinite;
        }
    }

    @keyframes loader-erroricon {
        0% {
            filter: brightness(0.5);
        }
        100% {

        }
    }

    .loader-spinner {
        opacity: 0;
        will-change: opacity;
        animation: spinner-in 1s 0.5s forwards;
    }

    @keyframes spinner-in {
        0% {
            opacity: 0;
        }
        100% {
            opacity: 1;
        }
    }

    .loader-bottom {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;

        position: absolute;
        bottom: 12vh;
    }

    .loader-crashhandler {
        text-align: center;
        width: 500px;

        a {
            color: lightskyblue;
        }

        code {
            margin-top: 16px;
            opacity: 0.6;
            font-size: 0.7rem;
        }
    }
</style>
