import { execFile } from "node:child_process";

export function speak(text: string, voice = "Samantha"): Promise<void> {
  return new Promise((resolve, reject) => {
    execFile("say", ["-v", voice, text], (err) => {
      if (err) return reject(err);
      resolve();
    });
  });
}
