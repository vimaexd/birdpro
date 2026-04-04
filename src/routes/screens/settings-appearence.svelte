<script>
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import SettingsSectionTitle from "@bird/components/feat/settings/SettingsSectionTitle.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import SelectList from "@bird/components/ui/SelectList.svelte";
    import SelectListOption from "@bird/components/ui/SelectListOption.svelte";
    import { configStore } from "@bird/lib/config";
    import ColorPicker from "svelte-awesome-color-picker";
    import { _ } from "svelte-i18n";
</script>

<SettingsPage>
    <div class="option">
        <SettingsSectionTitle>{$_("settings.appearence.theme")}</SettingsSectionTitle>
        <SelectList direction="horizontal" bind:value={$configStore["ui.theme"]}>
            <SelectListOption value="dark">{$_("settings.appearence.themeDark")}</SelectListOption>
            <SelectListOption value="light">{$_("settings.appearence.themeLight")}</SelectListOption>
            <SelectListOption value="auto">{$_("settings.appearence.themeAuto")}</SelectListOption>
        </SelectList>
    </div>

    <div class="option">
        <SettingsSectionTitle>{$_("settings.appearence.accentColor")}</SettingsSectionTitle>
        <ColorPicker bind:hex={$configStore['ui.accentColor']} isAlpha={false}/>
    </div>

    <div class="option">
        <SettingsSectionTitle>{$_("settings.appearence.rounding")}</SettingsSectionTitle>
        <div class="range-val">
            <p>{$configStore['ui.rounding']}px</p>
            <input type="range" name="rounding" width="100%" bind:value={$configStore['ui.rounding']} min={0} max={16} step={1}>
        </div>
    </div>

    <div class="option">
        <SettingsSectionTitle>
            {$_("settings.appearence.sections")}
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore["ui.showHistory"]}>
            {$_("settings.appearence.showHistory")}
        </Checkbox>
    </div>
</SettingsPage>

<style>
    .range-val {
        display: flex;
        gap: 8px;
        width: 100%;

        input {
            width: 100%;
        }
    }
</style>
