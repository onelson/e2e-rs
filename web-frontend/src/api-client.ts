import { MessagesAPI } from "e2e-client";

export function getClient(): MessagesAPI {
  return new MessagesAPI("/api");
}
