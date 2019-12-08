import React, { FormEvent, useState } from "react";
import { getClient } from "../api-client";
import "./ChatForm.css";

const ChatForm = () => {
  let [author, setAuthor] = useState<string | null>(null);
  let [text, setText] = useState("");
  let [inFlight, setInFlight] = useState(false);
  let client = getClient();

  React.useEffect(() => {
    client.getUsername().then(setAuthor, reason => console.error(reason));
  }, []);

  const handleSubmit = async (event: FormEvent) => {
    event.preventDefault();

    if (author === null) {
      throw new Error("author is not set.");
    }
    setInFlight(true);

    return client.createMessage({
        author,
        text
      }).then(() => {
        // reset the form once the request is complete.
        setText("");
      },
      reason => {
        console.error(reason);
      }).finally(() => {
        setInFlight(false);
      });
  };

  const handleTextChange = (event: FormEvent<HTMLInputElement>) => {
    setText(event.currentTarget.value);
  };

  return (
    <form className="ChatForm" onSubmit={handleSubmit}>
      <span className="author">{author}</span>
      <input
        type="text"
        autoFocus={true}
        placeholder="What's happening?"
        value={text}
        onChange={handleTextChange}
      />
      <input type="submit" value="Send" disabled={inFlight} />
    </form>
  );
};

export default ChatForm;
