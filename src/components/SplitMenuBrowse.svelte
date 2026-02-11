<script lang="ts">
    import SelectList from '@bird/components/ui/SelectList.svelte';
    import SelectListOption from './ui/SelectListOption.svelte';
    import SvelteVirtualList from '@humanspeak/svelte-virtual-list';
    import IconCloud from '@bird/assets/icons/IconCloud.svelte';
    import Button from './ui/Button.svelte';
    import { resolveProvider, ttsProviders, ttsStore, type Voice, setProvider, setVoice, getErrorText } from '@bird/lib/bird';
    import { invoke } from '@tauri-apps/api/core';
    import LoadingSpinner from './LoadingSpinner.svelte';
    import { showError } from '@bird/lib/toast';
    import { onMount } from 'svelte';

    let provider = $state<string>($ttsStore.providerId);
    let ttsVoices = $state<Voice[]>([]);
    let selectedVoice = $state("");

    const updateVoices = async () => {
      try {
        ttsVoices = await invoke("tts_get_voicelist", { providerId: provider })
      } catch(e: any) {
        let err = await getErrorText(e);
        showError(e, err);
      }
    }

    onMount(() => {
      updateVoices();
    })
</script>

<h2>Provider</h2>
<SelectList
    bind:value={provider}
    onChange={() => {
      ttsVoices = []
      selectedVoice = ""
    }}
    height="fit-content"
>
    {#each $ttsProviders as provider}
        <SelectListOption value={provider.id}>
            {provider.name}
            {#if provider.cloud}
                <IconCloud width="16px" height="16px" />
            {/if}
        </SelectListOption>
    {/each}
</SelectList>


<h2>Voice</h2>
<SelectList
    bind:value={selectedVoice}
    height="100%"
>
    {#await updateVoices() }
        <div class="selectlist-loading-container">
            <LoadingSpinner/>
        </div>
    {:then voices}
        <SvelteVirtualList items={ttsVoices as Voice[]}>
            {#snippet renderItem(voice)}
                <SelectListOption value={voice.id}>
                    {voice.name}
                </SelectListOption>
            {/snippet}
        </SvelteVirtualList>
    {/await}
</SelectList>

<div class="buttons">
    <!-- {#if selectedVoice == ""} -->
    <Button disabled={selectedVoice == ""} type="accent"
        onclick={() => {
          if(selectedVoice == "") return;
          $ttsStore.providerId = provider;
          $ttsStore.voice = ttsVoices.find((v: Voice) => (v.id == selectedVoice))!;
        }}
    >Use Voice
    </Button>
    <!-- {/if} -->
</div>

<style>
    .selectlist-loading-container {
        display: flex;
        width: 100%;
        justify-content: center;
        padding: 8px;
    }

    .buttons {
        display: flex;
        justify-content: end;
    }

    .provider-picker {
        display: flex;
        gap: 8px;
        align-items: center;
        width: 100%;

        & :global(ul) {
            width: 100%;
        }
    }
    h2 {
      font-size: 0.95rem;
      font-weight: 600;
    }
</style>
