<script>
    import { fade, fly } from "svelte/transition";
    import { expoOut } from "svelte/easing";
    let { children } = $props();
</script>
<div class="settings-page" in:fly={{duration: 400, x:-2, easing: expoOut }}>
    {@render children()}
</div>

<style>
    .settings-page {
        padding: 8px 0;
        display: flex;
        flex-direction: column;
        justify-content: center;

        overflow-y: scroll;

        gap: 16px;
    }

    /* todo: refactor into own component */
    .settings-page :global(.option) {
        display: flex;
        flex-direction: column;
        gap: 8px;
        margin-bottom: 8px;
    }
</style>
