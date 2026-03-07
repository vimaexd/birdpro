<script lang="ts">
    import IconPitch from "@bird/assets/icons/IconPitch.svelte";
    import IconRate from "@bird/assets/icons/IconRate.svelte";
    import { resolveProvider, type TTSStore } from "@bird/lib/bird";

    let {
        name,
        color,
        store,
        onclick,
        dragging = false,
        onpointerdown = undefined,
    } = $props<{
        name: string;
        color: string;
        store: TTSStore;
        onclick?: () => any;
        dragging?: boolean;
        onpointerdown?: (e: PointerEvent) => any;
    }>();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
    class="fav"
    class:dragging
    style="--preset-color: {color};"
    {onclick}
    {onpointerdown}
    role="button"
    tabindex="0"
>
    <p class="name">{name}</p>
    <div class="voice">
        <p class="voice-name">{store.voice.name}</p>
        <span class="chip">
            <IconPitch />
            {store.pitch}
        </span>
        <span class="chip">
            <IconRate />
            {store.rate}
        </span>
    </div>
    <p class="provider">
        {resolveProvider(store.voice.provider).name}
    </p>
</div>

<style>
    .fav {
        --preset-color: var(--color-accent);

        color: #fff;
        width: 100%;
        padding: 8px;

        border: 1px var(--color-transoutline) solid;
        /* the right border is kinda fucked up so lets just manually make it nice*/
        border-right: 1px var(--color-surface2) solid;
        border-radius: var(--rounding);

        .name {
            font-size: 0.9rem;
        }

        .voice {
            font-size: 0.9rem;
            display: flex;
            gap: 8px;
            opacity: 0.8;

            .voice-name {
                overflow: hidden;
                white-space: nowrap;
                text-overflow: ellipsis;
            }
        }

        .provider {
            font-size: 0.7em;
            opacity: 0.5;
            margin: -2px 0px;
        }

        background-image: linear-gradient(
            90deg,
            color-mix(in srgb, var(--preset-color) 40%, #000 60%) -50%,
            transparent
        );

        user-select: none;
        cursor: pointer;
        transition: filter 0.05s var(--ease-out-expo);
        filter: brightness(1);
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
        background-color: rgba(0, 0, 0, 0.4);
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
