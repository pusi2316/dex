import { spawn } from "child_process";
import * as readline from "readline";

export function recordUntilEnter(outputPath: string): Promise<void> {
  return new Promise((resolve, reject) => {
    console.log("Recording... Press Enter to stop.");

    const rec = spawn("sox", ["-d", outputPath]);

    rec.on("error", (err) => {
      reject(err);
    });

    const rl = readline.createInterface({
      input: process.stdin,
    });

    rl.once("line", () => {
      rl.close();
      rec.kill("SIGTERM");
    });

    rec.on("close", () => resolve());
  });
}

export function destroyAudio(outputPath: string): void {
  const fs = require("fs");
  try {
    fs.unlinkSync(outputPath);
  } catch (err) {
    console.error(`Error deleting ${outputPath}:`, err);
  }
}
