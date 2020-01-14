import React, { useEffect, useLayoutEffect, useRef, useState } from "react";
import isEqual from "lodash.isequal";
import { ChatLogEntry, getClient, MessageListRequest } from "../api-client";
import "./ChatList.css";

const ChatList = () => {
  const listHandle = useRef<HTMLUListElement | null>(null);
  const [messages, setMessages] = useState<ChatLogEntry[] | null>(null);

  const updateScrollPosition = () => {
    const node = listHandle.current;

    if (node !== null) {
      node.scrollTop = node.scrollHeight;
    }
  };

  useEffect(() => {
    const fetchMessages = async () => {
      const client = getClient();
      const resp = await client.getMessages(new MessageListRequest());
      setMessages(messages => {
        if (isEqual(messages, resp.getMessagesList())) {
          return messages;
        } else {
          return resp.getMessagesList();
        }
      });
    };

    fetchMessages()
      .then(() => updateScrollPosition())
      .finally(() => setInterval(fetchMessages, 900));
  }, []);

  useLayoutEffect(() => {
    updateScrollPosition();
  });

  if (messages === null) {
    return <div>Loading...</div>;
  }

  return (
    <ul className="ChatList" ref={listHandle}>
      {messages.length === 0 ? (
        <li>No messages (yet).</li>
      ) : (
        messages.map((entry, idx) => {
          const date = entry
            .getTimestamp()
            ?.toDate()
            .toLocaleTimeString();
          const author = entry.getMsg()?.getAuthor();
          const text = entry.getMsg()?.getText();
          return author === "SYSTEM" ? (
            <li key={idx} className="system">
              <span className="who">{`[${date}]: `}</span>
              <span>{text}</span>
            </li>
          ) : (
            <li key={idx}>
              <span className="who">{`[${date}] ${author}: `}</span>
              <span>{text}</span>
            </li>
          );
        })
      )}
    </ul>
  );
};

export default ChatList;
