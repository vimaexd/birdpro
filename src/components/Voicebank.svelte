<script lang="ts">
    import IconCloud from "../assets/icons/IconCloud.svelte";

    let ref: HTMLDivElement;
    const {
      provider, voiceName, cloud = false, onclick = undefined, expanded = false
    } = $props();
</script>

<div class="voice-status" bind:this={ref} role="button" onclick={onclick}>
    <div class="status-text">
        <h2>{voiceName}</h2>
        <p>
            {provider}
        {#if cloud}
            <IconCloud width="16px" height="16px"/>
        {/if}
        </p>
    </div>
    <div class="arrow {(expanded) ? 'arrow-down' : ''}">
        <p>
            ›
        </p>
    </div>
</div>

<style>
    .voice-status {
        display: flex;
        justify-content: space-between;
        align-items: center;

        width: 100%;
        color: #fff;
        background: #000 linear-gradient(
        160deg,
        rgba(255,255,255,0.2) 0%,
        rgba(255,255,255,0.10) 40%,
        rgba(255,255,255,0.02) 80%
        );

        border-radius: var(--rounding);
        border: 1px solid var(--color-transoutline);

        padding: 8px 12px;
        overflow: hidden;

        cursor: pointer;

        & .status-text {
            display: flex;
            flex-direction: column;
            justify-content: center;

            flex-grow: 0;
            min-width: 0;
            font-size: 0.8rem;

            & h2 {
                font-size: 1em;
                font-weight: 600;
                text-overflow: ellipsis;
                white-space: nowrap;
                overflow: hidden;
            }

            & p {
                display: flex;
                align-items: center;
                gap: 8px;
                height: 1.2rem;
            }
            user-select: none;
        }

        & .arrow {
            font-size: 1.25rem;
            font-weight: 500;
            margin-left: auto;

            transition: transform .2s var(--ease-out-expo);
            user-select: none;
        }

        & .arrow-down {
            transform: rotate(90deg);
        }
    }
</style>
