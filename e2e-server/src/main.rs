extern crate juniper;
use crate::data::{ChatStorage, NameGenerator};
use crate::schema::Context;
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use juniper::http::GraphQLRequest;
use schema::{create_schema, Schema};
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

mod data;
mod schema;

async fn graphiql() -> impl Responder {
    let html = juniper::graphiql::graphiql_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    name_generator: web::Data<NameGenerator>,
    chat_storage: web::Data<ChatStorage>,
    graph_query: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        name_generator: name_generator.clone(),
        chat_storage: chat_storage.clone(),
    };
    let body = web::block(move || {
        let res = graph_query.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let chat_storage = web::Data::new(data::ChatStorage::new());

    let data_dir: PathBuf = std::env::var("DATA_DIR")
        .unwrap_or_else(|_| String::from("."))
        .into();
    let adjectives = Arc::new(data::get_lines(
        File::open(data_dir.join("adjectives.txt")).expect("adjectives.txt"),
    ));
    let animals = Arc::new(data::get_lines(
        File::open(data_dir.join("animals.txt")).expect("animals.txt"),
    ));

    let schema = web::Data::new(Arc::new(create_schema()));
    let name_generator = web::Data::new(data::NameGenerator::new(adjectives, animals));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(schema.clone())
            .app_data(name_generator.clone())
            .app_data(chat_storage.clone())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
