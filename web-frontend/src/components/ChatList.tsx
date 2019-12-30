import React, { useEffect, useLayoutEffect, useRef } from "react";
import { useQuery } from "@apollo/react-hooks";
import { READ_MESSAGES } from "../api-client";
import { ReadMessages } from "../_gql/generated/ReadMessages";
import "./ChatList.css";

const ChatList = () => {
  const listHandle = useRef<HTMLUListElement | null>(null);
  const { loading, error, data: messageData } = useQuery<ReadMessages>(
    READ_MESSAGES,
    {
      pollInterval: 900
    }
  );

  const messages = messageData && messageData.allMessages;

  if (error) {
    console.error(error);
  }

  const updateScrollPosition = () => {
    const node = listHandle.current;

    if (node !== null) {
      node.scrollTop = node.scrollHeight;
    }
  };

  useEffect(() => {
    updateScrollPosition();
  }, [messages]);

  useLayoutEffect(() => {
    updateScrollPosition();
  });

  if (loading || !messages) {
    return <div>Loading...</div>;
  }

  return (
    <ul className="ChatList" ref={listHandle}>
      {messages.length === 0 ? (
        <li>No messages (yet).</li>
      ) : (
        messages.map(({ timestamp, msg }, idx) => {
          let date = new Date(timestamp);
          return msg.author === "SYSTEM" ? (
            <li key={idx} className="system">
              <span className="who">{`[${date.toLocaleTimeString()}]: `}</span>
              <span>{msg.text}</span>
            </li>
          ) : (
            <li key={idx}>
              <span className="who">{`[${date.toLocaleTimeString()}] ${
                msg.author
              }: `}</span>
              <span>{msg.text}</span>
            </li>
          );
        })
      )}
    </ul>
  );
};

export default ChatList;
