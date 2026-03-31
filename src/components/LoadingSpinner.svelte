<script>
    import IconSpinner from "../assets/icons/IconSpinner.svelte";
    import { fade } from 'svelte/transition';

    const props = $props();
</script>

<div class="spin spin-center" in:fade={{delay: 50}}>
    <IconSpinner height={24} width={24} {...props}/>
</div>

<style>
    @keyframes spin {
        0% {
            transform: rotate(0deg)
        }
        100% {
            transform: rotate(360deg)
        }
    }
    .spin-center {
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .spin {
        animation-name: spin;
        animation-duration: 0.45s;
        animation-timing-function: linear;
        animation-iteration-count: infinite;

        height: 24px;
        width: 24px;
    }
</style>
