<script>
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import SettingsSectionTitle from "@bird/components/feat/settings/SettingsSectionTitle.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import { configStore } from "@bird/lib/config";
    import { _ } from "svelte-i18n";

</script>

<SettingsPage>
    <div class="option">
        <SettingsSectionTitle>
            {$_("settings.behaviour.sound")}
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore["audioTypingIndicator"]}>
            {$_("settings.behaviour.audibleTypingIndicator")}
        </Checkbox>

    </div>

    <div class="option">
        <SettingsSectionTitle>
            {$_("settings.behaviour.textbox")}
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore["bypassCharLimit"]}>
            {$_("settings.behaviour.bypassCharLimit")}
        </Checkbox>
        {#if $configStore["bypassCharLimit"]}
            <SettingsExplainerText>
                {$_("settings.behaviour.bypassCharLimitExplainer")}
            </SettingsExplainerText>
        {/if}
    </div>

    <div class="option">
        <SettingsSectionTitle>
            {$_("settings.behaviour.app")}
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore["checkForUpdates"]}>
            {$_("settings.behaviour.checkForUpdates")}
        </Checkbox>
    </div>

</SettingsPage>

<style>
</style>
