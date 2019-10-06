use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};
use chrono::prelude::*;
use e2e_core::{ChatLogEntry, Message, MessageListResponse};
use std::sync::Mutex;

/// Data storage for the chats.
struct ChatStorage {
    entries: Mutex<Vec<ChatLogEntry>>,
}

impl ChatStorage {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(vec![]),
        }
    }
}

fn list(data: web::Data<ChatStorage>) -> impl Responder {
    let guard = data.entries.lock().unwrap();
    web::HttpResponse::Ok().json(MessageListResponse::new(&*guard.as_slice()))
}

fn create(body: web::Json<Message>, data: web::Data<ChatStorage>) -> impl Responder {
    let message = body.into_inner();
    let mut guard = data.entries.lock().unwrap();
    let timestamp = Utc::now();
    guard.push(ChatLogEntry { message, timestamp });
    web::HttpResponse::Created().finish()
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let storage = web::Data::new(ChatStorage::new());
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::resource("/messages")
                    .register_data(storage.clone())
                    .route(web::get().to(list))
                    .route(web::post().to(create)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
