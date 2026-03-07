<script lang="ts">
    import SelectList from '@bird/components/ui/SelectList.svelte';
    import SelectListOption from '@bird/components/ui/SelectListOption.svelte';
    import SvelteVirtualList from '@humanspeak/svelte-virtual-list';
    import IconCloud from '@bird/assets/icons/IconCloud.svelte';
    import Button from '@bird/components/ui/Button.svelte';
    import { ttsProviders, ttsStore, type Voice, getErrorText, resolveProvider } from '@bird/lib/bird';
    import { invoke } from '@tauri-apps/api/core';
    import LoadingSpinner from '@bird/components/LoadingSpinner.svelte';
    import { showError } from '@bird/lib/toast';
    import { onMount } from 'svelte';
    import { get } from 'svelte/store';
    import NoticeRequiresAuth from './NoticeRequiresAuth.svelte';
    import TextInput from '@bird/components/ui/TextInput.svelte';
    import IconSearch from '@bird/assets/icons/IconSearch.svelte';
    import { disableInputCapture } from '@bird/lib/modal';

    let provider = $state<string>($ttsStore.voice.provider);
    let ttsVoices = $state<Voice[]>([]);
    let ttsVoicesFiltered = $derived(
      ttsVoices.filter(v => v.name.toLowerCase().includes(voiceSearchQuery.toLowerCase().trim()))
    )
    let justApplied = $state(false);
    let selectedVoice = $state("");
    let voiceSearchQuery = $state("")

    let enableUseButton = $derived(!justApplied && selectedVoice !== "");

    let showRequiresAuth = $state(false);

    const updateVoices = async () => {
      try {
        ttsVoices = await invoke("tts_get_voicelist", { providerId: provider })
      } catch(e: any) {
        if(e == "AuthorizationRequired") {
          showRequiresAuth = true
        } else {
          let err = await getErrorText(e);
          showError(e, err);
        }
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
      showRequiresAuth = false
      ttsVoices = []
      selectedVoice = ""
    }}
    height="fit-content"
    shrink="0"
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


<div class="header header-inp">
    <h2>Voice</h2>
    <div>
        {#if voiceSearchQuery.length < 1}
        <div class="header-inp-label-container">
            <div>
                <label for="">
                    <IconSearch width="16px" height="16px"/>
                    Search
                </label>
            </div>
        </div>
        {/if}
        <TextInput bind:value={voiceSearchQuery} onclick={() => disableInputCapture.set(true)}/>
    </div>
</div>
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
        {#if showRequiresAuth}
            <NoticeRequiresAuth providerName={resolveProvider(provider).name}/>
        {:else}
            <SvelteVirtualList items={ttsVoicesFiltered as Voice[]}>
                {#snippet renderItem(voice)}
                    <SelectListOption value={voice.id}>
                        {voice.name}
                    </SelectListOption>
                {/snippet}
            </SvelteVirtualList>
        {/if}
    {/await}
</SelectList>

<div class="buttons">
    <Button style="width: 100%;justify-content: center;" disabled={!enableUseButton} type="accent"
        onclick={() => {
          if(!enableUseButton) return;
          let ttsStoreCopy = get(ttsStore);
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

    .header {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .header-inp {
        .header-inp-label-container {
            position: absolute;
            label {
                display: flex;
                align-items: center;
                gap: 2px;

                position: relative;
                font-size: 0.8rem;
                left: 8px;
                top: 3px;
                opacity: 0.5;
            }
            user-select: none;
            pointer-events: none;
        }
        :global(input) {
            height: 1.5rem;
            font-size: 0.8rem;
            width: 100%;
        }
    }
</style>
