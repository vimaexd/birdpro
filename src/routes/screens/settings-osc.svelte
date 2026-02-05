<script>
    import Button from "@bird/components/ui/Button.svelte";
    import { invoke } from "@tauri-apps/api/core";

    async function connectOsc() {
      console.log("starting osc")
      await invoke("osc_start");
    }
</script>

<Button onclick={connectOsc}>
    Connect OSC
</Button>
