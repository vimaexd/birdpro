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
    import { _ } from "svelte-i18n";

</script>
<SettingsPage>
    <div class="option">
        <h3> ElevenLabs </h3>
        <TextInput bind:value={$configStore["elevenlabs.apikey"]} secret>
            {$_("settings.providers.apikey")}
        </TextInput>

        <SettingsExplainerText>
            {#each $_("settings.providers.elevenlabsExplainer").split('{link}') as part, i}
                {part}{#if i === 0}<a onclick={() => {
              openUrl("https://elevenlabs.io/app/developers/api-keys")
            }}>https://elevenlabs.io/app/developers/api-keys</a>{/if}
            {/each}
        </SettingsExplainerText>
    </div>

    <div class="option">
        <h3> Piper </h3>
        <div class="textinput-withbtn">
            <TextInput bind:value={$configStore["piper.voicesPath"]}>
                {$_("settings.providers.piperVoicesPath")}
            </TextInput>
            <Button onclick={async () => {
              const folder = await open({
                multiple: false,
                directory: true
              });
              if(!folder) return;
              $configStore["piper.voicesPath"] = folder;
            }}>{$_("ui.selectFolder")}</Button>
        </div>

        <SettingsExplainerText>
            {#each $_("settings.providers.piperExplainer").split('{link}') as part, i}
                {part}{#if i === 0}<a onclick={() => {
              openUrl("https://huggingface.co/rhasspy/piper-voices/")
            }}>https://huggingface.co/rhasspy/piper-voices</a>{/if}
            {/each}
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
