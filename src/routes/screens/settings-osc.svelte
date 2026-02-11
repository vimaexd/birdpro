<script>
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import { configStore } from "@bird/lib/config";
    import { invoke } from "@tauri-apps/api/core";

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
    <Checkbox onchange={connectOsc} bind:checked={$configStore.vrcOsc}>
        Enable VRChat OSC
    </Checkbox>

    <SettingsExplainerText>
        Enabling VRChat OSC will send TTS messages to the VRChat textbox, and sync the typing indicator.
    </SettingsExplainerText>
</SettingsPage>

<style>

</style>
