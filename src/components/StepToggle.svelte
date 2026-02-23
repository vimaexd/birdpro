<script lang="ts">
    import SidebarItem from "./SidebarItem.svelte";
    let {
      children,
      value = $bindable(),
      min,
      max,
      initial,
      minStep,
      majStep,
      onChange
    } = $props<{
      children: any;
      value: number;
      initial: number;
      min: number;
      max: number;
      minStep: number;
      majStep: number;
      onChange?: () => any;
    }>();


    /**
     * Adjusts value with capping
     * @param newValue the new value to set
     */
    function adjValue(newValue: number) {
        value = Math.min(Math.max(value+newValue, min), max)

        if(onChange) onChange()
    }

    function resetValue() {
        value = initial;
        if(onChange) onChange()
    }

</script>

<div class="steptoggle">
    <div class="info">
        {@render children()}
    </div>
    <div class="buttons">
        <button style="--btn-sat:3" onclick={() => { adjValue(-majStep) }}>
            -{majStep}
        </button>
        <button style="--btn-sat:2" onclick={() => { adjValue(-minStep) }}>
            -{minStep}
        </button>
        <button style="--btn-sat:1" onclick={() => { resetValue() }}>
            {initial}
        </button>
        <button style="--btn-sat:2" onclick={() => { adjValue(minStep) }}>
            +{minStep}
        </button>
        <button style="--btn-sat:3" onclick={() => { adjValue(majStep) }}>
            +{majStep}
        </button>
    </div>
    <p class="value-display">{value}</p>
    <input type="range" id="steptoggle-slide" step={minStep} min={min} max={max} bind:value={value}>
</div>

<style>
    .steptoggle {
        display: grid;
        grid-template-columns: 48px 1fr;
        grid-template-rows: 1fr 32px;

        gap: 2px 16px;

        align-items: center;

        .info {
            width: 100%;
            user-select: none;
            display: flex;
            flex-direction: column;
            align-items: center;

        }

        .info :global(h2) {
            font-weight: 400;
            font-size: 1rem;
        }
    }

    .buttons {
        display: flex;
        gap: 8px;
    }

    .buttons > button {
        width: 20%;
        padding: 16px 0px;
        background-color: color-mix(in srgb, var(--color-surface0) 95%, var(--color-accent) 10%);
        color: var(--color-text);
        border: none;
        border-radius: var(--rounding);
        font-weight: 500;
        font-size: 1.5rem;

        cursor: pointer;
        transition: filter .15s var(--ease-out-expo);
        will-change: filter;
        filter: brightness(1.0) saturate(var(--btn-sat));

        &:hover {
            filter: brightness(1.55) saturate(var(--btn-sat));
        }

        &:active {
            filter: brightness(0.75) saturate(var(--btn-sat));
        }

        user-select: none;
    }

    .value-display {
        text-align: center;
        background-color: var(--color-surface0);
        border-radius: 99px;
        user-select: none;
    }

    #steptoggle-slide {
        width: 99%;
        height: 32px;
        appearance: none;
        background: transparent;
    }

    #steptoggle-slide::-webkit-slider-thumb {
        -webkit-appearance: none;
        border: 1px solid #000000;
        height: 20px;
        width: 16px;
        border-radius: 3px;
        background: #ffffff;
        cursor: pointer;
        margin-top: -6px;
    }

    #steptoggle-slide::-webkit-slider-runnable-track {
      width: 100%;
      height: 8.4px;
      cursor: pointer;
      background: linear-gradient(90deg,
        var(--color-accent),
        color-mix(in srgb, var(--color-surface0) 80%, var(--color-accent) 20%) ,
        var(--color-accent)
      );
      border-radius: var(--rounding);
    }
</style>
