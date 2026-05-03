<script lang="ts">
    import SelectList from "@bird/components/ui/SelectList.svelte";
    import SelectListOption from "@bird/components/ui/SelectListOption.svelte";
    import SvelteVirtualList from "@humanspeak/svelte-virtual-list";
    import IconCloud from "@bird/assets/icons/IconCloud.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import {
        ttsProviders,
        ttsStore,
        type Voice,
        getErrorText,
        resolveProvider,
    } from "@bird/lib/bird";
    import { invoke } from "@tauri-apps/api/core";
    import LoadingSpinner from "@bird/components/LoadingSpinner.svelte";
    import { showError } from "@bird/lib/toast";
    import { onMount } from "svelte";
    import { get } from "svelte/store";
    import NoticeRequiresAuth from "./NoticeRequiresAuth.svelte";
    import TextInput from "@bird/components/ui/TextInput.svelte";
    import IconSearch from "@bird/assets/icons/IconSearch.svelte";
    import { disableInputCapture } from "@bird/lib/modal";
    import { _ } from "svelte-i18n";
    import NoticeRequiresFolder from "./NoticeRequiresFolder.svelte";

    // provider
    let showProviders = $state(true);
    let provider = $state<string>("");

    // voices
    let ttsVoices = $state<Voice[]>([]);
    let ttsVoicesFiltered = $derived(
        ttsVoices.filter((v) =>
            v.name
                .toLowerCase()
                .includes(voiceSearchQuery.toLowerCase().trim()),
        ),
    );
    let justApplied = $state(false);
    let selectedVoice = $state("");
    let voiceSearchQuery = $state("");

    let enableUseButton = $derived(!justApplied && selectedVoice !== "");

    // notices
    let showRequiresAuth = $state(false);
    let showRequiresFolder = $state(false);

    const updateVoices = async () => {
        try {
            ttsVoices = await invoke("tts_get_voicelist", {
                providerId: provider,
            });
        } catch (e: any) {
            switch (e) {
                case "AuthorizationRequired":
                    showRequiresAuth = true;
                    break;

                case "MissingVoicesPath":
                    showRequiresFolder = true;
                    break;

                default:
                    let err = await getErrorText(e);
                    showError(e, err);
            }
        }
    };

    onMount(() => {
        updateVoices();
    });
</script>

{#if showProviders}
    <div class="header header-inp">
        <div>
            <h2>{$_("sidebar.chooseProvider")}</h2>
        </div>
    </div>
    <SelectList
        bind:value={provider}
        onChange={() => {
            showRequiresAuth = false;
            ttsVoices = [];
            selectedVoice = "";
            showProviders = false;
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

                <div style="margin-left: auto;">›</div>
            </SelectListOption>
        {/each}
    </SelectList>
{:else}
    <div class="header header-inp">
        <Button
            type="small"
            onclick={() => {
                showProviders = true;
                provider = "";
            }}
        >
            <h2>
                <span class="header-arrow">‹</span>
                {resolveProvider(provider).name}
            </h2>
        </Button>
        <div>
            {#if voiceSearchQuery.length < 1}
                <div class="header-inp-label-container">
                    <div>
                        <label for="">
                            <IconSearch width="16px" height="16px" />
                            {$_("ui.search")}
                        </label>
                    </div>
                </div>
            {/if}
            <TextInput
                bind:value={voiceSearchQuery}
                onclick={() => disableInputCapture.set(true)}
            />
        </div>
    </div>
    <SelectList
        bind:value={selectedVoice}
        onChange={() => {
            justApplied = false;
        }}
        height="100%"
    >
        {#await updateVoices()}
            <div class="selectlist-loading-container">
                <LoadingSpinner />
            </div>
        {:then voices}
            {#if showRequiresAuth}
                <NoticeRequiresAuth
                    providerName={resolveProvider(provider).name}
                />
            {:else if showRequiresFolder}
                <NoticeRequiresFolder
                    providerName={resolveProvider(provider).name}
                />
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
        <Button
            style="width: 100%;justify-content: center;"
            disabled={!enableUseButton}
            type="accent"
            onclick={() => {
                if (!enableUseButton) return;
                let ttsStoreCopy = get(ttsStore);
                ttsStoreCopy.voice = JSON.parse(
                    JSON.stringify(
                        ttsVoices.find((v: Voice) => v.id == selectedVoice)!,
                    ),
                );
                ttsStore.set(ttsStoreCopy);
                justApplied = true;
            }}
            >{$_("sidebar.useVoice")}
        </Button>
    </div>
{/if}

<style>
    .selectlist-loading-container {
        display: flex;
        width: 100%;
        justify-content: center;

        /* this is so cursed but whatever */
        padding: 32px;
    }

    .buttons {
        display: flex;
        justify-content: end;
    }

    h2 {
        font-size: 0.9rem;
        font-weight: 500;
        user-select: none;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .header {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .header-inp {
        font-size: 0.8rem;
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

            width: 100%;
        }
    }

    .header-arrow {
        display: inline-block;
        width: 16px;
        transform: scale(1.5);
        opacity: 0.75;

        border-radius: 12px;

        &::before {
            content: "";
            display: block;
            width: 100%;
            height: 100%;
            background: red;
            border-radius: var(--rounding);
            transform: scale(calc(1 / 1.5));
            transform-origin: center;
        }
    }
</style>
