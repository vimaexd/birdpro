<script>
    import { getContext } from "svelte";
    const { children, value, onSelect = undefined } = $props();

    let val = getContext('selectlist-value');
    let selected = $derived(val.current == value);

</script>

<button class={(selected) ? 'selected' : ''} onclick={() => {
  val.set(value)
}}>
    {@render children()}
</button>

<style>
    button {
        width: 100%;
        padding: 6px 16px;
        font-size: .9rem;

        background: transparent;
        color: var(--color-text);
        border: none;
        font-family: var(--font-family);
        text-align: left;

        will-change: background-color, filter;
        transition: background-color .2s var(--ease-out-expo), filter .07s var(--ease-out-expo);
        user-select: none;

        &:hover {
            background-color: var(--color-surface0);
            cursor: pointer;
        }

        &:active {
            filter: brightness(1.2);
        }

        display: flex;
        gap: 8px;
        align-items: center;
    }

    button.selected {
        background-color: var(--color-surface0);
    }
</style>
