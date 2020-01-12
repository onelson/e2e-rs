import React, { FormEvent, useState } from "react";
import { getClient, IdentityCreateRequest, Message } from "../api-client";
import "./ChatForm.css";

const ChatForm = () => {
  const [author, setAuthor] = useState<string | null>(null);
  const [text, setText] = useState("");
  const [inFlight, setInFlight] = useState(false);
  const client = getClient();

  React.useEffect(() => {
    client.getIdentity(new IdentityCreateRequest()).then(
      resp => {
        console.debug(resp);
        setAuthor(resp.getUsername());
      },
      reason => {
        console.error(reason);
      }
    );
    // eslint-disable-next-line
  }, []);

  const handleSubmit = async (event: FormEvent) => {
    event.preventDefault();

    if (!author) {
      throw new Error("author is not set.");
    }

    setInFlight(true);
    const msg = new Message();
    msg.setAuthor(author);
    msg.setText(text);
    return client
      .createMessage(msg)
      .then(
        resp => {
          console.debug(resp);
          // reset the form once the request is complete.
          setText("");
        },
        reason => {
          console.error(reason);
        }
      )
      .finally(() => {
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
