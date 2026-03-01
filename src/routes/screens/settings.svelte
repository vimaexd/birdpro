<script lang="ts">
    /* TODO: Refactor this to new Modal component */

    import SettingsAudio from "./settings-audio.svelte";
    import SettingsAbout from "./settings-about.svelte";
    import SettingsIntegrations from "./settings-integrations.svelte";
    import SettingsProviders from "./settings-providers.svelte";
    import SettingsBehaviour from "./settings-behaviour.svelte";
    import ModalContainer from "@bird/components/alert/ModalContainer.svelte";
    import ModalCloseButton from "@bird/components/alert/ModalCloseButton.svelte";
    import { disableInputCapture } from "@bird/lib/modal";
    import { onMount } from "svelte";
    import SettingsReplacements from "./settings-replacements.svelte";

    let selectedPage = $state("Audio");
    const pages: {[id: string]: any} = {
      "Audio": SettingsAudio,
      "Behaviour": SettingsBehaviour,
      // "Replacements": SettingsReplacements,
      "Providers": SettingsProviders,
      "Integrations": SettingsIntegrations,
      "About": SettingsAbout,
    }

    let currentScreen = $derived(pages[selectedPage]);

    let {
      onClose
    } = $props<{
      onClose: () => any;
    }>();

    let doClose = () => {
      disableInputCapture.set(false);
      onClose();
    }

    onMount(() => {
      disableInputCapture.set(true);
    })

    document.body.addEventListener('keydown', (e) => {
      if(e.key == "Escape") {
        onClose()
      }
    })
</script>

<ModalContainer>
    <div class="settings">
        <ModalCloseButton onclick={doClose}/>
        <h1>Settings</h1>
        <div class="categories">
            {#each Object.keys(pages) as page}
                <button
                    class={(selectedPage === page) ? "selected" : ""}
                    onclick={() => {
                      selectedPage = page
                    }}
                >{page}</button>
            {/each}
        </div>
        <div class="settings-content">
            <h2>{selectedPage}</h2>
            {@render currentScreen()}
        </div>
    </div>
</ModalContainer>

<style lang="postcss">
    @keyframes settings-in {
        0% {
            transform: scale(0.98);
            opacity: 0;
        }

        100% {
            opacity: 1;
            transform: scale(1);
        }
    }

    .settings {
        animation: settings-in .5s var(--ease-out-expo);
        position: relative;

        display: grid;
        grid-template-rows: 48px 1fr;
        grid-template-columns: 180px 1fr;
        grid-auto-flow: column;

        gap: 0 24px;

        width: 900px;
        height: 600px;
        background-color: var(--color-bg);
        border-radius: var(--rounding);
        padding: 24px;

        box-shadow: 0 2px 24px rgba(0,0,0,0.6);


        h1 {
            font-size: 1.5rem;
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

        & button {
            color: var(--color-text);
            text-decoration: none;
            padding: 8px 12px;
            font-size: .9rem;
            font-family: var(--font-family);
            border-radius: var(--rounding);
            border: none;
            background: transparent;
            text-align: left;

            cursor: pointer;

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
