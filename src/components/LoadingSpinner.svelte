<script>
    import IconSpinner from "../assets/icons/IconSpinner.svelte";
    const props = $props();
</script>

<div class="spin spin-center">
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
        animation-duration: 0.6s;
        animation-timing-function: linear;
        animation-iteration-count: infinite;

        height: 24px;
        width: 24px;
    }
</style>
