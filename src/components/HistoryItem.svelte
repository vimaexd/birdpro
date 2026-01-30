<script lang="ts">
    let { children } = $props();
</script>

<div class="history-item">
    <p>{@render children()}</p>
</div>

<style>
    .history-item {
        width: 100%;
        background-color: var(--color-surface0);
        padding: 12px 16px;
        /*height: 1.2em;*/
        min-width: 0;
        font-size: 1.115rem;

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
