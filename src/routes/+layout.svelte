<script>
  import '../assets/main.css'
  import '../assets/switzer.css'
  import ToastContainer from '@bird/components/alert/ToastContainer.svelte';
  const { children } = $props();
</script>

<ToastContainer/>
{@render children()}
