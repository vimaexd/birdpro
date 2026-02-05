<script lang="ts">
    import { toastStore } from "@bird/lib/toast";
    import { expoOut } from "svelte/easing";
    import { fade } from "svelte/transition";
</script>

<div class="toasts">
    {#each $toastStore as toast}
        <div class="toast" out:fade={{easing: expoOut, duration: 300}}>
            <h2>{toast.title}</h2>
            <p>{toast.description}</p>
        </div>
    {/each}
</div>

<style lang="postcss">
    .toasts {
        position: absolute;
        top: 0;
        left: 0;

        height: 100vh;
        width: 100vw;

        z-index: 30;

        display: flex;
        flex-direction: column;
        gap: 4px;
        padding: 16px 0;
        align-items: center;

        pointer-events: none;
    }

    .toast {
        color: var(--color-text);

        background: color-mix(in srgb, var(--color-danger) 10%,
            color-mix(in srgb, var(--color-bg) 60%, transparent 40%) 90%);

        backdrop-filter: blur(16px) brightness(0.8);
        border: 1px var(--color-danger) solid;
        border-radius: var(--rounding);
        padding: 8px 16px;
        width: 360px;

        box-shadow: 0px 4px 4px rgba(0,0,0,0.4);
        will-change: filter;

        animation: toast-in .3s var(--ease-out-expo),
            toast-active 2.5s var(--ease-in-out-sine) infinite;

        & h2 {
            font-size: 1rem;
        }

        & p {
            font-size: 0.9rem;
        }
    }

    @keyframes toast-active {
        50% {
            border-color: color-mix(in srgb, var(--color-danger) 70%, #fff 30%);
        }
    }

    @keyframes toast-in {
        0% {
            opacity: 0;
            transform: translateY(-8px) scale(0.9);
        }
        100% {
            opacity: 1;
            transform: translateX(0px);
        }
    }
</style>
