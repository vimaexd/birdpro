<script>
    let { children, ...props } = $props();
</script>
<h3 {...props}>
    {@render children()}
</h3>

<style>
    h3 {
        font-weight: 500;
    }
</style>
