import React from "react";
import { ApolloProvider } from "@apollo/react-hooks";
import "./App.css";
import ChatList from "./components/ChatList";
import ChatForm from "./components/ChatForm";
import { getClient } from "./api-client";

const App: React.FC = () => {
  const client = getClient();
  return (
    <ApolloProvider client={client}>
      <div className="App">
        <ChatList />
        <ChatForm />
      </div>
    </ApolloProvider>
  );
};

export default App;
