<script lang="ts">
    import { _ } from "svelte-i18n";
    import { expoOut } from "svelte/easing";
    import { fly } from "svelte/transition";
    import { resolveProvider, ttsStore, type Provider } from "@bird/lib/bird";
    import { configStore } from "@bird/lib/config";
    import { tooltip } from "@svelte-plugins/tooltips";
    import { invoke } from "@tauri-apps/api/core";

    import StepToggle from "@bird/components/StepToggle.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import Voicebank from "@bird/components/Voicebank.svelte";

    import AddFavourite from "@bird/routes/screens/add-favourite.svelte";

    import IconFavourite from "@bird/assets/icons/IconFavourite.svelte";
    import IconPitch from "@bird/assets/icons/IconPitch.svelte";
    import IconRate from "@bird/assets/icons/IconRate.svelte";
    import IconAudibleTypingIndicatorOn from "@bird/assets/icons/IconAudibleTypingIndicatorOn.svelte";
    import IconAudibleTypingIndicatorOff from "@bird/assets/icons/IconAudibleTypingIndicatorOff.svelte";
    import IconOsc from "@bird/assets/icons/IconOsc.svelte";
    import IconSettings from "@bird/assets/icons/IconSettings.svelte";

    let isMinimised = $state(false);
    let showAddFavourite = $state(false);

</script>

<div class="voicebank-top">
    <Voicebank
        voiceName={$ttsStore.voice.name}
        provider={resolveProvider($ttsStore.voice.provider).name}
        cloud={resolveProvider($ttsStore.voice.provider).cloud}
        onclick={() => isMinimised = !isMinimised}
        expanded={!isMinimised}
    />
    <div class="voicebank-action" use:tooltip={{
        content: $_("favourite.favourite"),
        position: 'left',
        delay: 0
      }}>
          <Button
              class="btn-normal"
              onclick={() => showAddFavourite = true}>
              <IconFavourite/>
          </Button>
    </div>
    {#if showAddFavourite}
        <AddFavourite onClose={() => showAddFavourite = false}/>
    {/if}
</div>
{#if !isMinimised}
    <div class="controls" in:fly={{y: -8, duration: 200, easing: expoOut }}>
        <StepToggle
            majStep={5}
            minStep={1}
            initial={0}
            min={-64}
            max={100}
            bind:value={$ttsStore.pitch}
        >
            <IconPitch width={24} height={24} />
            <h2>{$_('voiceEditor.features.pitch')}</h2>
        </StepToggle>

        {#if resolveProvider($ttsStore.voice.provider).supported_features.includes("Rate")}
            <StepToggle
                initial={0}
                majStep={1}
                minStep={0.5}
                min={-8}
                max={8}
                bind:value={$ttsStore.rate}
            >
                <IconRate width={24} height={24} />
                <h2>{$_('voiceEditor.features.rate')}</h2>
            </StepToggle>
        {/if}

        <div class="actions-container">
            <div class="actions">
                <div use:tooltip={{ content: $_('settings.behaviour.audibleTypingIndicator'), position: 'top', animation: "pop", delay: 0, align: "center"}}>
                    <button class="action-button {($configStore.audioTypingIndicator) ? 'on' : ''}" onclick={() => {
                      $configStore.audioTypingIndicator = !$configStore.audioTypingIndicator
                    }}>
                        {#if $configStore.audioTypingIndicator}
                            <IconAudibleTypingIndicatorOn height="24px" width="48px"/>
                        {:else}
                            <IconAudibleTypingIndicatorOff height="24px" width="48px"/>
                        {/if}
                    </button>
                </div>

                <div use:tooltip={{ content: 'VRChat OSC', animation: "pop", position: 'top', delay: 0, align: "center"}}>
                    <button class="action-button {($configStore.vrcOsc) ? 'on' : ''}" onclick={async () => {
                      $configStore.vrcOsc = !$configStore.vrcOsc
                      if($configStore.vrcOsc) {
                        await invoke("osc_start");
                      } else {
                        await invoke("osc_stop");
                      }
                    }}>
                        <IconOsc height="24px" width="48px"/>
                    </button>
                </div>
            </div>
        </div>

        <hr/>

    </div>
{/if}


<style>
    .voicebank-top {
        display: grid;

        grid-template-columns: auto 32px;
        grid-template-rows: auto auto;

        gap: 0 8px;

        :global(.voicebank-action) {
            width: 100%;
            padding: 0;
            display: flex;
            align-items: center;
            justify-content: center;

            :global(button) {
                width: 100%;
                height: 100%;
                padding: 0;
                display: flex;
                align-items: center;
                justify-content: center;
                border-radius: var(--rounding);

                background-color: var(--color-surface1);

                :global(svg) {
                    height: 20px;
                    width: 20px;
                }
            }
        }
    }

    .actions-container {
        display: flex;
        justify-content: space-around;
        align-items: center;

        h2 {
            font-weight: 400;
            font-size: 0.95rem;

            display: flex;
            justify-content: center;
            align-items: center;
            gap: 8px;
        }
    }

    .actions {
        display: flex;
        justify-content: center;
        gap: 16px;
        align-items: center;
    }

    .action-button {
        background: transparent;
        border: none;
        color: var(--color-text);
        cursor: pointer;

        display: flex;
        justify-content: center;
        align-items: center;
        padding: 2px;

        border: 1px var(--color-surface1) solid;
        border-radius: var(--rounding);

        transition: border-color 0.15s var(--ease-out-expo);

        :global(svg) {
            pointer-events: none;
            user-select: none;
        }
    }

    .action-button.on {
        border: 1px var(--color-accent) solid;
    }

    .controls {
        display: flex;
        flex-direction: column;
        gap: 8px;
        padding: 2px 0;
    }
</style>
