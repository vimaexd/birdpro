<script>
    import IconPitch from '@bird/assets/icons/IconPitch.svelte';
    import IconRate from '@bird/assets/icons/IconRate.svelte';
</script>

<div class="fav">
    <p class="name">
        Preset Name
    </p>
    <p class="voice">
        Microsoft Hazel
        <span class="chip">
            <IconPitch/>
            +14
        </span>
        <span class="chip">
            <IconRate/>
            -2
        </span>
    </p>
    <p class="provider">
        Windows
    </p>
</div>

<style>
    .fav {
        --preset-color: purple;

        width: 100%;
        padding: 8px;

        border: 1px rgba(255,255,255,0.20) solid;
        border-right: 1px var(--color-surface0) solid;
        border-radius: var(--rounding);

        .name {
            font-size: .9rem;
        }

        .voice {
            font-size: .9rem;
            display: flex;
            gap: 8px;
            opacity: .8;
        }

        .provider {
            font-size: .7em;
            opacity: .5;
            margin: -2px 0px;
        }

        background-image: linear-gradient(90deg,
            color-mix(in srgb, var(--preset-color) 40%, #000 60%) -50%,
            transparent);

        user-select: none;
        cursor: pointer;
        transition: filter .05s var(--ease-out-expo);
        filter: brightness(1.0);
        will-change: filter;

        &:hover {
            filter: brightness(1.4);
        }
        &:active {
            filter: brightness(0.9);
        }
    }

    .chip {
        display: flex;
        background-color: rgba(0,0,0,0.4);
        border: 1px color-mix(in srgb, var(--preset-color) 80%, #fff 20%) solid;
        border-radius: 99px;
        padding: 0 6px;
        gap: 4px;

        :global(svg) {
            width: 16px;
            height: 16px;
            transform: translateY(1px);
        }
    }
</style>
