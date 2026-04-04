<script>
    import IconKey from "@bird/assets/icons/IconKey.svelte";
    import { _ } from "svelte-i18n";

    let { providerName = "EXAMPLE PROVIDER" } = $props();
</script>

<div class="requiresauth">
    <IconKey width={64} height={64}/>
    <h3>{$_('sidebar.authRequired')}</h3>
    <p>{@html $_('sidebar.authRequiredExplainer', { values: { providerName: providerName }})}</p>
</div>

<style>
    .requiresauth {
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
