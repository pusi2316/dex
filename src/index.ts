import "dotenv/config";
import { destroyTxt, transcribeAudio } from "./transcription/transcribe";
import { askClaude } from "./claude/claude-prompter";
import { destroyAudio, recordUntilEnter } from "./audio/audio-recorder";
import { speak } from "./audio/audio-speaker";

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

  console.log("Speaking reply...");
  await speak(reply);
}

main();
