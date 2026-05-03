<script>
    import IconFolder from "@bird/assets/icons/IconFolder.svelte";
    import { _ } from "svelte-i18n";

    let { providerName = "EXAMPLE PROVIDER" } = $props();
</script>

<div class="notice">
    <IconFolder width={64} height={64} />
    <h3>{$_("sidebar.folderRequired")}</h3>
    <p>
        {@html $_("sidebar.folderRequiredExplainer", {
            values: { providerName: providerName },
        })}
    </p>
</div>

<style>
    .notice {
        user-select: none;
        padding: 0 16px;
        text-align: center;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 4px;
        height: 100%;

        p {
            font-size: 0.9rem;
        }
    }
</style>
