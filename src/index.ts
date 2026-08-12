import "dotenv/config";
import { spawn } from "child_process";
import * as readline from "readline";
import { transcribeAudio } from "./transcription/transcribe";

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

  const text = await transcribeAudio("test.wav");
  console.log("Transcription:", text);
}

main();
