import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

type ChatMessage = { role: "user" | "assistant"; text: string };

export function ChatUI() {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState("");
  const [loading, setLoading] = useState(false);
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    const text = input.trim();
    if (!text || loading) return;

    setMessages((prev) => [...prev, { role: "user", text }]);
    setInput("");
    setLoading(true);

    try {
      const reply = await invoke<string>("process_chat", { text });
      setMessages((prev) => [...prev, { role: "assistant", text: reply }]);
    } catch (err) {
      setMessages((prev) => [
        ...prev,
        { role: "assistant", text: `Error: ${err}` },
      ]);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div style={{ maxWidth: "800", margin: "0 auto" }}>
      <div
        style={{
          height: 500,
          overflowY: "auto",
          border: "1px solid #ccc",
          padding: 12,
        }}
      >
        {messages.map((m, i) => (
          <div
            key={i}
            style={{
              margin: "6px 0",
              textAlign: m.role === "user" ? "right" : "left",
            }}
          >
            <strong>{m.role === "user" ? "You" : "Dex"}:</strong> {m.text}
          </div>
        ))}
        <div ref={bottomRef} />
      </div>
      <form
        onSubmit={handleSubmit}
        style={{ display: "flex", gap: 8, marginTop: 8 }}
      >
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Type a message..."
          style={{ flex: 1 }}
          disabled={loading}
        />
        <button type="submit" disabled={loading}>
          {loading ? "..." : "Send"}
        </button>
      </form>
    </div>
  );
}
