use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use tonic::transport::Server;

mod data;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let addr = "[::1]:8080".parse()?;
    let chat_storage = data::ChatStorage::new();
    let data_dir: PathBuf = std::env::var("DATA_DIR")
        .unwrap_or_else(|_| String::from("."))
        .into();
    let adjectives = Arc::new(data::get_lines(
        File::open(data_dir.join("adjectives.txt")).expect("adjectives.txt"),
    ));
    let animals = Arc::new(data::get_lines(
        File::open(data_dir.join("animals.txt")).expect("animals.txt"),
    ));

    let name_generator = data::NameGenerator::new(adjectives, animals);

    let chatroom = schema::MyChatroom::new(chat_storage, name_generator);

    Server::builder()
        .add_service(schema::chatroom_server::ChatroomServer::new(chatroom))
        .serve(addr)
        .await?;

    Ok(())
}
