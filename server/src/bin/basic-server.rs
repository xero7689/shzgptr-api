use std::env;
use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

#[path = "../handlers.rs"]
mod handlers;

#[path = "../routes.rs"]
mod routes;

#[path = "../state.rs"]
mod state;

use routes::general_routes;
use state::AppState;

/*
enum FoundationModel {
    ChatGPT,
    Claude,
    Gemini,
}

impl FromStr for FoundationModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "chatgpt" => Ok(FoundationModel::ChatGPT),
            "claude" => Ok(FoundationModel::Claude),
            "gemini" => Ok(FoundationModel::Gemini),
            _ => Err(format!("Unknown Foundation Model: {}", s)),
        }
    }
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let shared_data = web::Data::new(AppState {
        visit_count: Mutex::new(0),
        system_openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY"),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
    };

    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}
