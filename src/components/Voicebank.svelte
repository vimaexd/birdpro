<div class="voice-status">
    <div class="status-text">
        <h2>Microsoft Hazel Desktop - English (Great Britain)</h2>
        <p>System</p>
    </div>

    <div class="arrow">
        <p>
            ›
        </p>
    </div>
</div>

<style>
    .voice-status {
        display: flex;
        justify-content: center;
        align-items: center;

        width: 100%;
        background: #000 linear-gradient(176deg, rgba(255,255,255,0.12) 0%, rgba(255,255,255,0.08) 10%, rgba(255,255,255,0.02) 40%);
        border-radius: var(--rounding);
        border: 1px solid rgba(255,255,255,0.2);

        padding: 12px;

        & .status-text {
            display: flex;
            flex-direction: column;
            justify-content: center;
            & h2 {
                font-size: 1em;
            }
        }

        & .arrow {
            font-size: 1.5rem;
            font-weight: 600;
        }
    }
</style>
