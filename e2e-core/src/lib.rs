use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// Base fields for a message.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub author: String,
    pub text: String,
}

/// Just a message with a timestamp attached, used for output only.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatLogEntry {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub message: Message,
}

/// This is the response from the List handler.
///
/// Messages in the response are _oldest first_.
#[derive(Clone, Debug, Deserialize, Serialize)]
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
