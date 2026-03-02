<script lang="ts">
    import { fade, fly } from "svelte/transition";
    import { expoOut } from "svelte/easing";
    let {
      children,
      containerClass = ''
    } = $props();
</script>
<div class="settings-page {containerClass}" in:fly={{duration: 400, x:-2, easing: expoOut }}>
    {@render children()}
</div>

<style>
    .settings-page {
        padding: 8px 0;
        display: flex;
        flex-direction: column;
        height: 100%;

        overflow-y: scroll;

        gap: 16px;
    }

    /* todo: refactor into own component */
    .settings-page :global(.option) {
        display: flex;
        flex-direction: column;
        gap: 12px;
        margin-bottom: 24px;
    }
</style>
