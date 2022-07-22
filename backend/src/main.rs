use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

mod objs;
mod db;
use objs::{DeckButton, DeckGrid, DeckTab, Keys};
use db::{Pool};

#[post("/api/press-keys")]
async fn press_keys(keys: web::Json<Keys>) -> HttpResponse {
    HttpResponse::Ok().json(keys)
}

#[get("/api/tab-by-name/{name}")]
async fn get_tab_by_name(name: web::Path<String>) -> HttpResponse {
    // Execute sql query first
    HttpResponse::Ok().json(DeckTab {
        grid: objs::DeckGrid {
            height: 5,
            width: 5,
        },
        bg: "#3399ff".to_string(),
        name: format!("Tab: {name}"),
        buttons: vec![],
    })
}

#[get("/api/list-decks")]
async fn list_decks() -> HttpResponse {
    // Execute sql query first
    HttpResponse::Ok().json(vec![
        DeckTab {
            grid: DeckGrid {
                height: 5,
                width: 5,
            },
            bg: "#3399ff".to_string(),
            name: "Tab 1".to_string(),
            buttons: vec![
                DeckButton {
                    color: "#3399ff".to_string(),
                    keys: vec!["ctrl".to_string(), "c".to_string()],
                    name: "Copy".to_string(),
                    pos: 1,
                    icon: "http://img.url/img.png".to_string(),
                },
                DeckButton {
                    color: "#3399ff".to_string(),
                    keys: vec!["ctrl".to_string(), "v".to_string()],
                    name: "Paste".to_string(),
                    pos: 1,
                    icon: "http://img.url/img2.png".to_string(),
                },
            ],
        },
        DeckTab {
            grid: DeckGrid {
                height: 5,
                width: 5,
            },
            bg: "#3399ff".to_string(),
            name: "Tab 2".to_string(),
            buttons: vec![
                DeckButton {
                    color: "#3399ff".to_string(),
                    keys: vec!["ctrl".to_string(), "c".to_string()],
                    name: "Copy".to_string(),
                    pos: 1,
                    icon: "http://img.url/img.png".to_string(),
                },
                DeckButton {
                    color: "#3399ff".to_string(),
                    keys: vec!["ctrl".to_string(), "v".to_string()],
                    name: "Paste".to_string(),
                    pos: 1,
                    icon: "http://img.url/img2.png".to_string(),
                },
            ],
        },
    ])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(list_decks)
            .service(get_tab_by_name)
            .service(press_keys)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
