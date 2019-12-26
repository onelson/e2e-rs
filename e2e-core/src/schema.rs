use crate::ChatLogEntry;
use chrono::prelude::*;
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("../messages.graphql");

pub struct Context;
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
