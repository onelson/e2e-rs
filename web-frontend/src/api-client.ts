import { ChatroomPromiseClient } from "e2e-client";
export * from "e2e-client";

let _client: ChatroomPromiseClient | null = null;

/**
 * Singleton gRPC client getter.
 *
 * Do we even really need this? Might make more sense to just return a new one
 * each time.
 */
export function getClient(): ChatroomPromiseClient {
  _client = _client || new ChatroomPromiseClient("http://[::1]:8080");
  return _client;
}
