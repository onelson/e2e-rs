use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};
use chrono::prelude::*;
use e2e_core::{ChatLogEntry, Message, MessageListResponse};
use rand::prelude::*;
use std::sync::Mutex;

#[derive(Default)]
struct NameGenerator {
    rng: Mutex<ThreadRng>,
    adjectives: Vec<String>,
    animals: Vec<String>,
}

/// Selects a random adjective and animal to create a new username.
impl NameGenerator {
    pub fn new() -> Self {
        // TODO: read the text data and build the word vecs.

        Self {
            rng: Mutex::new(thread_rng()),
            ..Self::default()
        }
    }

    pub fn get_name(&self) -> String {
        let (adjective, animal) = {
            let guard = self.rng.lock().unwrap();
            let mut rng = *guard;
            (
                self.adjectives.choose(&mut rng).unwrap(),
                self.animals.choose(&mut rng).unwrap(),
            )
        };
        let name = format!("{} {}", adjective, animal);
        println!("`{}` has arrived.", &name);
        name
    }
}

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
    let msg = body.into_inner();
    let mut guard = data.entries.lock().unwrap();
    let timestamp = Utc::now();
    guard.push(ChatLogEntry { msg, timestamp });
    web::HttpResponse::Created().finish()
}

fn username(data: web::Data<NameGenerator>) -> impl Responder {
    web::HttpResponse::Created().body(&data.get_name())
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        let name_data = web::Data::new(NameGenerator::new());
        let storage = web::Data::new(ChatStorage::new());

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::resource("/username")
                    .register_data(name_data)
                    .route(web::post().to(username)),
            )
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
