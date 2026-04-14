<script lang="ts">
    import TextInput from "@bird/components/ui/TextInput.svelte";
    import { disableInputCapture } from '@bird/lib/modal';
    import Button from '@bird/components/ui/Button.svelte';
    import Modal from '@bird/components/alert/Modal.svelte';
    import { _ } from 'svelte-i18n';
    import SettingsExplainerText from '@bird/components/feat/settings/SettingsExplainerText.svelte';
    import Checkbox from '@bird/components/ui/Checkbox.svelte';
    import { invoke } from "@tauri-apps/api/core";
    import { configStore } from "@bird/lib/config";
    import { onMount } from "svelte";

    let serviceStatus = $state(false);

    let { onClose } = $props<{
      onClose: () => any;
    }>();

    const doClose = () => {
      disableInputCapture.set(false);
      onClose();
    }

    const startSvc = async () => {
        await invoke("hrm_svc_start");
        await updateSvcStatus();
    }

    const stopSvc = async () => {
        await invoke("hrm_svc_stop");
        await updateSvcStatus();
    }

    const updateSvcStatus = async () => {
        serviceStatus = await invoke("hrm_svc_status")
    }

    onMount(async () => {
        await updateSvcStatus();
    })
</script>

<Modal onClose={() => doClose()} height="fit-content">
    <h1>{$_("hrm.hrm")}</h1>

    <SettingsExplainerText>
        {@html $_("hrm.explainer")}
    </SettingsExplainerText>
    <SettingsExplainerText>
        {$_("hrm.vrcOscExplainer")}
    </SettingsExplainerText>


    <Checkbox bind:checked={$configStore["heartrate"]}>{$_("hrm.useHrm")}</Checkbox>
    <TextInput bind:value={$configStore["heartrate.widgetId"]}>{$_("hrm.widgetId")}</TextInput>

    <details>
      <summary>Advanced</summary>

      <div class="advanced">
          <TextInput onchange={(e: string) => {
              if(!isNaN(parseInt(e)) && +e > 0) {
                  $configStore["heartrate.customMaxHeartrate"] = +e;
              }
          }} value={$configStore["heartrate.customMaxHeartrate"]} placeholder="200">{$_("hrm.advancedMaxHr")}</TextInput>
          <TextInput bind:value={$configStore["heartrate.customConnectedParam"]} placeholder="/avatar/parameters/hr_connected">{$_("hrm.advancedConnectedParam")}</TextInput>
          <TextInput bind:value={$configStore["heartrate.customPercentParam"]} placeholder="/avatar/parameters/hr_percent">{$_("hrm.advancedPercentParam")}</TextInput>
      </div>
    </details>


    {#if serviceStatus}
        <div class="svc-status">
            <Button onclick={async () => stopSvc()}>Stop service</Button>
        </div>
	{:else}
        <div class="svc-status">
            <Button type="accent" onclick={async () => startSvc()}>Start service</Button>
        </div>
    {/if}

</Modal>

<style>
    summary {
        cursor: pointer;
        font-size: 0.9rem;

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
    }

    .svc-status {
        display: flex;
        align-items: center;
        gap: 16px;
    }
</style>
