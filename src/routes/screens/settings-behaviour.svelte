<script>
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import SettingsSectionTitle from "@bird/components/feat/settings/SettingsSectionTitle.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import { configStore } from "@bird/lib/config";
</script>

<SettingsPage>
    <div class="option">
        <SettingsSectionTitle>
            Sound
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore["audioTypingIndicator"]}>
            Audible typing indicator
        </Checkbox>

    </div>

    <div class="option">
        <SettingsSectionTitle>
            Textbox
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore["bypassCharLimit"]}>
            Bypass character limit
        </Checkbox>
        {#if $configStore["bypassCharLimit"]}
            <SettingsExplainerText>
                If you are outputting over VRChat OSC, messages will be truncated.
            </SettingsExplainerText>
        {/if}
    </div>

</SettingsPage>

<style>
</style>
