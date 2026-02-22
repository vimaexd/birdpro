<script>
    let {
      value = $bindable(),
      children,
      secret = false,
      ...props
    } = $props();

    // svelte-ignore state_referenced_locally
    let hidden = $state(secret);
</script>

<div class="textinput">
    <label for="inp">
        {@render children()}
    </label>

    {#if hidden}
        <div class="inputbox inputbox-placeholder" onclick={() => {
          hidden = false;
        }}>
            <p>(hidden, click to reveal)</p>
        </div>
    {:else}
        <input class="inputbox" type="text" name="inp" bind:value={value} {...props}>
    {/if}

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

    .inputbox {
        margin: none;
        background-color: transparent;
        border: 1px var(--color-surface0) solid;
        border-radius: var(--rounding);
        padding: 8px 8px;
        color: var(--color-text);
        height: 2.25rem;
    }

    .inputbox-placeholder {
        font-size: .9rem;
        p {
            opacity: .7;
        }

        &:hover {
            filter: brightness(1.2);
        }

        &:active {
            filter: brightness(0.8);
        }
    }

</style>
