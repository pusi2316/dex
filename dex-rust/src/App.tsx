import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useState } from "react";
import { ChatUI } from "./components/chatUI";

function App() {
  const [text, setText] = useState("");

  return (
    <main className="container">
      <ChatUI />
    </main>
  );
}

export default App;
