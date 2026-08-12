import { execFile } from "child_process";

const WHISPER_BINARY = process.env.WHISPER_BINARY!;
const WHISPER_MODEL = process.env.WHISPER_MODEL!;

export function transcribeAudio(audioPath: string): Promise<string> {
  return new Promise((resolve, reject) => {
    console.log(`Transcribing audio file: ${audioPath}`);
    console.log(`Using Whisper binary: ${WHISPER_BINARY}`);
    console.log(`Using Whisper model: ${WHISPER_MODEL}`);
    const outputBase = audioPath.replace(".wav", "");
    execFile(
      WHISPER_BINARY,
      ["-m", WHISPER_MODEL, "-f", audioPath, "-ng", "-otxt", "-of", outputBase],
      (err) => {
        if (err) {
          reject(err);
          return;
        }
        const fs = require("fs");
        const text = fs.readFileSync(`${outputBase}.txt`, "utf8").trim();
        resolve(text);
      }
    );
  });
}
