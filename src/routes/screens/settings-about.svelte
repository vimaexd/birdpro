<script>
    import logo from "@bird/assets/img/birdpro-logo.png";
    import Button from "@bird/components/ui/Button.svelte";
    import { getTauriVersion, getVersion } from "@tauri-apps/api/app";
    import { openPath, openUrl } from "@tauri-apps/plugin-opener";
    import { path } from "@tauri-apps/api";
    import { devmode } from "@bird/lib/bird";
    import { showError } from "@bird/lib/toast";
    import { _ } from "svelte-i18n";

    let versionClicked = $state(0);
</script>
<div class="about">
    <img src={logo} class="logo" alt="Bird Pro logo"/>

    <div class="text">
        <h2>Bird Pro</h2>
       	<p onclick={() => {
          versionClicked++;
          if(versionClicked == 5) {
            devmode.set(true);
            showError("Developer mode enabled", "Sqwawk sqwawk");
          } else if(versionClicked == 15) {
            showError("You don't need to click it again", "Seriously");
          }
        }}>
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
    }}>{$_('settings.about.config')}</Button>

    <Button onclick={async () => {
      let cfgDir = await path.appDataDir();
      openPath(cfgDir);
    }}>{$_('settings.about.logs')}</Button>
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
