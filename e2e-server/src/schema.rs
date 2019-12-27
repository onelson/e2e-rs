use crate::data::{ChatStorage, NameGenerator};
use actix_web::web::Data;
use chrono::prelude::*;
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("../messages.graphql");

#[derive(Clone)]
pub struct Context {
    pub chat_storage: Data<ChatStorage>,
    pub name_generator: Data<NameGenerator>,
}

impl juniper::Context for Context {}

pub struct Query;

impl QueryFields for Query {
    fn field_all_messages<'a>(
        &self,
        executor: &juniper::Executor<'a, Context>,
        _trail: &self::QueryTrail<'_, ChatLogEntry, juniper_from_schema::Walked>,
    ) -> juniper::FieldResult<Vec<ChatLogEntry>> {
        let messages = match executor.context().chat_storage.entries.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        Ok(messages.clone())
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_get_username(
        &self,
        executor: &juniper::Executor<'_, Context>,
    ) -> juniper::FieldResult<String> {
        let username = executor.context().name_generator.get_name();
        {
            let announcement = crate::data::announce_login(&username);
            let mut messages = match executor.context().chat_storage.entries.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            messages.push(announcement);
        }
        Ok(username)
    }
    fn field_create_message(
        &self,
        executor: &juniper::Executor<'_, Context>,
        message: Message,
    ) -> juniper::FieldResult<&bool> {
        let mut messages = match executor.context().chat_storage.entries.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        messages.push(ChatLogEntry::new(message));
        Ok(&true)
    }
}

impl ChatLogEntryFields for ChatLogEntry {
    fn field_author(
        &self,
        _executor: &juniper::Executor<'_, Context>,
    ) -> juniper::FieldResult<&String> {
        Ok(&self.author)
    }
    fn field_text(
        &self,
        _executor: &juniper::Executor<'_, Context>,
    ) -> juniper::FieldResult<&String> {
        Ok(&self.text)
    }
    fn field_timestamp(
        &self,
        _executor: &juniper::Executor<'_, Context>,
    ) -> juniper::FieldResult<&DateTime<Utc>> {
        Ok(&self.timestamp)
    }
}

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}

/// Just a message with a timestamp attached, used for output only.
#[derive(Debug, Clone)]
pub struct ChatLogEntry {
    /// When the entry was collected by the server.
    pub timestamp: DateTime<Utc>,
    /// The author of the message.
    pub author: String,
    /// The message body itself.
    pub text: String,
}

impl ChatLogEntry {
    pub fn new(msg: Message) -> Self {
        Self {
            author: msg.author.clone(),
            text: msg.text.clone(),
            timestamp: Utc::now(),
        }
    }
}
