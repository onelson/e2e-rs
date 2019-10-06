import React, { useState, useEffect } from "react";

const ChatList = () => {
    // FIXME: get the message typedef from rust
    const [messages, setMessages] = useState(null);

    useEffect( () => {
        const fetchMessages = async () => {
            const { get_messages } = await import("e2e-client");
            const resp = await get_messages("/api");
            setMessages(resp.messages);
        };
        fetchMessages();

    }, []);

    if (messages === null) {
        return <div>Loading...</div>;
    } else if (messages.length === 0) {
        return <div>No messages.</div>;
    }
    console.log(messages);
    return <ul>
        {messages.map(msg => {
            return <li>
                `[${msg.author}] ${msg.timestamp}: ${msg.text}`
            </li>
        })}
    </ul>;
};

export default ChatList;