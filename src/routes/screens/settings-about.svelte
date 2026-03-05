<script>
    import logo from "@bird/assets/img/birdpro-logo.png";
    import Button from "@bird/components/ui/Button.svelte";
    import { getTauriVersion, getVersion } from "@tauri-apps/api/app";
    import { openPath, openUrl } from "@tauri-apps/plugin-opener";
    import { path } from "@tauri-apps/api";

</script>
<div class="about">
    <img src={logo} class="logo" alt="Bird Pro logo"/>

    <div class="text">
        <h2>Bird Pro</h2>
       	<p>
            v{#await getVersion() then ver}{ver}{/await}
            • {(import.meta.env.DEV) ? "Debug" : "Release"}
        </p>
        <p>Tauri v{#await getTauriVersion() then tv}{tv}{/await}</p>

        <p class="credit">
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            made with 💜 by <span class="link" role="link" tabindex="0" onclick={() => {
              openUrl("https://mae.wtf")
            }}>vimae</span>
        </p>
    </div>
</div>

<div class="bottom">
    <Button onclick={async () => {
      let cfgDir = await path.appConfigDir();
      openPath(cfgDir);
    }}>Config</Button>

    <Button onclick={async () => {
      let cfgDir = await path.appDataDir();
      openPath(cfgDir);
    }}>Logs</Button>
</div>

<style>
    h2 {
        font-size: 3rem;
    }
    .about {
        padding: 16px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        gap: 16px;
    }

    .logo {
        height: 128px;
    }

    :global(.theme-light) .logo {
        filter: invert()
    }

    .text {
        text-align: center;
    }

    .credit {
        margin-top: 3rem;
        & .link {
            color: var(--color-text);
            font-weight: 600;
            text-decoration-line: underline;
            text-decoration-style: wavy;
            text-decoration-color: #cba6f7;
            cursor: pointer;
        }
    }

    .bottom {
        margin-top: auto;
        display: flex;
        gap: 8px;
    }
</style>
