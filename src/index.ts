import { spawn } from "child_process";
import * as readline from "readline";

function recordUntilEnter(outputPath: string): Promise<void> {
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

async function main() {
  await recordUntilEnter("test.wav");
  console.log("Done recording.");
}

main();
