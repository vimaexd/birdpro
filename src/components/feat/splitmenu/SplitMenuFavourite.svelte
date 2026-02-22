<script lang="ts">
    import { ttsProviders, ttsStore } from "@bird/lib/bird";
    import FavouriteVoice from "@bird/components/feat/favourites/FavouriteVoice.svelte";
    import { favouritesStore, saveFavourites } from "@bird/lib/favourites";
    import IconFavourite from "@bird/assets/icons/IconFavourite.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import IconEdit from "@bird/assets/icons/IconEdit.svelte";
    import IconStop from "@bird/assets/icons/IconStop.svelte";
    import { animate, spring, stagger } from "animejs";
    import IconBin from "@bird/assets/icons/IconBin.svelte";

    // TODO: address this to the user and make it work with the sorting system
    // dont want to overwrite user prefs when they try and sort stuff

    /*
    let filteredFavourites = $derived.by(() => {
      let fs = $favouritesStore;



      return fs
        // only show voices with currently available providers on this system
        // .filter(f => $ttsProviders.find(p => p.id == f.store.providerId) != undefined)
    })
    */

    let itemsContainer: HTMLDivElement;
    let editMode = $state(false);
    let dragSource: number | undefined = undefined;
    let dragDestination: number | undefined = undefined;

    const onDragStart = (e: any) => {
      console.log("started dragging");
      e.dataTransfer.effectAllowed = "move";
      e.dataTransfer.setData("text/plain", "");

      let key = e.target.getAttribute("key");
      dragSource = +key;
      dragDestination = +key;
    }

    const onDragOver = (e: any) => {
      let key = e.target.getAttribute("key");
      if(key == undefined) return;
      if(dragDestination == +key) return;
      dragDestination = +key;

      // this is such a hack i hate this
      let els = [...e.target.parentElement.parentElement.children];

      dragSource = (dragSource as number);
      dragDestination = (dragDestination as number);

      els.forEach(e => e.style.transform = '');

      // make a copy of the favourites store that we will apply our things to
      let fs = $favouritesStore;
      let sourceItem = fs[dragSource];

      if(dragSource < dragDestination) {
        for(let i = dragSource + 1; i < dragDestination + 1; i++) {
          fs[i-1] = fs[i];
        }
      } else {
        for(let i = dragSource - 1; i > dragDestination - 1; i--) {
          fs[i+1] = fs[i];
        }
      }

      fs[dragDestination] = sourceItem;
      dragSource = dragDestination;
      $favouritesStore = fs;
    }

    const onDragEnd = (e: any) => {
      dragSource = undefined;
      dragDestination = undefined;
    }

    const toggleEditMode = () => {
      if(editMode) {
        editMode = false;
        saveFavourites();
        animate([...itemsContainer.children], {
          translateX: [28, 0],
          ease: "outExpo",
          duration: 400,

          onComplete() {
          }
        })
      } else {
        editMode = true;

        animate([...itemsContainer.children], {
          translateX: [-28, 0],
          delay: stagger(50),
          ease: "outExpo",
          duration: 400
        })
      }
    }

    const removeFavourite = (idx: number) => {
      let fs = $favouritesStore;
      fs.splice(idx, 1);
      favouritesStore.set(fs);
      saveFavourites();
    }
</script>

{#if $favouritesStore.length == 0}
    <p class="no-results">
        Click the <span><IconFavourite width="20px" height="20px"/></span>
        button to add a favourite!
    </p>
{:else}
    <div class="favourites-bar smol-buttons">
        <p>
            {#if editMode}
                Editing - drag to reorder
            {:else}
                2 Favourites
            {/if}
        </p>

        <Button onclick={toggleEditMode}>
            {#if editMode}
                Done
            {:else}
                <IconEdit width="16px" height="16px"/>
            {/if}
        </Button>
    </div>
{/if}
<div id="favourites" bind:this={itemsContainer}>
    {#key $favouritesStore}
        {#each $favouritesStore as fav, i}
            <div class="favourite-container smol-buttons" {...{ key: i } as any}>
                {#if editMode}
                    <div class="favourite-editcontrols">
                        <Button onclick={() => removeFavourite(i)}>
                            <IconBin width="24px" height="24px"/>
                        </Button>
                    </div>
                {/if}
                <FavouriteVoice
                    key={i}
                    name={fav.name}
                    color={fav.color}
                    store={fav.store}
                    draggable={editMode}
                    ondragstart={onDragStart}
                    ondragover={onDragOver}
                    ondragend={onDragEnd}
                    onclick={() => {
                      if(!editMode) {
                        ttsStore.set(fav.store);
                      }
                    }}/>
            </div>
        {/each}
    {/key}
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
        font-size: .8rem;

        display: flex;
        align-items: center;
        justify-content: space-between;

        height: 16px;
    }

    .smol-buttons {
        :global(button) {
            font-size: .8rem;
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
            background: color-mix(in srgb, var(--color-danger) 50%,
                color-mix(in srgb, var(--color-bg) 60%, transparent 40%) 50%);

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
