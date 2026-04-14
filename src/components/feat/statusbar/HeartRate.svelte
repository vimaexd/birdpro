<script>
    import IconHeart from "@bird/assets/icons/IconHeart.svelte";
    import LoadingSpinner from "@bird/components/LoadingSpinner.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import { listen } from '@tauri-apps/api/event';

    let { onclick } = $props();

    let isConnected = $state(false);
    let isLoading = $state(false);
    let heartrate = $state(100);

    listen("birdpro://hrm/connecting", (e) => {
        isConnected = false;
        isLoading = true;
    })

    listen("birdpro://hrm/connected", (e) => {
        isConnected = true;
        isLoading = false;
    })

    listen("birdpro://hrm/disconnected", (e) => {
        console.log("DC")
        isConnected = false;
    })

    listen("birdpro://hrm/update", (e) => {
        isConnected = true;
        heartrate = e.payload
    })
</script>

<div class="hrm-display">
    <Button type="small" {onclick}>
        <div class="hrm-container {(isConnected) ? 'connected' : 'disconnected'}">
            <span class="heart">
                <IconHeart height="16px" width="16px"/>
            </span>

            {#if isConnected}
                <span class="rate">{heartrate}</span>
            {:else if isLoading}
                <LoadingSpinner width={12} height={16}/>
            {:else}
                <span class="rate">0</span>
            {/if}
        </div>
    </Button>
</div>

<style>
    .hrm-container {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 4px;
    }

    .hrm-container.disconnected {
        opacity: 0.2;
    }

    @keyframes heartrate {
        0% {
            transform: scale(1.0) translateY(2px);
        }

        10% {
            transform: scale(1.1) translateY(2px);
        }

        100% {
            transform: scale(1.0) translateY(2px);
        }
    }

    .heart {
        transform: translateY(2px);
    }

    .hrm-container.connnected .heart {
        animation: heartrate 1s infinite;
    }

    .rate {
        width: 24px;
        text-align: left;
    }
</style>
