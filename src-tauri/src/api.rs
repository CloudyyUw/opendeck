use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_files::Files;
use serde::{Deserialize, Serialize};

#[allow(clippy::enum_variant_names)]
pub enum Server {
    start
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckButton {
    color: String,
    keys: Vec<String>,
    name: String,
    pos: i32,
    icon: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckTab {
    grid: Vec<i32>,
    bg: String,
    name: String,
    buttons: Vec<DeckButton>
}

#[actix_web::main]
pub async fn start(host: String, port: u16) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting server: http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(Files::new("/", "file_path").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}