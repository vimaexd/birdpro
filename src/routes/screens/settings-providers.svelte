<script>
    import Button from "@bird/components/ui/Button.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import TextInput from "@bird/components/ui/TextInput.svelte";
    import { configStore } from "@bird/lib/config";
    import { get } from "svelte/store";
    import { invoke } from "@tauri-apps/api/core";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { open } from "@tauri-apps/plugin-dialog";
</script>
<SettingsPage>
    <div class="option">
        <h3> ElevenLabs </h3>
        <TextInput bind:value={$configStore["elevenlabs.apikey"]} secret>
            API Key
        </TextInput>

        <SettingsExplainerText>
            Generate an API key at <a onclick={() => {
              openUrl("https://elevenlabs.io/app/developers/api-keys")
            }}>https://elevenlabs.io/app/developers/api-keys</a>
        </SettingsExplainerText>
    </div>

    <div class="option">
        <h3> Piper </h3>
        <div class="textinput-withbtn">
            <TextInput bind:value={$configStore["piper.voicesPath"]}>
                Voices folder
            </TextInput>
            <Button onclick={async () => {
              const folder = await open({
                multiple: false,
                directory: true
              });
              if(!folder) return;
              $configStore["piper.voicesPath"] = folder;
            }}>Select a folder</Button>
        </div>

        <SettingsExplainerText>
            Voices can be downloaded from <a onclick={() => {
              openUrl("https://huggingface.co/rhasspy/piper-voices/")
            }}>https://huggingface.co/rhasspy/piper-voices</a>
        </SettingsExplainerText>
    </div>
</SettingsPage>


<style>
    h3 {
        font-weight: 500;
    }

    .textinput-withbtn {
        display: flex;
        width: 100%;
        gap: 8px;


        :global(input) {
            width: 100%;
        }

        :global(button) {
            height: 61%;
            align-self: flex-end;
        }
    }
</style>
