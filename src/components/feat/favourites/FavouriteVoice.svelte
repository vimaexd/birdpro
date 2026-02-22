<script lang="ts">
    import IconPitch from '@bird/assets/icons/IconPitch.svelte';
    import IconRate from '@bird/assets/icons/IconRate.svelte';
    import { resolveProvider, type TTSStore } from '@bird/lib/bird';

    let {
      key = undefined,
      name,
      color,
      store,
      onclick,
      draggable = false,
      ondragstart = undefined,
      ondragover = undefined,
      ondragend = undefined
    } = $props<{
      key?: any;
      name: string;
      color: string;
      store: TTSStore;
      onclick?: () => any;
      draggable?: boolean;

      ondragstart?: (e: any) => any;
      ondragover?: (e: any) => any;
      ondragend?: (e: any) => any;
    } >();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="fav" style="--preset-color: {color};" {onclick} role="button"
    tabindex="0"
    {...{ key } as any}
    draggable={draggable}
    ondragstart={ondragstart}
    ondragover={ondragover}
    ondragend={ondragend}>
    <p class="name">
        {name}
    </p>
    <div class="voice">
        <p class="voice-name">{store.voice.name}</p>
        <span class="chip">
            <IconPitch/>
            {store.pitch}
        </span>
        <span class="chip">
            <IconRate/>
            {store.rate}
        </span>
    </div>
    <p class="provider">
        {resolveProvider(store.providerId).name}
    </p>
</div>

<style>
    .fav {
        --preset-color: var(--color-accent);

        width: 100%;
        padding: 8px;

        border: 1px rgba(255,255,255,0.20) solid;
        /* the right border is kinda fucked up so lets just manually make it nice*/
        border-right: 1px var(--color-surface2) solid;
        border-radius: var(--rounding);

        .name {
            font-size: .9rem;
        }

        .voice {
            font-size: .9rem;
            display: flex;
            gap: 8px;
            opacity: .8;

            .voice-name {
                overflow: hidden;
                white-space: nowrap;
                text-overflow: ellipsis;
            }
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
