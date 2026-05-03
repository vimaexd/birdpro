<script>
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import SettingsSectionTitle from "@bird/components/feat/settings/SettingsSectionTitle.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import TextInput from "@bird/components/ui/TextInput.svelte";
    import { configStore } from "@bird/lib/config";
    import {
        getTextFilePath,
        getTypingIndicatorTextFilePath,
    } from "@bird/lib/txtoutput";
    import { invoke } from "@tauri-apps/api/core";
    import { _ } from "svelte-i18n";
    import { writeText } from "@tauri-apps/plugin-clipboard-manager";

    async function connectOsc() {
        console.log("starting osc");

        if ($configStore.vrcOsc) {
            await invoke("osc_start");
        } else {
            await invoke("osc_stop");
        }
    }
</script>

<SettingsPage>
    <div class="option-section-header">
        <SettingsSectionTitle>VRChat</SettingsSectionTitle>
        <div class="option-section">
            <Checkbox onchange={connectOsc} bind:checked={$configStore.vrcOsc}>
                {$_("settings.integrations.vrcOsc")}
            </Checkbox>

            <SettingsExplainerText>
                {$_("settings.integrations.vrcOscExplainer")}
            </SettingsExplainerText>
        </div>
    </div>

    <div class="option-section-header">
        <SettingsSectionTitle>
            {$_("settings.integrations.txtOutput")}
        </SettingsSectionTitle>
        <div class="option-section">
            <Checkbox bind:checked={$configStore.txtoutput}>
                {$_("settings.integrations.txtOutputEnable")}
            </Checkbox>
            <Checkbox
                bind:checked={$configStore["txtoutput.clear"]}
                disabled={!$configStore.txtoutput}
            >
                {$_("settings.integrations.txtOutputClear")}
            </Checkbox>
            <div class="num {!$configStore.txtoutput ? 'disabled' : ''}">
                <input
                    name="txttimeout"
                    type="number"
                    min="0"
                    max="30"
                    bind:value={$configStore["txtoutput.clearTimeout"]}
                />
                <label for="txttimeout"
                    >{$_("settings.integrations.txtOutputTimeout")}</label
                >
            </div>
            {#if $configStore.txtoutput}
                {#await getTextFilePath() then path}
                    <div>
                        <h4>
                            {$_("settings.integrations.txtOutputPath")}
                        </h4>
                        <SettingsExplainerText>
                            <code>{path}</code>
                        </SettingsExplainerText>
                    </div>
                {/await}
            {/if}
        </div>
    </div>

    <div class="option-section-header">
        <SettingsSectionTitle>
            {$_("settings.integrations.txtIndicator")}
        </SettingsSectionTitle>
        <div class="option-section">
            <Checkbox bind:checked={$configStore["txtoutput.typingIndicator"]}>
                {$_("settings.integrations.txtIndicatorEnable")}
            </Checkbox>
            {#if $configStore["txtoutput.typingIndicator"]}
                <TextInput
                    bind:value={$configStore["txtoutput.typingIndicatorText"]}
                >
                    {$_("settings.integrations.txtIndicatorText")}
                </TextInput>
                {#await getTypingIndicatorTextFilePath() then path}
                    <div>
                        <h4>
                            {$_("settings.integrations.txtIndicatorPath")}
                        </h4>
                        <SettingsExplainerText>
                            <code>{path}</code>
                        </SettingsExplainerText>
                    </div>
                {/await}
            {/if}
        </div>
    </div>
</SettingsPage>

<style>
    .num {
        font-size: 0.9rem;
        input {
            color: var(--color-text);
            background-color: var(--color-surface0);
            padding: 2px 4px;
            border: none;
            border-radius: var(--rounding);
        }
        &.disabled {
            opacity: 0.4;
        }
    }

    h4 {
        font-size: 0.9rem;
        font-weight: 400;
    }
</style>
