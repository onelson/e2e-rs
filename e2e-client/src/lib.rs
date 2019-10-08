//! Adapted largely from the examples in the wasm-bindgen book, such as
//!
//! - https://rustwasm.github.io/docs/wasm-bindgen/examples/fetch.html
//!
pub use e2e_core::{ChatLogEntry, Message, MessageListResponse};
use futures::{future, Future};
use js_sys::Promise;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub struct MessagesAPI {
    prefix: String,
}

#[wasm_bindgen]
impl MessagesAPI {
    /// Create a new API client given a url prefix.
    #[wasm_bindgen(constructor)]
    pub fn new(prefix: &str) -> Self {
        let prefix = prefix.to_string();
        Self { prefix }
    }

    #[wasm_bindgen]
    pub fn get_messages(&self) -> Promise {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);
        let url = format!("{}/messages", &self.prefix);
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        request.headers().set("Accept", "application/json").unwrap();

        let window = web_sys::window().unwrap();
        let request_promise = window.fetch_with_request(&request);

        let future = JsFuture::from(request_promise)
            .and_then(|resp_value| {
                // `resp_value` is a `Response` object.
                assert!(resp_value.is_instance_of::<Response>());
                let resp: Response = resp_value.dyn_into().unwrap();
                resp.json()
            })
            .and_then(|json_value: Promise| {
                // Convert this other `Promise` into a rust `Future`.
                JsFuture::from(json_value)
            })
            .and_then(|json| {
                // Use serde to parse the JSON into a struct.
                let message_list: MessageListResponse = json.into_serde().unwrap();

                // Send the `Branch` struct back to JS as an `Object`.
                future::ok(JsValue::from_serde(&message_list).unwrap())
            });

        // Convert this Rust `Future` back into a JS `Promise`.
        future_to_promise(future)
    }

    #[wasm_bindgen]
    pub fn create_message(&self, message: &JsValue) -> Promise {
        let message: Message = message.into_serde().unwrap();
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);
        opts.body(Some(&JsValue::from(
            serde_json::to_string(&message).unwrap(),
        )));

        let url = format!("{}/messages", &self.prefix);
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        request.headers().set("Accept", "application/json").unwrap();
        request
            .headers()
            .set("Content-Type", "application/json")
            .unwrap();

        let window = web_sys::window().unwrap();
        window.fetch_with_request(&request)
    }
}
