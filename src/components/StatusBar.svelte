<script>
    import { isSettingsOpen } from "@bird/lib/modal";
    import Button from "./ui/Button.svelte";
    import IconSettings from "@bird/assets/icons/IconSettings.svelte";
    import UpdateButton from "./feat/update/UpdateButton.svelte";
    import { isUpdateAvailable } from "@bird/lib/updates";

</script>
<div class="statusbar">
    <div class="left">
        {#if $isUpdateAvailable}
           	<UpdateButton/>
        {/if}
    </div>

    <Button type="small" onclick={() => ($isSettingsOpen = true)}>
        <IconSettings width="16px" height="16px"/> Settings
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
            margin-right: auto;
            width: 50%;
        }
    }
</style>
