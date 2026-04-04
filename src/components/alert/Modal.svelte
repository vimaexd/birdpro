<script lang="ts">
    import ModalContainer from "@bird/components/alert/ModalContainer.svelte";
    import { onMount, type Snippet } from 'svelte';
    import { disableInputCapture } from '@bird/lib/modal';
    import ModalCloseButton from '@bird/components/alert/ModalCloseButton.svelte';

    let { onClose, modalClass, children, width = "600px", height = "600px" } = $props<{
      onClose: () => any;
      modalClasses?: string;
      children: Snippet;
      width?: string;
      height?: string;
    }>();

    onMount(() => {
      $disableInputCapture = true;
    })

    document.body.addEventListener('keydown', (e) => {
      if(e.key == "Escape") {
        onClose()
      }
    })
</script>

<ModalContainer>
    <div class="modal {modalClass || ''}" style="width: {width}; height: {height}">
        <ModalCloseButton onclick={() => onClose()}/>
        {@render children()}
    </div>
</ModalContainer>

<style>
    .modal {
        animation: settings-in .5s var(--ease-out-expo);
        position: relative;

        display: flex;
        flex-direction: column;

        gap: 16px;

        background-color: var(--color-bg);
        border-radius: var(--rounding);
        padding: 24px;

        box-shadow: 0 2px 24px rgba(0,0,0,0.6);

        :global(h1) {
            font-size: 1.5rem;
        }
    }

    :global(.modal-section) {
        display: flex;
        flex-direction: column;
        gap: 4px;
        :global(label) {
            font-size: 0.9rem;
        }
    }
</style>
