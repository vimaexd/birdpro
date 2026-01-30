import { invoke } from "@tauri-apps/api/core";

export async function speakTts(text: string) {
  await invoke("tts_say", { message: text });
}
