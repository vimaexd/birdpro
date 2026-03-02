<script>
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import SettingsSectionTitle from "@bird/components/feat/settings/SettingsSectionTitle.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import SelectList from "@bird/components/ui/SelectList.svelte";
    import SelectListOption from "@bird/components/ui/SelectListOption.svelte";
    import { configStore } from "@bird/lib/config";
    import ColorPicker from "svelte-awesome-color-picker";
</script>

<SettingsPage>
    <div class="option">
        <SettingsSectionTitle>Theme</SettingsSectionTitle>
        <SelectList bind:value={$configStore["ui.theme"]}>
            <SelectListOption value="dark">Dark</SelectListOption>
            <SelectListOption value="light">Light</SelectListOption>
        </SelectList>
    </div>

    <div class="option">
        <SettingsSectionTitle>Accent color</SettingsSectionTitle>
        <ColorPicker bind:hex={$configStore['ui.accentColor']} isAlpha={false}/>
    </div>

    <div class="option">
        <SettingsSectionTitle>Rounding</SettingsSectionTitle>
        <div class="range-val">
            <p>{$configStore['ui.rounding']}px</p>
            <input type="range" name="rounding" width="100%" bind:value={$configStore['ui.rounding']} min={0} max={16} step={1}>
        </div>
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
