import { invoke } from "@tauri-apps/api/core";

export function speakTts(text: string) {
  invoke("tts_say", { message: text });
}
