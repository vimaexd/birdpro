import { path } from "@tauri-apps/api";
import { writeTextFile } from "@tauri-apps/plugin-fs";

export let textTimeout: number; // set whenever a timeout is setup

export async function startClearTimeout(secs: number) {
  textTimeout = setTimeout(async () => {
    await setTextFileContents("")
  }, secs * 1000)
}

export async function getTextFilePath(): Promise<string> {
  let appdata = await path.appDataDir();
  return path.join(appdata, "tts-output.txt")
}

export async function setTextFileContents(content: string) {
  let path = await getTextFilePath();
  await writeTextFile(path, content);
}
