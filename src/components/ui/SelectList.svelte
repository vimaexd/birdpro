<script lang="ts">
    import { setContext } from 'svelte';
    let { children, value = $bindable(), onChange, height = "auto" } = $props();

   	setContext('selectlist-value', {
          get current() { return value; },
          set: (newValue: string) => {
            value = newValue;
            if(onChange != null) onChange();
          }
    });
</script>

<ul style="height: {height};">
    {@render children()}
</ul>

<style>
    ul {
        list-style-type: none;
        padding: 0px;
        margin: 0;

        border: 1px var(--color-surface0) solid;
        border-radius: var(--rounding);

        overflow: scroll;

        flex-shrink: 1;

        display: flex;
        flex-direction: column;
        flex-grow: 0;

    }

    ul :global(*:not(:last-of-type)) {
        border-bottom: 1px var(--color-surface0) solid;
    }
</style>
