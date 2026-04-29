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
    <div class="option-row">
        <div class="option">
            <SettingsSectionTitle>{$_("settings.appearance.theme")}</SettingsSectionTitle>
            <SelectList direction="horizontal" bind:value={$configStore["ui.theme"]}>
                <SelectListOption value="dark">{$_("settings.appearance.themeDark")}</SelectListOption>
                <SelectListOption value="light">{$_("settings.appearance.themeLight")}</SelectListOption>
                <SelectListOption value="auto">{$_("settings.appearance.themeAuto")}</SelectListOption>
            </SelectList>
        </div>

        <div class="option">
            <SettingsSectionTitle>{$_("settings.appearance.accentColor")}</SettingsSectionTitle>
            <ColorPicker bind:hex={$configStore['ui.accentColor']} isAlpha={false}/>
        </div>
    </div>

    <div class="option">
        <SettingsSectionTitle>{$_("settings.appearance.rounding")}</SettingsSectionTitle>
        <div class="range-val">
            <p>{$configStore['ui.rounding']}px</p>
            <input type="range" name="rounding" width="100%" bind:value={$configStore['ui.rounding']} min={0} max={16} step={1}>
        </div>
    </div>

    <div class="option">
        <SettingsSectionTitle>{$_("settings.appearance.textSize")}</SettingsSectionTitle>
        <div class="range-val">
            <p>{$configStore['ui.textboxTextSize']}px</p>
            <input type="range" name="rounding" width="100%" bind:value={$configStore['ui.textboxTextSize']} min={12} max={78} step={1}>
        </div>
    </div>

    <div class="option">
        <SettingsSectionTitle>
            {$_("settings.appearance.sections")}
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore["ui.showHistory"]}>
            {$_("settings.appearance.showHistory")}
        </Checkbox>
        <Checkbox bind:checked={$configStore["ui.showHrm"]}>
            {$_("settings.appearance.showHrm")}
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
