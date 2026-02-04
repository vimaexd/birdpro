<script>
    import logo from "@bird/assets/img/birdpro-logo.png";
    import { getTauriVersion, getVersion } from "@tauri-apps/api/app";

</script>
<div class="about">
    <img src={logo} class="logo" alt="Bird Pro logo"/>

    <div class="text">
       	<p>Bird Pro v{#await getVersion() then ver}{ver}{/await}</p>
       	<p>Tauri v{#await getTauriVersion() then tv}{tv}{/await}</p>
        <br/>
        <p>Written by vimae</p>
    </div>
</div>

<style>
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

    .text {
        text-align: center;
    }
</style>
