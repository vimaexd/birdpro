<script>
    let { children, ...props } = $props();
</script>
<p {...props}>
    {@render children()}
</p>

<style>
    p {
        font-size: .9rem;
        opacity: .6;

        :global(a) {
            color: var(--color-accent);
            text-decoration: none;

            filter: brightness(2.3);
        }
    }
</style>
