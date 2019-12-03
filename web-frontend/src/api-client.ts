/**
 * So, the e2e_client package has the raw wasm module in it, but in order to
 * get a nicer interface for our callers, we'll want to wrap those functions.
 *
 * We can use the types exported with the wasm output here and use them to make
 * the signatures of those "Promise<any>" fetch calls more specific.
 *
 * Modeling this sort of API entirely in the wasm module is difficult for a
 * couple of reasons, but mainly we have limited control to annotate the return
 * types of functions, and classes from wasm don't mix well with async.
 *
 * We can't currently, for example, have a MessageAPI class that holds the
 * prefix, then also implement async methods on it. The borrow checker complains
 * that `me` doesn't live long enough. This might be resolved later, but for now
 * our async functions will be exported as standalone functions.
 */

import { Message, MessageListResponse } from "e2e-client";

const rust = import("e2e-client/e2e_client").catch(e => console.error(e));

export class MessagesAPI {
  // URI prefix for the backend server
  prefix: string;
  // this is the wasm module...
  rust: Promise<any>;

  constructor(prefix: string) {
    this.prefix = prefix;
    this.rust = rust;
  }

  async createMessage(message: Message): Promise<Response> {
    return this.rust
      .then(r => r.create_message(this.prefix, message))
      .then(resp => {
        // Likely we'd be doing all the response inspection in the wasm layer
        // and converting protocol-level signals into errors or values, but it
        // is also possible to hand the fetch response back to the js layer
        // directly as we are here.
        return resp;
      });
  }

  async getMessages(): Promise<MessageListResponse> {
    return this.rust.then(r => r.get_messages(this.prefix));
  }
}

export function getClient(): MessagesAPI {
  return new MessagesAPI("/api");
}
