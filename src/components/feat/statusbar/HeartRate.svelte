<script>
    import IconHeart from "@bird/assets/icons/IconHeart.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import { listen } from '@tauri-apps/api/event';

    let { onclick } = $props();

    let heartrate = $state(100);

    let hr = listen("birdpro://hrm/update", (e) => {
        heartrate = e.payload
    })

</script>
<Button type="small" {onclick}>
    <div class="hrm-container">
        <span class="heart">
            <IconHeart height="16px" width="16px"/>
        </span>
        <span class="rate">{heartrate}</span>
    </div>
</Button>

<style>
    .hrm-container {
        display: flex;
        align-items: center;
        gap: 4px;
    }

    @keyframes heartrate {
        0% {
            transform: scale(1.0);
        }

        10% {
            transform: scale(1.1);
        }

        100% {
            transform: scale(1.0);
        }
    }

    .heart {
        animation: heartrate 1s infinite;
        transform: translateY(-5px);
    }

    .rate {
        width: 32px;
        text-align: left;
    }
</style>
