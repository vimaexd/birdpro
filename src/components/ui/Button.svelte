<script>
    let props = $props();
</script>
<button {...props}>{@render props.children()}</button>

<style>
    button {
        background-color: var(--color-surface0);
        border: none;
        color: var(--color-text);
        padding: 8px 16px;
        font-size: 0.9rem;
        width: fit-content;
        border-radius: var(--rounding);
        cursor: pointer;

        &:hover {
            filter: brightness(1.2);
        }

        &:active {
            filter: brightness(0.9);
        }
    }
</style>
