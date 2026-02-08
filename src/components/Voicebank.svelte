<script>
    import IconCloud from "../assets/icons/IconCloud.svelte";

    const {
      provider, voiceName, cloud = false
    } = $props();
</script>

<div class="voice-status">
    <div class="status-text">
        <h2>{voiceName}</h2>
        <p>
            {provider}
        {#if cloud}
            <IconCloud width="24px" height="24px"/>
        {/if}
        </p>
    </div>
</div>

<style>
    .voice-status {
        display: flex;
        justify-content: space-between;

        width: 100%;
        background: #000 linear-gradient(176deg, rgba(255,255,255,0.12) 0%, rgba(255,255,255,0.08) 10%, rgba(255,255,255,0.02) 40%);
        border-radius: var(--rounding);
        border: 1px solid rgba(255,255,255,0.2);

        padding: 12px;

        & .status-text {
            display: flex;
            flex-direction: column;
            justify-content: center;

            flex-grow: 0;
            min-width: 0;
            font-size: 0.9rem;

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
            display: flex;

            align-self: center;
            transform: translateY(-3px);
            font-size: 1.5rem;
            font-weight: 600;

            & p {
                user-select: none;
            }
        }
    }
</style>
