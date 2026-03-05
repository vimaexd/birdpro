<script>
    import Button from "@bird/components/ui/Button.svelte";
    import { configStore } from "@bird/lib/config";
    import { showError } from "@bird/lib/toast";

</script>

<Button onclick={() => $configStore['onboarded'] = false}>Reset onboarding</Button>
<Button onclick={() => {showError("TestError", "This is a test error")}}>Show test error</Button>
