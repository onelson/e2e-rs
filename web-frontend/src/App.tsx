import React from "react";
import "./App.css";
import ChatList from "./components/ChatList";
import ChatForm from "./components/ChatForm";

const App: React.FC = () => {
  return (
    <div className="App">
      <ChatList />
      <ChatForm />
    </div>
  );
};

export default App;
