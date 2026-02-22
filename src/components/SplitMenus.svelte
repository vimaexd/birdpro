<script lang="ts">
    import { type Component } from "svelte";
    import IconCloud from "@bird/assets/icons/IconCloud.svelte";
    import IconFavourite from "@bird/assets/icons/IconFavourite.svelte";
    import IconPitch from "@bird/assets/icons/IconPitch.svelte";
    import IconSparkle from "@bird/assets/icons/IconSparkle.svelte";
    import SplitMenuFavourite from "./SplitMenuFavourite.svelte";
    import SplitMenuBrowse from "./SplitMenuBrowse.svelte";

    let selectedPage = $state("Favourite");
    let menus: {[id: string]: any} = {
      "Favourite": SplitMenuFavourite,
      "All Voices": SplitMenuBrowse
    }

    let icons: any[] = [IconFavourite, IconSparkle];

    let currentScreen = $derived(menus[selectedPage]);

</script>
<div class="menus">
    {#each Object.keys(menus) as menu, idx}
        <div class="menu {(selectedPage == menu) ? 'selected' : ''}" onclick={() => {
          selectedPage = menu
        }}>
            {@render icons[idx]({})}
            {menu}
        </div>
    {/each}
</div>

{@render currentScreen()}

<style>
    .menus {
        display: flex;
        /*gap: 2px;*/

        .menu {
            width: 100%;
            padding: 8px 16px;
            font-size: .9rem;

            border: 1px var(--color-surface0) solid;

            display: flex;
            align-items: center;
            justify-content: center;
            gap: 4px;
            user-select: none;

            cursor: pointer;

            transition: .15s var(--ease-out-expo);

            &:hover {
                background-color: var(--color-surface0);
            }

            &:active {
                filter: brightness(0.8);
            }

            :global(svg) {
                width: 18px;
                height: 18px;
                transform: translateY(-1px);
            }
        }

        .selected {
            background-color: var(--color-surface0);
        }

        .menu:first-of-type {
            border-top-left-radius: var(--rounding);
            border-bottom-left-radius: var(--rounding);
        }

        .menu:last-of-type {
            border-top-right-radius: var(--rounding);
            border-bottom-right-radius: var(--rounding);
        }

        .menu:not(:last-of-type) {
            border-right: none;
        }


        @property --gradient-spin {
            syntax: '<angle>';
            initial-value: 0deg;
            inherits: false;
        }
    }
</style>
