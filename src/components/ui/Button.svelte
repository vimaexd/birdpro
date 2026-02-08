<script>
    let props = $props();

    let styleClass = $derived.by(() => {
      switch(true) {
        case (props.disabled === true):
          return "btn-normal btn-disabled"

        case props.type == "accent":
          return "btn-accent"

        default:
          return "btn-normal"
      }
    })
</script>
<button class={styleClass} {...props}>{@render props.children()}</button>

<style>
    button {
        display: flex;
        gap: 4px;
        align-items: center;

        border: none;
        color: var(--color-text);
        padding: 8px 16px;
        font-size: 0.9rem;
        font-family: var(--font-family);
        width: fit-content;
        border-radius: var(--rounding);
        cursor: pointer;

        &:hover {
            filter: brightness(1.2);
        }

        &:active {
            filter: brightness(0.9);
        }
    }

    .btn-normal {
        background-color: var(--color-surface0);
    }

    .btn-disabled {
        opacity: 0.4;

        cursor: not-allowed;

        &:hover {
            filter: none;
        }

        &:active {
            filter: none;
        }
    }

    .btn-accent {
        background-color: var(--color-accent);
    }
</style>
