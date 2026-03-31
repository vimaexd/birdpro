<script>
    import ClickyButton from "@bird/components/ClickyButton.svelte";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import SettingsSectionTitle from "@bird/components/feat/settings/SettingsSectionTitle.svelte";
    import LoadingSpinner from "@bird/components/LoadingSpinner.svelte";
    import StepToggle from "@bird/components/StepToggle.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import TextInput from "@bird/components/ui/TextInput.svelte";

</script>

<SettingsPage>
    <SettingsSectionTitle>Spinner</SettingsSectionTitle>
    <LoadingSpinner/>

    <div class="option">
        <SettingsSectionTitle>Buttons</SettingsSectionTitle>
        <Button>Normal button</Button>
        <Button disabled>Disabled button</Button>
        <Button type="accent">Accent button</Button>
        <Button type="small">Small button</Button>
        <ClickyButton color={"blue"}> Clicky button </ClickyButton>
    </div>



    <TextInput value="hello world">Text input</TextInput>
    <TextInput secret value="ivy is a bird">Secret text input</TextInput>
    <Checkbox>Checkbox</Checkbox>




</SettingsPage>
