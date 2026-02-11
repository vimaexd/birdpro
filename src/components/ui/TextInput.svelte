<script>
    let {
      value = $bindable(),
      children,
      ...props
    } = $props();
</script>

<div class="textinput">
    <label for="inp">
        {@render children()}
    </label>
    <input type="text" name="inp" bind:value={value} {...props}>
</div>

<style>
    .textinput {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    label {
        font-size: .9rem;
        font-weight: 400;
    }

    input {
        margin: none;
        background-color: transparent;
        border: 1px var(--color-surface0) solid;
        border-radius: var(--rounding);
        padding: 8px 8px;
        color: var(--color-text);
    }

</style>
