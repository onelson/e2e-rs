import { MessagesAPI } from "e2e-client";

// wasm module needs to be imported like this (in the app project).
// The webpack loader can't seem to reason about the deferred import
// when it's seem in a dependency rather than the project itself.
const rust = import("e2e-client-wasm").catch(e => console.error(e));

export function getClient(): MessagesAPI {
  return new MessagesAPI("/api", rust);
}
