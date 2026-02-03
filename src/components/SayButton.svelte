<script>
    import IconEnter from "../assets/icons/IconEnter.svelte";
    import LoadingSpinner from "./LoadingSpinner.svelte";

    const props = $props();
</script>

<button class="btn-say {props.active ? 'active' : ''}" {...props}>
{#if props.loading}
    <LoadingSpinner/>
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
        background-color: var(--color-accent);
        color: #fff;
        border: none;

        font-size: 1.25rem;
        font-family: var(--font-family);
        height: 164px;

        border-radius: var(--rounding);

        transition: filter, transform .15s var(--ease-out-expo);
        will-change: filter;

        box-shadow: 0px 4px color-mix(in srgb, var(--color-accent) 70%, white 30%);

        cursor: pointer;

        &:hover {
            filter: brightness(1.35);
        }

        &:active, :global(&.active) {
            transform: translateY(4px);
            box-shadow: 0px 0px color-mix(in srgb, var(--color-accent) 70%, white 30%);
            filter: brightness(0.90);
        }

        & :global(*) {
            user-select: none;
        }
    }



    .action {
        margin-left: 8px;
    }


</style>
