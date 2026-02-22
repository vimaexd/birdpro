import { writable, get } from "svelte/store";
import type { TTSStore } from "./bird";
import { configStore } from "./config";

export interface Favourite {
  name: string;
  color: string;
  store: TTSStore;
}
// stores copies of the TTSStore and swaps them out on demand
export let favouritesStore = writable<Favourite[]>([]);

export function saveFavourites() {
  let cs = get(configStore);
  cs.favourites = get(favouritesStore);
  configStore.set(cs);
};
