<script>
    import { historyStore } from "@bird/lib/history";
    import HistoryItem from "./HistoryItem.svelte";

    const {
      onSelectMessage
    } = $props()
</script>
<div class="history">
    <div class="history-side">
        <span class="vr"></span>
        <h2 class="history-title">History</h2>
        <span class="vr"></span>
    </div>
    <div class="history-items">
        {#if $historyStore.length < 1}
            <span class="history-empty"
                >Say something, and it'll show up here!</span
            >
        {/if}
        {#each $historyStore as item}
            <HistoryItem
                onclick={() => onSelectMessage(item)}>{item}</HistoryItem
            >
        {/each}
    </div>
</div>

<style>
    .history {
        margin-top: 4px;
        height: 248px;
        display: flex;
        gap: 8px;

        .history-side {
            writing-mode: vertical-rl;
            text-orientation: sideways;
            transform: rotate(180deg);
            display: flex;
            flex-direction: row;
            align-items: center;

            gap: 16px;

            .vr {
                height: 100%;
                border-right: 1px var(--color-surface0) solid;
            }
        }

        .history-title {
            letter-spacing: 0.1px;
            font-size: 0.75rem;
            user-select: none;
        }

        .history-items {
            justify-content: end;
            display: flex;
            flex-direction: column;
            min-width: 0;
            gap: 8px;
            width: 100%;
        }

        .history-empty {
            opacity: 0.35;
            font-style: italic;
        }
    }
</style>
