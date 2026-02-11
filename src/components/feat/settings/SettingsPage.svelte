<script>
    import { fade, fly } from "svelte/transition";
    import { expoOut } from "svelte/easing";
    let { children } = $props();
</script>
<div in:fly={{duration: 400, x:-2, easing: expoOut }}>
    {@render children()}
</div>

<style>
    div {
        padding: 8px 0;
        display: flex;
        flex-direction: column;
        justify-content: center;

        gap: 16px;
    }
</style>
