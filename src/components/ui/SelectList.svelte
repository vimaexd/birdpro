<script lang="ts">
    import { setContext } from 'svelte';
    let { children, value = $bindable(), onChange } = $props();

   	setContext('selectlist-value', {
          get current() { return value; },
          set: (newValue: string) => {
            value = newValue;
            if(onChange != null) onChange();
          }
    });
</script>

<ul>
    {@render children()}
</ul>

<style>
    ul {
        list-style-type: none;
        padding: 0px;
        margin: 0;

        border: 1px var(--color-surface0) solid;
        border-radius: 4px;

        /*max-height: 10%;*/
        overflow: scroll;

        flex-shrink: 1;
    }

    ul :global(*:not(:last-of-type)) {
        border-bottom: 1px var(--color-surface0) solid;
    }
</style>
