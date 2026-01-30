<script lang="ts">
    import SidebarItem from "./SidebarItem.svelte";

    const {
      name
    } = $props();
</script>

<SidebarItem title={name}>
    <div class="buttons">
        <button style="--btn-sat:4">
            -5
        </button>
        <button style="--btn-sat:2">
            -1
        </button>
        <button style="--btn-sat:1">
            0
        </button>
        <button style="--btn-sat:2">
            +1
        </button>
        <button style="--btn-sat:4">
            +5
        </button>
    </div>
    <input type="range" id="steptoggle-slide">
</SidebarItem>

<style>

    .buttons {
        display: flex;
        gap: 8px;
    }

    .buttons > button {
        width: 100%;
        padding: 16px 16px;
        background-color: var(--color-surface0);
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
            filter: brightness(1.75) saturate(var(--btn-sat));
        }
    }

    #steptoggle-slide {
        width: 100%;
        height: 32px;
        appearance: none;
        background: transparent;
    }

    #steptoggle-slide::-webkit-slider-thumb {
        -webkit-appearance: none;
        border: 1px solid #000000;
        height: 28px;
        width: 16px;
        border-radius: 3px;
        background: #ffffff;
        cursor: pointer;
        margin-top: -10px; /* You need to specify a margin in Chrome, but in Firefox and IE it is automatic */
        box-shadow: 1px 1px 1px #000000, 0px 0px 1px #0d0d0d; /* Add cool effects to your sliders! */
    }

    #steptoggle-slide::-webkit-slider-runnable-track {
      width: 100%;
      height: 8.4px;
      cursor: pointer;
      box-shadow: 1px 1px 1px #000000, 0px 0px 1px #0d0d0d;
      background: #3071a9;
      border-radius: 1.3px;
      border: 0.2px solid #010101;
    }
</style>
