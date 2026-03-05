<script>
    import IconKey from "@bird/assets/icons/IconKey.svelte";

    let { providerName = "EXAMPLE PROVIDER" } = $props();
</script>

<div class="requiresauth">
    <IconKey width={64} height={64}/>
    <h3>Authentication required</h3>
    <p>To use voices from <b>{providerName}</b>, enter an API key in settings</p>
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
