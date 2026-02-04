<script lang="ts">
    import { audioStore, audioDevices, setAudioDevice, getDeviceInfo, destroyAudioDevice } from "$lib/bird";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import SelectList from "@bird/components/ui/SelectList.svelte";
    import SelectListOption from "@bird/components/ui/SelectListOption.svelte"

    let showOutput2 = $state($audioStore.devices[1] !== undefined);



    $effect(() => {
      // hack to create effect dependency
      const outputShown = showOutput2;

      console.log("awa")

      if(!outputShown) {
        console.log("destroying")
      }
    })

</script>

<div class="devices">

    <p>Output Device</p>
    <SelectList onChange={() => setAudioDevice($audioStore.devices[0], 0)} bind:value={$audioStore.devices[0]} height="400px">
        {#each $audioDevices as device}
            <SelectListOption value={device}>
                {device}
            </SelectListOption>
        {/each}
    </SelectList>

    <p class="device-info">
        {#await getDeviceInfo(0) then audioDevice}
            {audioDevice.sample_rate}Hz
            {audioDevice.bit_depth ** 2}bit
        {/await}
    </p>

    <Checkbox bind:checked={showOutput2} onchange={() => {
      if(!showOutput2) {
        destroyAudioDevice(1)
      }
    }}>
        Preview Device
    </Checkbox>

    {#if showOutput2}
        <SelectList onChange={() => setAudioDevice($audioStore.devices[1], 1)} bind:value={$audioStore.devices[1]}>
            {#each $audioDevices as device}
                <SelectListOption value={device}>
                    {device}
                </SelectListOption>
            {/each}
        </SelectList>
        {#if $audioStore.devices[1]}
            <p class="device-info">
                {#await getDeviceInfo(1) then audioDevice}
                    {audioDevice.sample_rate}Hz
                    {audioDevice.bit_depth ** 2}bit
                {/await}
            </p>
        {/if}
    {/if}
</div>


<style>
    .devices {
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 20px 1fr 1fr;
        grid-auto-flow: column;
        gap: 16px;
    }

    .device-info {
        width: fit-content;
        height: fit-content;
        padding: 4px;

        opacity: .5;
    }
</style>
