<script lang="ts">
    import SelectList from '@bird/components/ui/SelectList.svelte';
    import SelectListOption from '@bird/components/ui/SelectListOption.svelte';
    import SvelteVirtualList from '@humanspeak/svelte-virtual-list';
    import IconCloud from '@bird/assets/icons/IconCloud.svelte';
    import Button from '@bird/components/ui/Button.svelte';
    import { ttsProviders, ttsStore, type Voice, getErrorText } from '@bird/lib/bird';
    import { invoke } from '@tauri-apps/api/core';
    import LoadingSpinner from '@bird/components/LoadingSpinner.svelte';
    import { showError } from '@bird/lib/toast';
    import { onMount } from 'svelte';
    import { get } from 'svelte/store';

    let provider = $state<string>($ttsStore.providerId);
    let ttsVoices = $state<Voice[]>([]);
    let justApplied = $state(false);
    let selectedVoice = $state("");

    let enableUseButton = $derived(!justApplied && selectedVoice !== "")

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
    onChange={() => {
      justApplied = false
    }}
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
    <Button style="width: 100%;justify-content: center;" disabled={!enableUseButton} type="accent"
        onclick={() => {
          if(!enableUseButton) return;
          let ttsStoreCopy = get(ttsStore);
          ttsStoreCopy.providerId = provider;
          ttsStoreCopy.voice = JSON.parse(JSON.stringify(ttsVoices.find((v: Voice) => (v.id == selectedVoice))!));
          ttsStore.set(ttsStoreCopy);
          justApplied = true;
        }}
    >Use Voice
    </Button>
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

    h2 {
      font-size: 0.95rem;
      font-weight: 600;
      user-select: none;
    }
</style>
