<script>
    let props = $props();
</script>

<button class="close-btn" {...props}>×</button>

<style>
    .close-btn {
        position: absolute;
        top: 10px;
        right: 10px;
        background: none;
        border: none;
        color: var(--color-text);
        font-size: 1.4rem;
        line-height: 1;
        width: 28px;
        height: 28px;
        border-radius: var(--rounding);
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        &:hover {
            background-color: var(--color-surface0);
        }
        &:active {
            filter: brightness(0.8);
        }
    }
</style>
