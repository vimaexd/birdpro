<script lang="ts">
    import { ttsStore } from "@bird/lib/bird";
    import FavouriteVoice from "@bird/components/feat/favourites/FavouriteVoice.svelte";
    import { favouritesStore, saveFavourites } from "@bird/lib/favourites";
    import IconFavourite from "@bird/assets/icons/IconFavourite.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import IconEdit from "@bird/assets/icons/IconEdit.svelte";
    import { animate, stagger } from "animejs";
    import IconBin from "@bird/assets/icons/IconBin.svelte";

    let itemsContainer: HTMLDivElement;
    let editMode = $state(false);

    // pointer drag state
    let dragIndex: number | undefined = undefined;
    let dragY = $state(0);
    let startY = 0;
    let itemHeight = 0;
    let isDragging = $state(false);
    let visualIndex = $state<number | undefined>(undefined);

    const onPointerDown = (e: PointerEvent, index: number) => {
        if (!editMode) return;
        e.preventDefault();

        dragIndex = index;
        visualIndex = index;
        startY = e.clientY;
        dragY = 0;
        isDragging = true;

        const el = itemsContainer.children[index] as HTMLElement;
        itemHeight = el.getBoundingClientRect().height + 8; // 8 = gap

        window.addEventListener("pointermove", onPointerMove);
        window.addEventListener("pointerup", onPointerUp);
    };

    const onPointerMove = (e: PointerEvent) => {
        if (!isDragging || dragIndex === undefined) return;
        e.preventDefault();

        dragY = e.clientY - startY;

        const newIndex = Math.min(
            Math.max(0, dragIndex + Math.round(dragY / itemHeight)),
            $favouritesStore.length - 1,
        );

        if (newIndex !== visualIndex) {
            visualIndex = newIndex;
        }
    };

    const onPointerUp = () => {
        if (!isDragging || dragIndex === undefined || visualIndex === undefined)
            return;

        // reorder
        let fs = [...$favouritesStore];
        const [item] = fs.splice(dragIndex, 1);
        fs.splice(visualIndex, 0, item);
        $favouritesStore = fs;
        saveFavourites();

        dragIndex = undefined;
        visualIndex = undefined;
        dragY = 0;
        isDragging = false;

        window.removeEventListener("pointermove", onPointerMove);
        window.removeEventListener("pointerup", onPointerUp);
    };

    const getItemStyle = (i: number): string => {
        if (!isDragging || dragIndex === undefined || visualIndex === undefined)
            return "";

        if (i === dragIndex) {
            return `transform: translateY(${dragY}px); z-index: 10; opacity: 0.85; position: relative;`;
        }

        // shift other items out of the way
        if (dragIndex < visualIndex) {
            if (i > dragIndex && i <= visualIndex) {
                return `transform: translateY(-${itemHeight}px); transition: transform 0.15s ease;`;
            }
        } else if (dragIndex > visualIndex) {
            if (i < dragIndex && i >= visualIndex) {
                return `transform: translateY(${itemHeight}px); transition: transform 0.15s ease;`;
            }
        }

        return `transform: translateY(0); transition: transform 0.15s ease;`;
    };

    const toggleEditMode = () => {
        if (editMode) {
            editMode = false;
            saveFavourites();
            animate([...itemsContainer.children], {
                translateX: [32, 0],
                ease: "outExpo",
                duration: 400,
            });
        } else {
            editMode = true;
            animate([...itemsContainer.children], {
                translateX: [-32, 0],
                delay: stagger(50),
                ease: "outExpo",
                duration: 400,
            });
        }
    };

    const removeFavourite = (idx: number) => {
        let fs = $favouritesStore;
        fs.splice(idx, 1);
        favouritesStore.set(fs);
        saveFavourites();
    };
</script>

{#if $favouritesStore.length == 0}
    <p class="no-results">
        Click the <span><IconFavourite width="20px" height="20px" /></span>
        button to add a favourite!
    </p>
{:else}
    <div class="favourites-bar smol-buttons">
        <p>
            {#if editMode}
                Editing - drag to reorder
            {:else}
                {$favouritesStore.length} Favourites
            {/if}
        </p>
        <Button onclick={toggleEditMode}>
            {#if editMode}
                Done
            {:else}
                <IconEdit width="16px" height="16px" />
            {/if}
        </Button>
    </div>
{/if}

<div id="favourites" bind:this={itemsContainer}>
    {#each $favouritesStore as fav, i}
        <div class="favourite-container smol-buttons" style={getItemStyle(i)}>
            {#if editMode}
                <div class="favourite-editcontrols">
                    <Button onclick={() => removeFavourite(i)}>
                        <IconBin width="24px" height="24px" />
                    </Button>
                </div>
            {/if}
            <FavouriteVoice
                name={fav.name}
                color={fav.color}
                store={fav.store}
                dragging={isDragging && dragIndex === i}
                onpointerdown={editMode
                    ? (e) => onPointerDown(e, i)
                    : undefined}
                onclick={() => {
                    if (!editMode) {
                        ttsStore.set(structuredClone(fav.store));
                    }
                }}
            />
        </div>
    {/each}
</div>

<style>
    .no-results {
        display: flex;
        align-items: center;
        justify-content: center;

        font-size: 0.9rem;
        opacity: 0.6;

        span {
            transform: translateY(1px);
            margin: 0 2px;
        }
    }

    .favourites-bar {
        font-size: 0.8rem;

        display: flex;
        align-items: center;
        justify-content: space-between;

        height: 16px;
    }

    .smol-buttons {
        :global(button) {
            font-size: 0.8rem;
            background-color: transparent;
            padding: 2px;

            background-color: var(--color-surface0);

            &:hover {
                background-color: var(--color-surface1);
            }
        }
    }

    .favourite-container {
        display: flex;
        width: 100%;
        overflow: hidden;
        gap: 4px;
        flex-shrink: 0;
    }

    .favourite-editcontrols {
        display: flex;
        width: 32px;

        :global(button) {
            background: color-mix(
                in srgb,
                var(--color-danger) 50%,
                color-mix(in srgb, var(--color-bg) 60%, transparent 40%) 50%
            );

            border: 1px var(--color-danger) solid;

            width: 100%;
            display: flex;

            justify-content: center;
        }
    }

    #favourites {
        display: flex;
        flex-direction: column;
        gap: 8px;
        overflow-x: hidden;
        overflow-y: scroll;
    }
</style>
