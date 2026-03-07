<script lang="ts">
    import { expoOut } from "svelte/easing";
    import { fly } from "svelte/transition";
    import { resolveProvider, ttsStore, type Provider } from "@bird/lib/bird";
    import { configStore } from "@bird/lib/config";

    import StepToggle from "@bird/components/StepToggle.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import Voicebank from "@bird/components/Voicebank.svelte";

    import AddFavourite from "@bird/routes/screens/add-favourite.svelte";

    import IconFavourite from "@bird/assets/icons/IconFavourite.svelte";
    import IconPitch from "@bird/assets/icons/IconPitch.svelte";
    import IconRate from "@bird/assets/icons/IconRate.svelte";

    let provider: Provider = $derived.by(() => {
      return resolveProvider($ttsStore.voice.provider)
    });

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
    <Button class="btn-normal voicebank-action" onclick={() => showAddFavourite = true}>
        <IconFavourite/>
    </Button>
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
            <h2>Pitch</h2>
        </StepToggle>

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

        <Checkbox bind:checked={$configStore.audioTypingIndicator}>
            Audible typing indicator
        </Checkbox>

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

            background-color: var(--color-surface1);

            :global(svg) {
                /*color: var(--color-accent);*/
                height: 20px;
                width: 20px;
            }
        }
    }

    .controls {
        display: flex;
        flex-direction: column;
        gap: 8px;
        padding: 2px 0;
    }
</style>
