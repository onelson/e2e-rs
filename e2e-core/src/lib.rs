use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "typescript")]
use typescript_definitions::TypescriptDefinition;
#[cfg(feature = "typescript")]
use wasm_bindgen::prelude::*;

/// Base fields for a message.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(TypescriptDefinition))]
pub struct Message {
    pub author: String,
    pub text: String,
}

/// Just a message with a timestamp attached, used for output only.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(TypescriptDefinition))]
pub struct ChatLogEntry {
    /// When the entry was collected by the server.
    #[cfg_attr(feature = "typescript", ts(ts_type = "string"))]
    pub timestamp: DateTime<Utc>,
    /// The message itself.
    pub msg: Message,
}

impl ChatLogEntry {
    pub fn new(msg: Message) -> Self {
        Self {
            msg,
            timestamp: Utc::now(),
        }
    }
}

/// This is the response from the List handler.
///
/// Messages in the response are _oldest first_.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(TypescriptDefinition))]
pub struct MessageListResponse {
    messages: Vec<ChatLogEntry>,
}

impl MessageListResponse {
    pub fn new(messages: &[ChatLogEntry]) -> Self {
        Self {
            messages: messages.to_vec(),
        }
    }
}
