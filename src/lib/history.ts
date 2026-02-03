import { writable, get } from "svelte/store";

const HISTORY_MAX = 3;

export let historyStore = writable<string[]>([]);

export function pushHistory(item: string) {
  let hist = get(historyStore);

  hist.push(item);
  if (hist.length > HISTORY_MAX) {
    hist = hist.slice(1, hist.length)
  }

  historyStore.set(hist);
}

export function getLastMessage() {
  let hist = get(historyStore);
  if (hist.length < 1) {
    return ""
  }
  return hist[hist.length-1];
}

export function clearHistory() {
  historyStore.set([]);
}
