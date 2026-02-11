<script lang="ts">
    import { audioDevices } from "@bird/lib/bird";
    import { audioStore, setAudioDevice, getAudioDeviceInfo, destroyAudioDevice } from "@bird/lib/audio";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import SelectList from "@bird/components/ui/SelectList.svelte";
    import SelectListOption from "@bird/components/ui/SelectListOption.svelte"
    import { configStore } from "@bird/lib/config";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";

    let showOutput2 = $state($audioStore.devices[1] !== undefined);
</script>

<SettingsPage>
    <div class="devices">

        <p>Output Device</p>
        <SelectList onChange={() => setAudioDevice($audioStore.devices[0], 0)} bind:value={$audioStore.devices[0]} maxHeight="400px">
            {#each $audioDevices as device}
                <SelectListOption value={device}>
                    {device}
                </SelectListOption>
            {/each}
        </SelectList>

        <p class="device-info">
            {#await getAudioDeviceInfo(0) then audioDevice}
                {audioDevice.sample_rate}Hz
                {audioDevice.bit_depth ** 2}bit
            {/await}
        </p>

        <Checkbox bind:checked={$configStore.audio.usePreviewOutput} onchange={() => {
          if(!$configStore.audio.usePreviewOutput) {
            destroyAudioDevice(1)
          }
        }}>
            Preview Device
        </Checkbox>

        {#if $configStore.audio.usePreviewOutput}
            <SelectList onChange={() => setAudioDevice($audioStore.devices[1], 1)} bind:value={$audioStore.devices[1]} maxHeight="400px">
                {#each $audioDevices as device}
                    <SelectListOption value={device}>
                        {device}
                    </SelectListOption>
                {/each}
            </SelectList>
            {#if $audioStore.devices[1]}
                <p class="device-info">
                    {#await getAudioDeviceInfo(1) then audioDevice}
                        {audioDevice.sample_rate}Hz
                        {audioDevice.bit_depth ** 2}bit
                    {/await}
                </p>
            {/if}
        {/if}
    </div>
</SettingsPage>


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
