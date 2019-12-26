use chrono::prelude::*;
pub mod schema;

/// Just a message with a timestamp attached, used for output only.
#[derive(Debug)]
pub struct ChatLogEntry {
    /// When the entry was collected by the server.
    pub timestamp: DateTime<Utc>,
    /// The message itself.
    pub msg: schema::Message,
}

impl ChatLogEntry {
    pub fn new(msg: schema::Message) -> Self {
        Self {
            msg,
            timestamp: Utc::now(),
        }
    }
}
