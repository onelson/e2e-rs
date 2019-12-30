use crate::data::{ChatStorage, NameGenerator};
use actix_web::web::Data;
use chrono::prelude::*;
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("schema.graphql");

pub struct Context {
    pub chat_storage: Data<ChatStorage>,
    pub name_generator: Data<NameGenerator>,
}

impl juniper::Context for Context {}

#[derive(Debug, Clone)]
pub struct Message {
    pub author: String,
    pub text: String,
}

impl From<NewMessage> for Message {
    fn from(NewMessage { author, text }: NewMessage) -> Self {
        Self { author, text }
    }
}

impl MessageFields for Message {
    fn field_author<'a>(
        &self,
        _executor: &juniper::Executor<'a, Context>,
    ) -> juniper::FieldResult<&String> {
        Ok(&self.author)
    }
    fn field_text<'a>(
        &self,
        _executor: &juniper::Executor<'a, Context>,
    ) -> juniper::FieldResult<&String> {
        Ok(&self.text)
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_all_messages<'a>(
        &self,
        executor: &juniper::Executor<'a, Context>,
        _trail: &self::QueryTrail<'_, ChatLogEntry, juniper_from_schema::Walked>,
    ) -> juniper::FieldResult<Vec<ChatLogEntry>> {
        Ok(executor.context().chat_storage.all_messages())
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_get_username(
        &self,
        executor: &juniper::Executor<'_, Context>,
    ) -> juniper::FieldResult<String> {
        let username = executor.context().name_generator.get_name();
        executor.context().chat_storage.announce_login(&username);
        Ok(username)
    }
    fn field_create_message(
        &self,
        executor: &juniper::Executor<'_, Context>,
        message: NewMessage,
    ) -> juniper::FieldResult<&bool> {
        executor
            .context()
            .chat_storage
            .publish_message(message.into());
        Ok(&true)
    }
}

impl ChatLogEntryFields for ChatLogEntry {
    fn field_msg(
        &self,
        _executor: &juniper::Executor<'_, Context>,
        _trail: &self::QueryTrail<'_, Message, juniper_from_schema::Walked>,
    ) -> juniper::FieldResult<&Message> {
        Ok(&self.msg)
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
    pub msg: Message,
    /// When the entry was collected by the server.
    pub timestamp: DateTime<Utc>,
}

impl ChatLogEntry {
    pub fn new(msg: Message) -> Self {
        Self {
            msg,
            timestamp: Utc::now(),
        }
    }
}
