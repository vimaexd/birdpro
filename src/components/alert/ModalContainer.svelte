<script lang="ts">
    let { children } = $props();
</script>
<div class="modal-container">
    {@render children()}
</div>

<style>
    .modal-container {
        padding: 0;
        margin: 0;
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100vh;
        background-color: rgba(0,0,0,0.40);
        backdrop-filter: blur(6px) brightness(0.80);
        animation: modal-in .2s var(--ease-out-expo);
        z-index: 10;
        overflow: hidden;

        display: flex;
        justify-content: center;
        align-items: center;

        user-select: none;
    }

    @keyframes modal-in {
        0% {
            transform: scale(0.98);
            opacity: 0;
        }

        100% {
            opacity: 1;
            transform: scale(1);
        }
    }

</style>
