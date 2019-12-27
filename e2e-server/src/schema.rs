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
        _executor: &juniper::Executor<'a, Context>,
        _trail: &self::QueryTrail<'_, ChatLogEntry, juniper_from_schema::Walked>,
    ) -> juniper::FieldResult<Vec<ChatLogEntry>> {
        Ok(vec![])
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_get_username(
        &self,
        _executor: &juniper::Executor<'_, Context>,
    ) -> juniper::FieldResult<String> {
        Ok(String::from("FIXME"))
    }
    fn field_create_message(
        &self,
        _executor: &juniper::Executor<'_, Context>,
        _message: Message,
    ) -> juniper::FieldResult<&bool> {
        Ok(&true)
    }
}

impl ChatLogEntryFields for ChatLogEntry {
    fn field_author(
        &self,
        _executor: &juniper::Executor<'_, Context>,
    ) -> juniper::FieldResult<&String> {
        Ok(&self.msg.author)
    }
    fn field_text(
        &self,
        _executor: &juniper::Executor<'_, Context>,
    ) -> juniper::FieldResult<&String> {
        Ok(&self.msg.text)
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
#[derive(Debug)]
pub struct ChatLogEntry {
    /// When the entry was collected by the server.
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
