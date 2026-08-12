import { execFile } from "child_process";

export function askClaude(prompt: string): Promise<string> {
  return new Promise((resolve, reject) => {
    execFile(
      "claude",
      ["-p", prompt],
      { maxBuffer: 1024 * 1024 * 10 },
      (err, stdout) => {
        if (err) return reject(err);
        resolve(stdout.trim());
      }
    );
  });
}
