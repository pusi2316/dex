import "dotenv/config";
import { spawn } from "child_process";
import * as readline from "readline";
import { destroyTxt, transcribeAudio } from "./transcription/transcribe";
import { askClaude } from "./claude/claude-prompter";

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

function destroyAudio(outputPath: string): void {
  const fs = require("fs");
  try {
    fs.unlinkSync(outputPath);
  } catch (err) {
    console.error(`Error deleting ${outputPath}:`, err);
  }
}

function cleanup() {
  destroyTxt("test");
  destroyAudio("test.wav");
}

async function main() {
  await recordUntilEnter("test.wav");
  console.log("Done recording.");

  const text = await transcribeAudio("test.wav");
  console.log("Transcription:", text);

  cleanup();

  console.log("Asking Claude...");
  const reply = await askClaude(text);
  console.log("Claude says:", reply);
}

main();
