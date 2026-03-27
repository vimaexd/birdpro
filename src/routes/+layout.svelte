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
    import { showError } from "@bird/lib/toast";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    let ready = $state(false);

    // used to set actual theme - can NOT have auto
    let theme = $state("dark");

    // used to compare - can have auto
    let previousTheme = "dark";

    onMount(async () => {
        // avoid white flash on startup
        // by showing the window after we're ready
        getCurrentWindow().show();

        try {
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
            showError("Startup Error", e);
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
            in:fade={{ duration: 500 }}
            out:fade={{ duration: 100 }}
        >
            <img src={logo} height="128px" width="128px" alt="Bird Pro logo" />
            <div class="spinner">
                <LoadingSpinner />
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

    @keyframes spinner-in {
        0% {
            opacity: 0;
        }
        100% {
            opacity: 1;
        }
    }

    .spinner {
        position: absolute;
        bottom: 64px;
        opacity: 0;
        will-change: opacity;
        animation: spinner-in 1s 0.5s forwards;
    }
</style>
