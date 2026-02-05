<script>
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

<Checkbox onchange={connectOsc} bind:checked={$configStore.vrcOsc}>
    Enable VRChat OSC
</Checkbox>

<p class="explainer">
    Enabling VRChat OSC will send TTS messages to the VRChat textbox, and sync the typing indicator.
</p>

<!-- <Button onclick={connectOsc}>
    Connect OSC
</Button> -->

<style>
    p {
        font-size: .9rem;
        opacity: .75;
    }
</style>
