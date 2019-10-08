import React, { useState, useEffect } from "react";
import "./ChatList.css";
import { getClient } from "../api-client";
const ChatList = () => {
  const [messages, setMessages] = useState(null);

  useEffect(() => {
    const fetchMessages = async () => {
      const client = await getClient();
      const resp = await client.get_messages();
      setMessages(resp.messages);
    };
    fetchMessages().finally(() => setInterval(fetchMessages, 1500));
  }, []);

  if (messages === null) {
    return <div>Loading...</div>;
  } else if (messages.length === 0) {
    return <div>No messages.</div>;
  }

  return (
    <ul className="ChatList">
      {messages.map(({ timestamp, msg }) => {
        let date = new Date(timestamp);
        return (
          <li>
            <span className="who">{`[${
              msg.author
            }] ${date.toLocaleTimeString()}: `}</span>
            <span>{msg.text}</span>
          </li>
        );
      })}
    </ul>
  );
};

export default ChatList;
