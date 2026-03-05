import { writable } from "svelte/store"
import UpdateWorker from './worker/updateWorker?worker';
import { getVersion } from "@tauri-apps/api/app";

export const PROD_CONFIG = {
  updateGitForge: "https://codeberg.org",
  updateRepo: "vimae/birdpro"
}

export let updateUrl = "";
export let isUpdateAvailable = writable(false);

export async function startUpdateCheck() {
  const worker = new UpdateWorker;

  worker.onmessage = (msg: any) => {
    isUpdateAvailable.set(msg.data.updateAvailable);
    updateUrl = msg.data.updateUrl;
  }

  worker.postMessage({
    currentVersion: (await getVersion())
  })
}
