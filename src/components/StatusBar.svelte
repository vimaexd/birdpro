<script>
    import { _ } from "svelte-i18n";
    import { isSettingsOpen } from "@bird/lib/modal";
    import Button from "./ui/Button.svelte";
    import IconSettings from "@bird/assets/icons/IconSettings.svelte";
    import UpdateButton from "./feat/update/UpdateButton.svelte";
    import { isUpdateAvailable } from "@bird/lib/updates";
    import HeartRate from "./feat/statusbar/HeartRate.svelte";
    import HrmScreen from "@bird/routes/screens/hrm.svelte";
    import { configStore } from "@bird/lib/config";

    let showHrmScreen = $state(false);
</script>
{#if showHrmScreen}
    <HrmScreen onClose={() => showHrmScreen = false}/>
{/if}
<div class="statusbar">
    <div class="left">
        {#if $configStore["ui.showHrm"]}
            <HeartRate onclick={() => showHrmScreen = true}/>
        {/if}

        {#if $isUpdateAvailable}
           	<UpdateButton/>
        {/if}
    </div>

    <Button type="small" onclick={() => ($isSettingsOpen = true)}>
        <IconSettings width="16px" height="16px"/> {$_("settings.settings")}
    </Button>
</div>

<style>
    .statusbar {
        /*background-color: var(--color-bg-darker);*/
        display: flex;
        justify-content: end;
        font-size: .8rem;
        overflow: hidden;

        .left {
            display: flex;
            margin-right: auto;
            width: 50%;
        }
    }
</style>
