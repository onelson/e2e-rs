import React, { useState, useEffect } from "react";
import { getClient } from "../api-client";
import { ChatLogEntry } from "e2e-client";
import "./ChatList.css";

const ChatList = () => {
  const [messages, setMessages] = useState<ChatLogEntry[] | null>(null);

  useEffect(() => {
    const fetchMessages = async () => {
      const client = getClient();
      const resp = await client.getMessages();
      setMessages(resp.messages);
    };
    fetchMessages().finally(() => setInterval(fetchMessages, 800));
  }, []);

  if (messages === null) {
    return <div>Loading...</div>;
  }

  return (
    <ul className="ChatList">
      {messages.length === 0 ? (
        <li>No messages (yet).</li>
      ) : (
        messages.map(({ timestamp, msg }, idx) => {
          let date = new Date(timestamp);
          return (
            <li key={idx}>
              <span className="who">{`[${
                msg.author
              }] ${date.toLocaleTimeString()}: `}</span>
              <span>{msg.text}</span>
            </li>
          );
        })
      )}
    </ul>
  );
};

export default ChatList;
