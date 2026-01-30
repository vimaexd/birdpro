<script>
    let { children, title } = $props();
</script>

<div class="sidebaritem">
    <div class="sidebaritem-header">
        <h2>{title}</h2>
        <hr/>
    </div>
    {@render children()}
</div>

<style>
    .sidebaritem {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }
    .sidebaritem-header {
        display: grid;
        grid-template-rows: 1fr;
        grid-template-columns: auto 1fr;
        gap: 32px;
        padding: 4px 0px;
        user-select: none;

        & h2 {
            font-weight: 500;
            font-size: 0.95rem;
        }
    }

    hr {
        border-top-width: 0px;
        border-bottom-width: 1px;
        border-left-width: 0px;
        border-bottom-color: rgba(255,255,255,0.45);
        width: 100%;
        height: fit-content;
        align-self: center;
    }
</style>
