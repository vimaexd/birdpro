<script lang="ts">
    import { setContext } from 'svelte';
    let { children, value = $bindable(), onChange = undefined, height = "auto", maxHeight = "auto", shrink = "1" } = $props();

   	setContext('selectlist-value', {
          get current() { return value; },
          set: (newValue: string) => {
            value = newValue;
            if(onChange != null) onChange();
          }
    });
</script>

<ul style="height: {height}; max-height: {maxHeight}; flex-shrink: {shrink}; overflow-y: {(maxHeight !== 'auto') ? 'scroll' : 'none'}">
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

        background-clip: border-box;
        overflow: hidden;
    }

    ul :global(*:not(:last-of-type)) {
        border-bottom: 1px var(--color-surface0) solid;
    }
</style>
