<script>
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import SettingsSectionTitle from "@bird/components/feat/settings/SettingsSectionTitle.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import TextInput from "@bird/components/ui/TextInput.svelte";
    import { configStore } from "@bird/lib/config";
    import { getTextFilePath, getTypingIndicatorTextFilePath } from "@bird/lib/txtoutput";
    import { invoke } from "@tauri-apps/api/core";
    import { writeText } from "@tauri-apps/plugin-clipboard-manager";

    async function connectOsc() {
      console.log("starting osc")

      if($configStore.vrcOsc) {
        await invoke("osc_start");
      } else {
        await invoke("osc_stop");
      }
    }
</script>

<SettingsPage>
    <div class="option">
        <SettingsSectionTitle>
            VRChat
        </SettingsSectionTitle>
        <Checkbox onchange={connectOsc} bind:checked={$configStore.vrcOsc}>
            Enable VRChat OSC
        </Checkbox>

        <SettingsExplainerText>
            Enabling VRChat OSC will send TTS messages to the VRChat textbox, and sync the typing indicator.
        </SettingsExplainerText>
    </div>


    <div class="option">
        <SettingsSectionTitle>
            Text file output
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore.txtoutput}>
            Enable text file output
        </Checkbox>
        <Checkbox bind:checked={$configStore["txtoutput.clear"]} disabled={!$configStore.txtoutput}>
            Clear after time
        </Checkbox>
        <div class="num {(!$configStore.txtoutput) ? 'disabled' : ''}">
            <input name="txttimeout" type="number" min="0" max="30" bind:value={$configStore["txtoutput.clearTimeout"]}>
            <label for="txttimeout">second timeout before clearing</label>
        </div>
        {#if $configStore.txtoutput}
            {#await getTextFilePath() then path}
                <div>
                    <h4>
                        Text file path
                    </h4>
                    <SettingsExplainerText>
                        <code>{path}</code>
                    </SettingsExplainerText>
                </div>
            {/await}
        {/if}
    </div>


    <div class="option">
        <SettingsSectionTitle>
            Text typing indicator
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore["txtoutput.typingIndicator"]}>
            Enable .txt typing indicator
        </Checkbox>
        {#if $configStore["txtoutput.typingIndicator"]}
            <TextInput bind:value={$configStore["txtoutput.typingIndicatorText"]}>
                Typing indicator text
            </TextInput>
            {#await getTypingIndicatorTextFilePath() then path}
                <div>
                    <h4>
                        Typing indicator path
                    </h4>
                    <SettingsExplainerText>
                        <code>{path}</code>
                    </SettingsExplainerText>
                </div>
            {/await}

        {/if}
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
        font-size: .9rem;
        font-weight: 400;
    }
</style>
