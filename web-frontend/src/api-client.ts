import { ChatroomPromiseClient } from "e2e-client";
export * from "e2e-client";

interface MaybeDevToolWindow extends Window {
  __GRPCWEB_DEVTOOLS__?: any;
}

const _client: ChatroomPromiseClient = new ChatroomPromiseClient(
  "http://localhost:8080"
);

declare const window: MaybeDevToolWindow;
const enableDevTools = window.__GRPCWEB_DEVTOOLS__ || (() => {});
enableDevTools([_client]);

/**
 * Singleton gRPC client getter.
 */
export function getClient(): ChatroomPromiseClient {
  return _client;
}
