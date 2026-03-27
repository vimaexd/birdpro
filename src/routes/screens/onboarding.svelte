<script>
    import logo from "@bird/assets/img/birdpro-logo.png";
    import Button from "@bird/components/ui/Button.svelte";
    import SelectList from "@bird/components/ui/SelectList.svelte";
    import SelectListOption from "@bird/components/ui/SelectListOption.svelte";
    import { configStore } from "@bird/lib/config";
    import { disableInputCapture } from "@bird/lib/modal";
    import { onMount } from "svelte";
    import ColorPicker from "svelte-awesome-color-picker";
    import { expoOut } from "svelte/easing";
    import { fade } from "svelte/transition";

    const completeOnboarding = () => {
      disableInputCapture.set(false);
      $configStore.onboarded = true;
    }

    onMount(() => {
      disableInputCapture.set(true);
    })
</script>

<div class="onboarding" out:fade={{duration: 900, easing: expoOut}}>
    <div class="onboarding-container">
        <img src={logo} class="logo" alt="Bird Pro logo"/>
        <h1>Welcome to Bird Pro!</h1>

        <div class="box">
            <div class="option">
                <h2>Theme</h2>
                <SelectList direction="horizontal" bind:value={$configStore["ui.theme"]}>
                    <SelectListOption value="dark">Dark</SelectListOption>
                    <SelectListOption value="light">Light</SelectListOption>
                    <SelectListOption value="auto">Auto</SelectListOption>
                </SelectList>
            </div>
            <div class="option">
                <h2>Color</h2>
                <ColorPicker bind:hex={$configStore['ui.accentColor']} isAlpha={false}/>
            </div>
        </div>

        <div class="actions">
            <Button type="accent" onclick={completeOnboarding}>Lets go!</Button>
        </div>
    </div>
</div>

<style>
    .onboarding {
        position: absolute;
        width: 100vw;
        height: 100vh;
        top: 0;
        left: 0;

        z-index: 400;

        transition: background 1s ease-out;
        background: linear-gradient(
            to bottom,
            var(--color-accent) 0%,
            var(--color-bg) 100%
        );
    }

    .onboarding-container {
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        h1 {
            font-size: 3rem;
            font-weight: 700;

            color: #fff;
        }

        .logo {
            height: 128px;
        }

        .box {
            margin-top: 32px;
            background-color: var(--color-bg);
            border-radius: var(--rounding);
            width: 500px;
            padding: 24px 32px;

            display: flex;
            justify-content: space-around;

            h2 {
                font-size: 1.2rem;
                padding: 2px 0;
                font-weight: 600;

                margin-bottom: 8px;
            }

            .option :global(ul) {
                display: flex;
                flex-direction: row;
            }
        }

        .actions {
            margin-top: 16px;
            display: flex;
            justify-content: center;
            width: 500px;

            :global(button) {
                width: 150px;
                display: flex;
                justify-content: center;
            }
        }
    }
</style>
