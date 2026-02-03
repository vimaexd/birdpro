<script lang="ts">
    import type { Snippet } from "svelte";
    import SettingsAudio from "./settings-audio.svelte";
    import SettingsPlaceholder from "./settings-placeholder.svelte";
    import SettingsAbout from "./settings-about.svelte";


    let selectedPage = $state("Audio");
    const pages: {[id: string]: any} = {
      "Audio": SettingsAudio,
      "Behaviour": SettingsPlaceholder,
      "Providers": SettingsPlaceholder,
      "About": SettingsAbout,
    }

    let currentScreen = $derived(pages[selectedPage]);


    let {
      onClose
    } = $props<{
      onClose: () => any;
    }>();

    document.body.addEventListener('keydown', (e) => {
      if(e.key == "Escape") {
        onClose()
      }
    })
</script>

<div class="settings-screen">
    <div class="settings">
        <button class="close-btn" onclick={onClose}>×</button>
        <h1>Settings</h1>
        <div class="categories">
            {#each Object.keys(pages) as page}
                <a href="#"
                    class={(selectedPage == page) ? "selected" : ""}
                    onclick={() => {
                      selectedPage = page
                    }}
                >{page}</a>
            {/each}
        </div>
        <div class="settings-content">
            <h2>{selectedPage}</h2>
            {@render currentScreen()}
        </div>
    </div>
</div>

<style>
    .settings-screen {
        padding: 0;
        margin: 0;
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100vh;
        background-color: rgba(0,0,0,0.40);
        backdrop-filter: blur(6px) brightness(0.80);
        z-index: 10;
        overflow: hidden;

        display: flex;
        justify-content: center;
        align-items: center;

        user-select: none;
    }

    .settings {
        position: relative;

        display: grid;
        grid-template-rows: 48px 1fr;
        grid-template-columns: 180px 1fr;
        grid-auto-flow: column;

        gap: 0px 16px;

        width: 800px;
        height: 450px;
        background-color: var(--color-bg);
        border-radius: var(--rounding);
        padding: 16px;

        box-shadow: 0px 2px 24px rgba(0,0,0,0.6);


        h1 {
            font-size: 1.5rem;
        }
    }

    .close-btn {
        position: absolute;
        top: 10px;
        right: 10px;
        background: none;
        border: none;
        color: var(--color-text);
        font-size: 1.4rem;
        line-height: 1;
        width: 28px;
        height: 28px;
        border-radius: var(--rounding);
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        &:hover {
            background-color: var(--color-surface0);
        }
        &:active {
            filter: brightness(0.8);
        }
    }

    .settings-content {
        display: flex;
        flex-direction: column;
        gap: 16px;

        grid-row: 1 / 3;

    }

    .categories {
        display: flex;
        flex-direction: column;
        gap: 4px;

        & a {
            color: var(--color-text);
            text-decoration: none;
            padding: 6px 12px;
            font-size: .9rem;
            border-radius: var(--rounding);

            user-select: none;

            transition: filter, background-color .2s var(--ease-out-expo);

            &:hover {
                background-color: color-mix(in srgb, var(--color-surface0) 50%, transparent 50%);
            }

            &:active {
                filter: brightness(0.8);
            }

            &.selected {
                background-color: var(--color-surface0);
            }
        }
    }
</style>
