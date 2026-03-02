<script lang="ts">
    import IconAdd from "@bird/assets/icons/IconAdd.svelte";
    import IconBin from "@bird/assets/icons/IconBin.svelte";
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import TextInput from "@bird/components/ui/TextInput.svelte";
    import { configStore } from "@bird/lib/config";
    import { onMount } from "svelte";
    import { get } from "svelte/store";

    // local replacements here are stored in a different way to replacements
    // as its difficult to change the value of a k/v
    // so the primary key is essentially the index
    let replacements = $state<{
      from: string;
      to: string;
    }[]>([]);

    const saveReplacements = () => {
      console.log("saving replacements")

      // map replacements to normal format
      let mapped: Record<string, string> = {};
      replacements.forEach(r => {
        // if theres no key we cant store it
        if(r.from == "") return;
        mapped[r.from] = r.to;
      })

      let cs = get(configStore);
      cs.replacements = mapped;
      configStore.set(cs);
    }

    const updateReplacement = (index: number, keyOrValue: "k" | "v", value: string) => {
      let existing = replacements[index];
      if(keyOrValue == "k") {
        existing.from = value;
      } else {
        existing.to = value;
      }
      saveReplacements();
    }

    const removeReplacement = (index: number) => {
      replacements.splice(index, 1);
      saveReplacements();
    }

    const addNewReplacement = () => {
      let keys = replacements.map(r => r.from);
      let triedNumber = 1;

      while(true) {
        if(keys.includes(`Text to replace (${triedNumber})`)) {
          triedNumber++;
        } else {
          break;
        }
      }

      replacements.push({
        from: `Text to replace (${triedNumber})`,
        to: "Replacement text"
      })

      saveReplacements();
    }

    onMount(() => {
      // get current replacements and put into the right format
      let r = get(configStore).replacements;
      replacements = Object.entries(r).map(rep => {
        return {
          from: rep[0],
          to: rep[1]
        }
      })
    })
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
            {#each replacements as entry, i}
                <tr>
                    <td>
                        <!-- todo make this work -->
                        <TextInput oninput={(e: any) => updateReplacement(i, "k", e.target.value)} value={entry.from}></TextInput>
                    </td>
                    <td>
                        <!-- todo make this work -->
                        <TextInput oninput={(e: any) => updateReplacement(i, "v", e.target.value)} value={entry.to}></TextInput>
                    </td>
                    <td class="actions">
                        <Button onclick={() => removeReplacement(i)}>
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
