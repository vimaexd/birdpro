<script lang="ts">
    let { children } = $props();
</script>
<div class="status">
    <div class="status-text">
        <div class="status-dot"></div>
        <h2>{@render children()}</h2>
    </div>

    <div class="arrow">
        <p>
            ›
        </p>
    </div>
</div>

<style>
    .status {
        display: flex;
        justify-content: center;

        width: 100%;
        border-radius: var(--rounding);
        border: 1px solid rgba(255,255,255,0.2);

        padding: 4px 12px;

        & .status-text {
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 8px;
            & h2 {
                font-size: 1em;
            }
        }

        & .arrow {
            font-size: 1.25rem;
            font-weight: 600;
            margin-left: auto;
        }

        .status-dot {
            width: 12px;
            height: 12px;
            background-color: green;
            border-radius: 999px;
        }
    }
</style>
