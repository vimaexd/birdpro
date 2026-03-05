<script>
    import Button from "@bird/components/ui/Button.svelte";
    import IconUpdate from "@bird/assets/icons/IconUpdate.svelte";
    import { fly } from "svelte/transition";
    import { expoOut } from "svelte/easing";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { updateUrl } from "@bird/lib/updates";

</script>

<div class="update-btn-wrapper" in:fly={{y: 16, duration: 800, easing: expoOut }}>
    <Button type="small" onclick={async () => {
      await openUrl(updateUrl)
    }}>
        <IconUpdate width="16px" height="16px"/> An update is available!
    </Button>
</div>

<style>
    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    @keyframes lookatme {
        0% {
            background-color: transparent;
        }
        50% {
            background-color: color-mix(in srgb, var(--color-accent) 30%, transparent)
        }
        100% {
            background-color: transparent;
        }
    }

    .update-btn-wrapper {
        :global(button){
            border-radius: var(--rounding);
            animation: update-btn-anim 1s linear infinite;


            box-sizing: border-box;

            position: absolute;

            transition: box-shadow .1s ease-out;
            animation: lookatme 0.5s 2 linear;
            animation-delay: .4s;

            :global(svg) {
                animation: spin 8s infinite linear;
            }
        }

        :global(button):hover {
            background-color: color-mix(in srgb, var(--color-accent) 30%, transparent);
            box-shadow: 1px 1px 8px 0px color-mix(in srgb, var(--color-accent) 50%, transparent);
        }
    }
</style>
