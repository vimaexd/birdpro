<script>
    import { audioStore, audioDevices, setAudioDevice } from "$lib/bird";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import SelectList from "@bird/components/ui/SelectList.svelte";
    import SelectListOption from "@bird/components/ui/SelectListOption.svelte"

    let showOutput2 = $state(false);

    console.log(showOutput2)
</script>

<div class="devices">

    <p>Output Device</p>
    <SelectList onChange={() => setAudioDevice($audioStore.devices[0])} bind:value={$audioStore.devices[0]}>
        {#each $audioDevices as device}
            <SelectListOption value={device}>
                {device}
            </SelectListOption>
        {/each}
    </SelectList>

    <Checkbox bind:checked={showOutput2}>
        Output 2
    </Checkbox>

    {#if showOutput2}
        <SelectList onChange={() => setAudioDevice($audioStore.devices[1])} bind:value={$audioStore.devices[1]}>
            {#each $audioDevices as device}
                <SelectListOption value={device}>
                    {device}
                </SelectListOption>
            {/each}
        </SelectList>
    {/if}
</div>


<style>
    .devices {
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 20px 1fr;
        grid-auto-flow: column;
        gap: 16px;
    }
</style>
