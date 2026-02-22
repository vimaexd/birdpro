<script lang="ts">
    import ColorPicker from 'svelte-awesome-color-picker';
    import FavouriteVoice from "@bird/components/feat/favourites/FavouriteVoice.svelte";
    import TextInput from "@bird/components/ui/TextInput.svelte";
    import { ttsStore } from "@bird/lib/bird";
    import { onMount } from 'svelte';
    import { disableInputCapture } from '@bird/lib/modal';
    import Button from '@bird/components/ui/Button.svelte';
    import Modal from '@bird/components/alert/Modal.svelte';
    import { favouritesStore, saveFavourites } from '@bird/lib/favourites';
    import { get } from 'svelte/store';

    let name = $state("My voice");
    let color = $state("#4744eb");

    onMount(() => {
      $disableInputCapture = true;
    })

    let { onClose } = $props<{
      onClose: () => any;
    }>();

    const doClose = () => {
      disableInputCapture.set(false);
      onClose();
    }

    const doAdd = () => {
      // for some weird reason if we dont structuredclone
      // then it stores a reference still?? huh??
      let store = structuredClone(get(ttsStore));
      let favStore = get(favouritesStore);
      favStore.push({
        name,
        color,
        store
      });
      favouritesStore.set(favStore);
      saveFavourites();
      doClose();
    }
</script>

<Modal onClose={() => doClose()}>
    <h1>Add favourite</h1>

    <div class="modal-section preview">
        <label for="preview">Preview</label>
        <div class="preview-fav-container">
            <FavouriteVoice name={name || "My voice"} color={color} store={$ttsStore}/>
        </div>
    </div>

    <div class="modal-section">
        <TextInput bind:value={name} required={true} maxlength={64}>
            Name
        </TextInput>
    </div>

    <div class="modal-section color">
        <label for="color">Color</label>
        <ColorPicker bind:hex={color} isAlpha={false}/>
    </div>

    <div class="modal-section bottom">
        <Button type="accent" onclick={doAdd}> Add </Button>
    </div>
</Modal>

<style>
    .bottom {
        margin-top: auto;

        :global(button) {
            width: 100%;
            display: flex;
            justify-content: center;
        }
    }

    .preview-fav-container {
        padding: 16px;
        background-color: var(--color-bg-layer);
        border-radius: var(--rounding);

        /* don't touch that! */
        pointer-events: none;
    }
</style>
