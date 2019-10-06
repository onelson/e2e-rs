import React, { useState, useEffect } from "react";
import getClient from "../api-client";
const ChatList = () => {
    const [messages, setMessages] = useState(null);

    useEffect( () => {
        const fetchMessages = async () => {
            const client = await getClient("/api");
            const resp = await client.get_messages();
            setMessages(resp.messages);
        };
        fetchMessages().finally(
            () => setInterval(fetchMessages, 1500)
        );
    }, []);

    if (messages === null) {
        return <div>Loading...</div>;
    } else if (messages.length === 0) {
        return <div>No messages.</div>;
    }

    return <ul>
        {messages.map(msg => {
            let date = new Date(msg.timestamp);
            return <li>
                <span className="who">{`[${msg.author}] ${date.toLocaleTimeString()}: `}</span>
                <span>{msg.text}</span>
            </li>
        })}
    </ul>;
};

export default ChatList;