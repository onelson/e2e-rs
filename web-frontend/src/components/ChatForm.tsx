import React, { FormEvent, useState } from "react";
import { useApolloClient, useMutation } from "@apollo/react-hooks";
import { GENERATE_USERNAME, CREATE_MESSAGE } from "../api-client";
import "./ChatForm.css";
import { GenerateUsername } from "../_gql/generated/GenerateUsername";
import {
  CreateMessage,
  CreateMessageVariables
} from "../_gql/generated/CreateMessage";

const ChatForm = () => {
  // const [author, setAuthor] = useState<string | null>(null);
  const [text, setText] = useState("");
  const [inFlight, setInFlight] = useState(false);
  const client = useApolloClient();

  const [generateUsername, { data }] = useMutation<GenerateUsername>(
    GENERATE_USERNAME
  );
  const author = data && data.username;
  React.useEffect(() => {
    generateUsername().catch(reason => {
      console.error(reason);
    });
    // eslint-disable-next-line
  }, []);

  const handleSubmit = async (event: FormEvent) => {
    event.preventDefault();

    if (!author) {
      throw new Error("author is not set.");
    }

    setInFlight(true);

    return client
      .mutate<CreateMessage, CreateMessageVariables>({
        mutation: CREATE_MESSAGE,
        variables: {
          msg: {
            author,
            text
          }
        }
      })
      .then(
        () => {
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
