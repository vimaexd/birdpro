<script>
    import Button from "@bird/components/ui/Button.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import TextInput from "@bird/components/ui/TextInput.svelte";
    import { configStore } from "@bird/lib/config";
    import { get } from "svelte/store";
    import { invoke } from "@tauri-apps/api/core";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
</script>
<SettingsPage>
    <h3> ElevenLabs </h3>
    <TextInput bind:value={$configStore["elevenlabs.apikey"]} secret>
        API Key
    </TextInput>

    <SettingsExplainerText>
        Generate an API key at <a href="https://elevenlabs.io/app/developers/api-keys">https://elevenlabs.io/app/developers/api-keys</a>
    </SettingsExplainerText>
</SettingsPage>


<style>
    h3 {
        font-weight: 500;
    }
</style>
