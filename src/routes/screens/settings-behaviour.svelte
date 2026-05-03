<script>
    import SettingsExplainerText from "@bird/components/feat/settings/SettingsExplainerText.svelte";
    import SettingsPage from "@bird/components/feat/settings/SettingsPage.svelte";
    import SettingsSectionTitle from "@bird/components/feat/settings/SettingsSectionTitle.svelte";
    import Checkbox from "@bird/components/ui/Checkbox.svelte";
    import { configStore } from "@bird/lib/config";
    import SelectList from "@bird/components/ui/SelectList.svelte";
    import SelectListOption from "@bird/components/ui/SelectListOption.svelte";

    import { _ } from "svelte-i18n";
    import HistoryItemCompact from "@bird/components/feat/history/HistoryItemCompact.svelte";
    import HistoryItem from "@bird/components/feat/history/HistoryItem.svelte";
</script>

<SettingsPage>
    <div class="option">
        <SettingsSectionTitle>
            {$_("settings.behaviour.sound")}
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore["audioTypingIndicator"]}>
            {$_("settings.behaviour.audibleTypingIndicator")}
        </Checkbox>
    </div>

    <div class="option">
        <SettingsSectionTitle>
            {$_("settings.behaviour.textbox")}
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore["bypassCharLimit"]}>
            {$_("settings.behaviour.bypassCharLimit")}
        </Checkbox>
        {#if $configStore["bypassCharLimit"]}
            <SettingsExplainerText>
                {$_("settings.behaviour.bypassCharLimitExplainer")}
            </SettingsExplainerText>
        {/if}
    </div>

    <div class="option">
        <SettingsSectionTitle>
            {$_("settings.behaviour.history")}
        </SettingsSectionTitle>

        <div class="option">
            <p>{$_("settings.behaviour.historyDesign")}</p>

            <div class="preview">
                {#if $configStore["history.messageType"] == "compact"}
                    <HistoryItemCompact
                        >{$_(
                            "settings.behaviour.historyDesignExampleMessage",
                        )}</HistoryItemCompact
                    >
                {/if}

                {#if $configStore["history.messageType"] == "large"}
                    <HistoryItem
                        >{$_(
                            "settings.behaviour.historyDesignExampleMessage",
                        )}</HistoryItem
                    >
                {/if}
            </div>

            <SelectList
                direction="horizontal"
                bind:value={$configStore["history.messageType"]}
            >
                <SelectListOption value="large"
                    >{$_(
                        "settings.behaviour.historyMessageLarge",
                    )}</SelectListOption
                >
                <SelectListOption value="compact"
                    >{$_(
                        "settings.behaviour.historyMessageCompact",
                    )}</SelectListOption
                >
            </SelectList>
        </div>

        <div class="option">
            <p>{$_("settings.behaviour.historyMode")}</p>
            <SettingsExplainerText>
                {$_("settings.behaviour.historyModeExplainer")}
            </SettingsExplainerText>
            <SelectList
                direction="horizontal"
                bind:value={$configStore["history.mode"]}
            >
                <SelectListOption value="bar"
                    >{$_(
                        "settings.behaviour.historyMode.bar",
                    )}</SelectListOption
                >
                <SelectListOption value="panel"
                    >{$_(
                        "settings.behaviour.historyMode.panel",
                    )}</SelectListOption
                >
                <SelectListOption value="single"
                    >{$_(
                        "settings.behaviour.historyMode.single",
                    )}</SelectListOption
                >
            </SelectList>
        </div>
    </div>

    <div class="option">
        <SettingsSectionTitle>
            {$_("settings.behaviour.app")}
        </SettingsSectionTitle>
        <Checkbox bind:checked={$configStore["checkForUpdates"]}>
            {$_("settings.behaviour.checkForUpdates")}
        </Checkbox>
    </div>
</SettingsPage>

<style>
    .preview {
        padding: 24px;
        border-radius: var(--rounding);
        background-color: var(--color-bg-layer);
    }
</style>
