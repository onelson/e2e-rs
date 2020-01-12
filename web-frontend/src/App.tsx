import React from "react";
import "./App.css";
import ChatList from "./components/ChatList";
import ChatForm from "./components/ChatForm";
import { getClient } from "./api-client";

const App: React.FC = () => {
  const client = getClient();
  return (
    <div className="App">
      <ChatList />
      <ChatForm />
    </div>
  );
};

export default App;
