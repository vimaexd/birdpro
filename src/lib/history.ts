import dayjs, { Dayjs } from "dayjs";
import { writable, get } from "svelte/store";

const HISTORY_MAX = 999;

interface HistoryItem {
    message: string;
    timestamp: Dayjs;
}

export let historyStore = writable<HistoryItem[]>([]);

export function pushHistory(message: string) {
    let hist = get(historyStore);

    hist.push({
        message,
        timestamp: dayjs(),
    });
    if (hist.length > HISTORY_MAX) {
        hist = hist.slice(1, hist.length);
    }

    historyStore.set(hist);
}

export function getLastMessage() {
    let hist = get(historyStore);
    if (hist.length < 1) {
        return "";
    }
    return hist[hist.length - 1].message;
}

export function clearHistory() {
    historyStore.set([]);
}
