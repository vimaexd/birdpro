<script>
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
</SettingsPage>

<style>
</style>
