<script lang="ts">
    import IconAdd from "@bird/assets/icons/IconAdd.svelte";
    import IconBin from "@bird/assets/icons/IconBin.svelte";
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import TextInput from "@bird/components/ui/TextInput.svelte";
    import { configStore } from "@bird/lib/config";
    import { get } from "svelte/store";

    let replacements = $state(get(configStore).replacements);
    let entries = $derived(Object.entries(replacements));

    const saveReplacements = () => {
      console.log("saving replacements")
      let cs = get(configStore);
      cs.replacements = replacements;
      configStore.set(cs);
    }

    const updateReplacement = () => {


      // saveReplacements();
    }

    const removeReplacement = (key: string) => {
      delete replacements[key];
      saveReplacements();
    }

    const addNewReplacement = () => {
      console.log("awawa")
      let keys = Object.keys(replacements);
      let triedNumber = 1;

      while(true) {
        if(keys.includes(`Text to replace (${triedNumber})`)) {
          triedNumber++;
        } else {
          break;
        }
      }

      let replacementsCopy = replacements
      replacementsCopy[`Text to replace (${triedNumber})`] = "Replacement text";
      replacements = replacementsCopy;

      saveReplacements();
    }
</script>

<SettingsPage>
    <SettingsExplainerText>
        WORK IN PROGRESS
    </SettingsExplainerText>

    <table class="replacement-table">
        <thead>
            <tr>
                <th>Pattern</th>
                <th>Replacement</th>
                <th></th>
            </tr>
        </thead>
        <tbody>
            {#each entries as entry, i}
                <tr>
                    <td>
                        <!-- todo make this work -->
                        <TextInput onchange={(e: any) => updateReplacement()} value={entry[0]}></TextInput>
                    </td>
                    <td>
                        <!-- todo make this work -->
                        <TextInput onchange={(e: any) => updateReplacement()} value={entry[1]}></TextInput>
                    </td>
                    <td class="actions">
                        <Button onclick={() => removeReplacement(entry[0])}>
                            <IconBin width="20px" height="20px"/>
                        </Button>
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
    <div class="replacements-new">
        <Button type="accent" onclick={() => addNewReplacement()}>
            <IconAdd width={16} height={16}/>
        </Button>
    </div>
</SettingsPage>

<style>
    .replacement-table {
        width: 100%;
        thead tr th {
            font-weight: 600;
            width: 50%;
        }

        thead tr th:last-child {
            width: 52px;
        }

        tbody {
            overflow-y: scroll;
            margin: 0;

            tr td {
                padding: 0;
            }

            .actions :global(button) {
                /* using box shadow to have the line on the inside */
                box-shadow: inset 0px 0px 0px 1px var(--color-danger);
                background: color-mix(in srgb, var(--color-danger) 30%,
                    color-mix(in srgb, var(--color-bg) 60%, transparent 50%) 70%);
            }
        }
    }

    .replacements-new :global(button) {
        width: 100%;
        column-span: 2;
        display: flex;
        justify-content: center;
    }
</style>
