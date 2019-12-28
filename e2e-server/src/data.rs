//! This module is all about managing the "persistence" layer of the application.
//!
//! Normally you'd be pushing data into a remote data store like a database or
//! something. For this toy app, we're using in-process memory, so our storage
//! is only persistent while the service is running.

use crate::schema::{ChatLogEntry, Message};
use rand::prelude::*;
use rand::rngs::OsRng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct NameGenerator {
    rng: Mutex<OsRng>,
    adjectives: Arc<Vec<String>>,
    animals: Arc<Vec<String>>,
}

/// Break up a file into lines and collect into a vec.
/// Blank lines are omitted, and the text is forced to lowercase.
pub(crate) fn get_lines(file: File) -> Vec<String> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|s| s.as_str().trim() != "")
        .map(|s| s.to_lowercase())
        .collect()
}

/// Selects a random adjective and animal to create a new username.
impl NameGenerator {
    pub fn new(adjectives: Arc<Vec<String>>, animals: Arc<Vec<String>>) -> Self {
        Self {
            rng: Mutex::new(OsRng::default()),
            adjectives,
            animals,
        }
    }

    pub fn get_name(&self) -> String {
        let guard = match self.rng.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let mut rng = *guard;
        // It's possible we have animals who don't have matching adjectives, so
        // loop until we find a pair.
        loop {
            let animal = self.animals.choose(&mut rng).unwrap();
            let adjective = self
                .adjectives
                .iter()
                .filter(|s| s.chars().nth(0) == animal.chars().nth(0))
                .choose(&mut rng);
            if let Some(adjective) = adjective {
                return format!("{} {}", adjective, animal);
            }
            unreachable!("failed to generate a new name.");
        }
    }
}

/// Data storage for the chats.
pub struct ChatStorage {
    entries: Mutex<Vec<ChatLogEntry>>,
}

impl ChatStorage {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(vec![]),
        }
    }

    /// Prepare a system message to announce a user when they connect.
    pub fn announce_login(&self, name: &str) {
        let msg = Message {
            author: "SYSTEM".to_string(),
            text: format!("`{}` has logged on.", &name),
        };
        self.publish_message(msg);
    }

    pub fn publish_message(&self, msg: Message) {
        let mut messages = match self.entries.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        messages.push(ChatLogEntry::new(msg));
    }

    pub fn all_messages(&self) -> Vec<ChatLogEntry> {
        let messages = match self.entries.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        messages.clone()
    }
}
