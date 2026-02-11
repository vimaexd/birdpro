<script>
    import { isSettingsOpen } from "@bird/lib/modal";
    import Button from "./ui/Button.svelte";
    import IconSettings from "@bird/assets/icons/IconSettings.svelte";
    import IconStop from "@bird/assets/icons/IconStop.svelte";
    import { invoke } from "@tauri-apps/api/core";

</script>
<div class="statusbar">
    <div class="left">
    </div>

    <Button onclick={() => ($isSettingsOpen = true)}>
        <IconSettings width="16px" height="16px"/> Settings
    </Button>
</div>

<style>
    .statusbar {
        /*background-color: var(--color-bg-darker);*/
        display: flex;
        justify-content: end;
        font-size: .8rem;
        padding: 2px;

        :global(button) {
            font-size: inherit;
            background-color: transparent;
            padding: 2px;

            &:hover {
                background-color: var(--color-surface0);
            }
        }

        .left {
            margin-right: auto;
        }
    }
</style>
