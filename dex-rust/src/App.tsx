import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useState } from "react";
import { ChatUI } from "./components/chatUI";

function App() {
  const [text, setText] = useState("");
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setText(await invoke("transcribe"));
    setText(await invoke("translate", { text: text }));
    await invoke("speak", { text: text, voice: "Samantha" });
  }

  return (
    <main className="container">
      <ChatUI />
    </main>
  );
}

export default App;
