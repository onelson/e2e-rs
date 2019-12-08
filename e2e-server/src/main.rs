use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};
use e2e_core::{ChatLogEntry, Message, MessageListResponse};
use rand::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct NameGenerator {
    rng: Mutex<ThreadRng>,
    adjectives: Arc<Vec<String>>,
    animals: Arc<Vec<String>>,
}

/// Break up a file into lines and collect into a vec.
/// Blank lines are omitted, and the text is forced to lowercase.
fn get_lines(file: File) -> Vec<String> {
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
            rng: Mutex::new(thread_rng()),
            adjectives,
            animals,
        }
    }

    pub fn get_name(&self) -> String {
        let guard = self.rng.lock().unwrap();
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
    let messages = data.entries.lock().unwrap();
    log::debug!("History: {}", messages.len());
    web::HttpResponse::Ok().json(MessageListResponse::new(&*messages.as_slice()))
}

fn create(body: web::Json<Message>, data: web::Data<ChatStorage>) -> impl Responder {
    let msg = body.into_inner();
    {
        let mut messages = data.entries.lock().unwrap();
        messages.push(ChatLogEntry::new(msg));
    }
    web::HttpResponse::Created().finish()
}

fn username(name_gen: web::Data<NameGenerator>, data: web::Data<ChatStorage>) -> impl Responder {
    let name = name_gen.get_name();

    let msg = Message {
        author: "SYSTEM".to_string(),
        text: format!("`{}` has logged on.", &name),
    };

    let mut messages = data.entries.lock().unwrap();
    messages.push(ChatLogEntry::new(msg));

    web::HttpResponse::Created().body(&name)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let storage = web::Data::new(ChatStorage::new());

    let data_dir: PathBuf = std::env::var("DATA_DIR").unwrap().into();
    let adjectives = Arc::new(get_lines(
        File::open(data_dir.join("adjectives.txt")).unwrap(),
    ));
    let animals = Arc::new(get_lines(File::open(data_dir.join("animals.txt")).unwrap()));

    HttpServer::new(move || {
        let name_data = web::Data::new(NameGenerator::new(adjectives.clone(), animals.clone()));

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::resource("/username")
                    .register_data(name_data)
                    .register_data(storage.clone())
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
