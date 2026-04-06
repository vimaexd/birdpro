<script lang="ts">
    import TextInput from "@bird/components/ui/TextInput.svelte";
    import { disableInputCapture } from '@bird/lib/modal';
    import Button from '@bird/components/ui/Button.svelte';
    import Modal from '@bird/components/alert/Modal.svelte';
    import { _ } from 'svelte-i18n';
    import SettingsExplainerText from '@bird/components/feat/settings/SettingsExplainerText.svelte';
    import Checkbox from '@bird/components/ui/Checkbox.svelte';
    import { invoke } from "@tauri-apps/api/core";

    let { onClose } = $props<{
      onClose: () => any;
    }>();

    const doClose = () => {
      disableInputCapture.set(false);
      onClose();
    }
</script>

<Modal onClose={() => doClose()} height="fit-content">
    <!-- LOCALISE -->
    <h1>Heart rate monitor</h1>

    <!-- LOCALISE -->
    <SettingsExplainerText>
        Bird Pro can connect to a heart rate monitor using <b>Pulsoid</b> and send your heart rate information over OSC for avatars that support <b>hr-osc</b>.
    </SettingsExplainerText>
    <SettingsExplainerText>
        Heart rate monitoring will only work if VRChat OSC is enabled in settings.
    </SettingsExplainerText>


    <Checkbox>Use heart rate monitor</Checkbox>
    <TextInput>Pulsoid widget ID</TextInput>

    <details>
      <summary>Advanced</summary>

      <div class="advanced">
          <TextInput value="200">Max heart rate</TextInput>
          <TextInput value="/avatar/parameters/hr_connected">OSC connected parameter</TextInput>
          <TextInput value="/avatar/parameters/hr_percent">OSC percentage parameter</TextInput>
      </div>
    </details>


    <Button type="accent" onclick={async () => await invoke("hrm_svc_start")}>Start service</Button>

</Modal>

<style>
    summary {
        cursor: pointer;

        transition: transform .1s ease-out;
        &:hover {
            transform: translateX(2px);
        }
    }

    .advanced {
        margin-top: 8px;
        padding: 16px;
        background-color: var(--color-bg-layer);
        border-radius: var(--rounding);

        display: flex;
        flex-direction: column;
        gap: 8px;

        /* don't touch that! */
        pointer-events: none;
    }
</style>
