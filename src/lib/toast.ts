import { writable, get } from "svelte/store";

export type ToastType = "success" | "error";
export let toastStore = writable<{
  type: ToastType;
  id: number;
  title: string;
  description: string;
}[]>([])

export function showError(title: string, description: string) {
  let newToastId = Math.floor(Math.random() * 10000);
  toastStore.set([
    ...get(toastStore),
    {
      type: "error",
      id: newToastId,
      title: title,
      description: description
    }
  ])

  // remove after 8s
  setTimeout(() => {
    let ts = get(toastStore)
    let target = ts.find(t => t.id == newToastId);
    if (!target) return;
    ts.splice(ts.indexOf(target), 1)
    toastStore.set(ts);
  }, 6000)
}
