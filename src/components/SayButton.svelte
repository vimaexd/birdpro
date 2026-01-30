<script>
    import IconEnter from "../assets/icons/IconEnter.svelte";
    import IconSpinner from "../assets/icons/IconSpinner.svelte";

    const props = $props();
</script>

<button class="btn-say" {...props}>
{#if props.loading}
    <div class="spin spin-center">
        <IconSpinner height={24}/>
    </div>
{:else}
    <IconEnter height={24} width={24}/> <span class="action">say</span>
{/if}
</button>


<style>
    .btn-say {
        display: flex;
        justify-content: center;
        align-items: center;

        width: 100%;
        background-color: #4744eb;
        color: #fff;
        border: none;
        /*padding: 24px 16px;*/
        font-size: 1.25rem;
        height: 164px;

        border-radius: var(--rounding);

        transition: filter, transform .15s var(--ease-out-expo);
        will-change: filter;

        box-shadow: 0px 4px color-mix(in srgb, #4744eb 70%, white 30%);

        cursor: pointer;

        &:hover {
            filter: brightness(1.35);
        }

        &:active {
            transform: translateY(4px);
            box-shadow: 0px 0px color-mix(in srgb, #4744eb 70%, white 30%);
        }
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg)
        }
        100% {
            transform: rotate(360deg)
        }
    }

    .action {
        margin-left: 8px;
    }

    .spin-center {
        display: flex;
        align-items: center;
    }

    .spin {
        animation-name: spin;
        animation-duration: 0.6s;
        animation-timing-function: linear;
        animation-iteration-count: infinite;
    }
</style>
