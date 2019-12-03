//! Adapted largely from the examples in the wasm-bindgen book, such as
//!
//! - https://rustwasm.github.io/docs/wasm-bindgen/examples/fetch.html
//!
pub use e2e_core::{ChatLogEntry, Message, MessageListResponse};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};


/// Fetch chat messages from the server.
#[wasm_bindgen]
pub async fn get_messages(prefix: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url = format!("{}/messages", &prefix);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let json_value = JsFuture::from(resp.json()?).await?;
    // Use serde to parse the JSON into a struct.
    let message_list: MessageListResponse = json_value.into_serde().unwrap();
    Ok(JsValue::from_serde(&message_list).unwrap())
}

/// Create a new chat message and persist it.
#[wasm_bindgen]
pub async fn create_message(prefix: String, message: JsValue) -> Result<JsValue, JsValue> {
    let message: Message = message.into_serde().unwrap();
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from(
        serde_json::to_string(&message).unwrap(),
    )));

    let url = format!("{}/messages", &prefix);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json").unwrap();
    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();

    let window = web_sys::window().unwrap();
    Ok(JsFuture::from(window.fetch_with_request(&request)).await?)
}

/// Request a new username from the server.
#[wasm_bindgen]
pub async fn get_username(prefix: String, message: JsValue) -> Result<JsValue, JsValue> {
    let message: Message = message.into_serde().unwrap();
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    let url = format!("{}/username", &prefix);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    Ok(JsFuture::from(window.fetch_with_request(&request)).await?)
}
