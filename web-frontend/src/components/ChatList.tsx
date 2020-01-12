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
        messages
          .map(x => {
            return {
              timestamp: x?.getTimestamp()?.toDate(),
              msg: x?.getMsg()?.toObject()
            };
          })
          .map(({ timestamp, msg }, idx) => {
            let date = new Date();
            return msg?.author === "SYSTEM" ? (
              <li key={idx} className="system">
                <span className="who">{`[${date.toLocaleTimeString()}]: `}</span>
                <span>{msg.text}</span>
              </li>
            ) : (
              <li key={idx}>
                <span className="who">{`[${date.toLocaleTimeString()}] ${
                  msg?.author
                }: `}</span>
                <span>{msg?.text}</span>
              </li>
            );
          })
      )}
    </ul>
  );
};

export default ChatList;
