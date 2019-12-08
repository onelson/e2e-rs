import React, {useState, useEffect,useLayoutEffect, useRef} from "react";
import isEquals from "lodash.isequal";
import { getClient } from "../api-client";
import { ChatLogEntry } from "e2e-client";
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
      const resp = await client.getMessages();
        setMessages(messages => {
          if (isEquals(messages, resp.messages)) {
            return messages;
          } else {
            return resp.messages;
          }
        });
    };

    fetchMessages().then(() => updateScrollPosition())
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
