<script lang="ts">
    import { audioDevices } from "@bird/lib/bird";
    import { audioStore, setAudioDevice, getAudioDeviceInfo, destroyAudioDevice } from "@bird/lib/audio";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import SelectList from "@bird/components/ui/SelectList.svelte";
    import SelectListOption from "@bird/components/ui/SelectListOption.svelte"
    import { configStore } from "@bird/lib/config";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import { invoke } from "@tauri-apps/api/core";

    const adjustVolume = async (setup: number, amount: number) => {
      await invoke("audio_set_volume", { setupIdx: setup, volume: amount })
    }
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

        <div class="extractrl">
            <label for="vol0">Main Volume</label>
            <input type="range" name="vol0" width="100%" bind:value={$configStore.volumes[0]} onchange={() => {
              adjustVolume(0, $configStore.volumes[0])
            }} min={0} max={1} step={0.01}>

            <p class="device-info">
                {#await getAudioDeviceInfo(0) then audioDevice}
                    {audioDevice.sample_rate}Hz
                    {audioDevice.bit_depth ** 2}bit
                {/await}
            </p>
        </div>

        <Checkbox bind:checked={$configStore.audio.usePreviewOutput} onchange={() => {
          if(!$configStore.audio.usePreviewOutput) {
            // if we just disabled, destroy audio device
            destroyAudioDevice(1)
          } else if (!$audioStore.devices[1]) {
            // if we just enabled and theres no device
            // set device 0 as device 1 to prevent there being no device
            // which causes errors
            $audioStore.devices[1] = $audioStore.devices[0]
            setAudioDevice($audioStore.devices[0], 1)
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
                <div class="extractrl">
                    <label for="vol0">Preview Volume</label>
                    <input type="range" name="vol0" width="100%" bind:value={$configStore.volumes[1]} onchange={() => {
                      adjustVolume(1, $configStore.volumes[1])
                    }} min={0} max={1} step={0.01}>

                    <p class="device-info">
                        {#await getAudioDeviceInfo(1) then audioDevice}
                            {audioDevice.sample_rate}Hz
                            {audioDevice.bit_depth ** 2}bit
                        {/await}
                    </p>
                </div>
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

    .extractrl {
        width: 100%;

        input[type=range] {
            width: 100%;
        }
    }
</style>
