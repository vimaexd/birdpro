<script>
    let {
      checked = $bindable(),
      children,
      onchange = undefined,
      disabled = false
    } = $props();
</script>

<div class="box {(disabled) ? 'disabled' : ''}">
    <input type="checkbox" disabled={disabled} bind:checked={checked} onchange={() => {
      if(onchange && !disabled) {
        onchange()
      }
    }}>
    <p onclick={() => {
      checked = !checked
    }}>{@render children()}</p>
</div>

<style lang="postcss">
    .box {
        display: flex;
        align-items: center;

        &.disabled {
            opacity: 0.4;
            user-select: none;
        }
    }

    p {
        padding-left: 8px;
        font-size: 0.95rem;
        user-select: none;
        &:hover {
            cursor: pointer;
        }
    }

    input[type="checkbox"] {
      box-sizing: border-box;
      width: 20px;
      height: 20px;
      padding: 4px;
      border: none;
      appearance: none;
      background-color: var(--color-surface0);
      outline: 1px solid color-mix(in srgb, var(--color-surface0) 80%, #fff 20%);
      transition: outline 0.1s;

      border-radius: 2px;

      cursor: pointer;
    }

    input[type="checkbox"]:hover {
        filter: brightness(1.2);
    }

    input[type="checkbox"]:active {
        filter: brightness(0.8);
    }

    input[type="checkbox"]:checked {
      background-size: cover;
      padding: 2px;
    }

    input[type="checkbox"]:not(:disabled):checked {
      background-color: var(--color-accent);
      background-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="none" viewBox="0 0 32 32"><path fill="%23fff" d="M23.858 6.716a2.1 2.1 0 0 1 3.16 2.768L13.454 24.97a2.1 2.1 0 0 1-3.005.159l-5.775-5.338a2.1 2.1 0 0 1 2.85-3.084l4.19 3.873L23.859 6.716Z"/></svg>');
    }

</style>
