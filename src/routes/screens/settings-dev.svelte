<script>
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import SettingsSectionTitle from "@bird/components/feat/settings/SettingsSectionTitle.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import { audioStore } from "@bird/lib/audio";
    import { ttsStore } from "@bird/lib/bird";
    import { configStore } from "@bird/lib/config";
    import { favouritesStore } from "@bird/lib/favourites";
    import { historyStore } from "@bird/lib/history";
    import { showError } from "@bird/lib/toast";

</script>

<SettingsPage>
    <SettingsSectionTitle>Test buttons</SettingsSectionTitle>
    <div class="quick-buttons">
        <Button onclick={() => $configStore['onboarded'] = false}>Reset onboarding</Button>
        <Button onclick={() => {showError("TestError", "This is a test error")}}>Show test error</Button>
    </div>

    <SettingsSectionTitle>ttsStore</SettingsSectionTitle>
    <code>
        {@html
            JSON.stringify($ttsStore, null, 2)
            .replace(/\n/g, '<br/>')
            .replace(/ /g, '&nbsp;')
        }
    </code>
    <Button onclick={() => {
      $configStore['last'] = undefined;
      showError("Dev", "Reloading..")
      setTimeout(() => {
        window.location.reload()
      }, 1000)
    }}>Nuke ttsStore and reload</Button>

    <SettingsSectionTitle>historyStore</SettingsSectionTitle>
    <code>
        {@html
            JSON.stringify($historyStore, null, 2)
            .replace(/\n/g, '<br/>')
            .replace(/ /g, '&nbsp;')
        }
    </code>

    <SettingsSectionTitle>audioStore</SettingsSectionTitle>
    <code>
        {@html
            JSON.stringify($audioStore, null, 2)
            .replace(/\n/g, '<br/>')
            .replace(/ /g, '&nbsp;')
        }
    </code>

    <SettingsSectionTitle>favouritesStore</SettingsSectionTitle>
    <code>
        {@html
            JSON.stringify($favouritesStore, null, 2)
            .replace(/\n/g, '<br/>')
            .replace(/ /g, '&nbsp;')
        }
    </code>


    <SettingsSectionTitle>configStore</SettingsSectionTitle>
    <code>
        {@html
            JSON.stringify($configStore, null, 2)
            .replace(/\n/g, '<br/>')
            .replace(/ /g, '&nbsp;')
        }
    </code>

</SettingsPage>

<style>
    .quick-buttons {
        display: flex;
        gap: 4px;
    }

    code {
        font-size: 0.7rem; background-color: var(--color-bg-layer);
        border-radius: var(--rounding);
        padding: 4px;
    }
</style>
