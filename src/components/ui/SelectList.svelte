<script lang="ts">
    import { setContext } from 'svelte';
    let { children, value = $bindable(), onChange = undefined, height = "auto", maxHeight = "auto" } = $props();

   	setContext('selectlist-value', {
          get current() { return value; },
          set: (newValue: string) => {
            value = newValue;
            if(onChange != null) onChange();
          }
    });
</script>

<ul style="height: {height}; max-height: {maxHeight}; overflow-y: {(maxHeight !== 'auto') ? 'scroll' : 'none'}">
    {@render children()}
</ul>

<style>

    ul {
        list-style-type: none;
        padding: 0px;
        margin: 0;

        border: 1px var(--color-surface0) solid;
        border-radius: var(--rounding);


        display: flex;
        flex-direction: column;

    }

    ul :global(*:not(:last-of-type)) {
        border-bottom: 1px var(--color-surface0) solid;
    }
</style>
