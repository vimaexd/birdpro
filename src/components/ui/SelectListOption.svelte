<script>
    import { getContext, setContext } from "svelte";
    import { derived } from "svelte/store";
    const { children, value, onSelect = undefined } = $props();

    let val = getContext('selectlist-value');
    let selected = $derived(val.current == value);

</script>

<li class={(selected) ? 'selected' : ''} onclick={() => {
  console.log("clic")
  val.set(value)
}} role="button">
    {@render children()}
</li>

<style>
    li {
        padding: 6px 16px;
        font-size: .90rem;

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

    li.selected {
        background-color: var(--color-surface0);
    }
</style>
