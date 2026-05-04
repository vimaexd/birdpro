<script lang="ts">
    import { _ } from "svelte-i18n";
    import { historyStore } from "@bird/lib/history";
    import HistoryItem from "./HistoryItem.svelte";
    import HistoryItemCompact from "./HistoryItemCompact.svelte";
    import { configStore } from "@bird/lib/config";
    import IconBin from "@bird/assets/icons/IconBin.svelte";
    import Button from "@bird/components/ui/Button.svelte";
    import { animate, stagger } from "animejs";
    import { fly } from "svelte/transition";
    import { expoOut } from "svelte/easing";
    import { derived } from "svelte/store";
    import HistoryItemSingle from "./HistoryItemSingle.svelte";

    const {
        onSelectMessage,
        fullHeight = false,
        width = "100%",
        maxItems = 3,
        singleLineMode = false,
    } = $props();

    let historyModeClass = $derived.by(() => {
        if (fullHeight) return "history-full";
        if (singleLineMode) return "history-singleline";

        return "history-" + $configStore["history.messageType"];
    });

    let itemsContainer: HTMLDivElement;
</script>

<div class="history {historyModeClass}" style="width: {width};">
    {#if !singleLineMode}
        <div class="history-side">
            <h2 class="history-title">{$_("history.historyTitle")}</h2>
            <span class="vr"></span>
            <div class="history-clear">
                <Button
                    type="small"
                    onclick={() => {
                        if ($historyStore.length < 1) return;
                        animate([...itemsContainer.children], {
                            translateY: [0, 16],
                            ease: "outExpo",
                            duration: 400,
                            opacity: [1, 0],
                            onComplete() {
                                historyStore.set([]);
                            },
                        });
                    }}
                >
                    <IconBin width="18px" height="18px" />
                </Button>
            </div>

        </div>
    {:else}
        <div class="history-side-inline">History</div>
    {/if}
    <div class="history-items" bind:this={itemsContainer}>
        {#if $historyStore.length < 1}
            <span
                class="history-empty"
                in:fly={{ duration: 400, x: -2, easing: expoOut }}
                >{$_("history.noHistory")}</span
            >
        {/if}
        {#each $historyStore.slice(-maxItems) as item}
            {#if $configStore["history.mode"] == "single"}
                <HistoryItemSingle
                    time={item.timestamp}
                    onclick={() => onSelectMessage(item.message)}
                    >{item.message}</HistoryItemSingle
                >
            {:else if $configStore["history.messageType"] == "compact"}
                <HistoryItemCompact
                    time={item.timestamp}
                    onclick={() => onSelectMessage(item.message)}
                    >{item.message}</HistoryItemCompact
                >
            {:else if $configStore["history.messageType"] == "large"}
                <HistoryItem
                    time={item.timestamp}
                    onclick={() => onSelectMessage(item.message)}
                    >{item.message}</HistoryItem
                >
            {/if}
        {/each}
    </div>
</div>

<style>
    .history-large {
        height: 150px;
    }

    .history-compact {
        height: 102px;
    }

    .history-singleline {
        height: 24px;
    }

    .history-full {
        height: 100%;
        flex-shrink: 1;
    }

    .history {
        width: 100%;
        margin-top: 4px;
        display: flex;
        gap: 8px;


        .history-side {
            height: calc(100% - 16px);
            writing-mode: vertical-rl;
            text-orientation: sideways;
            display: flex;
            align-items: center;

            gap: 8px;

            .vr {
                height: 100%;
                border-right: 1px var(--color-surface0) solid;
            }
        }

        .history-side-inline {
            font-size: 0.8rem;
            font-weight: 600;
            align-self: center;
            margin: 0 8px;
        }

        .history-title {
            letter-spacing: 0.1px;
            font-size: 0.75rem;
            user-select: none;
        }

        .history-items {
            height: 100%;
            width: 100%;
            justify-content: end;
            display: flex;
            flex-direction: column;
            min-width: 0;
            gap: 4px;

            overflow: hidden;
        }

        .history-empty {
            opacity: 0.35;
            font-style: italic;
        }

        .history-clear {
            display: block;
            flex-shrink: 0;
            opacity: 0.8;
        }
    }
</style>
