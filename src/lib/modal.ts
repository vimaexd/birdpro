import { writable } from "svelte/store";

// when true, disables input capturing
// for modals with textual inputs
export let disableInputCapture = writable(false);

export let isSettingsOpen = writable(false);
