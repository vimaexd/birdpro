<script lang="ts">
  import '../assets/main.css'
  import '../assets/switzer.css'

  import ToastContainer from '@bird/components/alert/ToastContainer.svelte';
  const { children } = $props();

  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { initialiseConfig } from '@bird/lib/config';
  import { initialiseApp } from '@bird/lib/bird';
  import LoadingSpinner from '@bird/components/LoadingSpinner.svelte';
  import logo from "@bird/assets/img/birdpro-logo.png";
  import { showError } from '@bird/lib/toast';

  let ready = $state(false);

  onMount(async () => {
    try {
      await initialiseConfig();
      await initialiseApp();
      ready = true;
    } catch(e: any) {
      showError("Startup Error", e);
    }
  })
</script>

<ToastContainer/>


{#if ready}
    {@render children()}
{:else}
    <div class="loader" in:fade={{ duration: 500 }} out:fade={{duration: 100 }}>
        <img src={logo} height="128px" width="128px" alt="Bird Pro logo"/>
        <div class="spinner">
            <LoadingSpinner/>
        </div>
    </div>
{/if}

<style>
    .loader {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100vw;
        height: 100vh;

        color: var(--color-text);
    }

    @keyframes spinner-in {
        0% { opacity: 0; }
        100% { opacity: 1; }
    }

    .spinner {
        position: absolute;
        bottom: 64px;
        opacity: 0;
        will-change: opacity;
        animation: spinner-in 1s .5s forwards;
    }
</style>
