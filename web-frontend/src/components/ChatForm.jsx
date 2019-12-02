import React, { useState } from "react";
import "./ChatForm.css";
import { getClient } from "../api-client";
const ChatForm = () => {
  let [author] = useState("owen"); // TODO: dynamic author?
  let [text, setText] = useState("");
  let [inFlight, setInFlight] = useState(false);

  const handleSubmit = async event => {
    setInFlight(true);
    event.preventDefault();
    try {
      let client = getClient();
      await client.createMessage({
        author,
        text
      });
      setText(""); // reset the form once the request is complete.
    } catch (reason) {
      console.error(reason);
    } finally {
      setInFlight(false);
    }
  };

  const handleTextChange = event => {
    setText(event.target.value);
  };

  return (
    <form className="ChatForm" onSubmit={handleSubmit}>
      <span className="author">{author}</span>
      <input
        type="text"
        placeholder="What's happening?"
        value={text}
        onChange={handleTextChange}
      />
      <input type="submit" value="Send" disabled={inFlight} />
    </form>
  );
};

export default ChatForm;
