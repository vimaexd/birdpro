<script lang="ts">
    let { children, onclick = undefined } = $props();
</script>

<button class="history-item" onclick={onclick} tabindex="0">
    <p>{@render children()}</p>
</button>

<style>
    .history-item {
        width: 100%;
        background-color: var(--color-surface0);
        padding: 12px 16px;
        min-width: 0;
        font-size: 1.115rem;
        font-family: var(--font-family);
        color: var(--color-text);
        text-align: left;

        border: none;
        border-radius: var(--rounding);
        border-bottom-left-radius: 0;

        transition: filter .02s var(--ease-out-expo);
        will-change: filter;

        & p {
            display: block;
            text-overflow: ellipsis;
            overflow: hidden;
            white-space: nowrap;
            user-select: none;
        }

        &:hover {
            filter: brightness(1.15);
            cursor:pointer;
        }
    }
</style>
