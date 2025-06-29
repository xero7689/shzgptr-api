use super::state::AppState;
use actix_web::{web, HttpResponse};

use actix_web::{get, Responder};
use serde::{Deserialize, Serialize};

use agent::openai::ChatGPTClient;
use agent::Handler;

#[derive(Deserialize)]
pub struct ChatRequest {
    message: String,
    system_message: Option<String>,
}

#[derive(Serialize)]
pub struct ChatResponse {
    response: String,
}

/// General chat endpoint
pub async fn chat(app_state: web::Data<AppState>, request: web::Json<ChatRequest>) -> HttpResponse {
    let llm_client = ChatGPTClient {
        api_key: &app_state.system_openai_api_key,
        model_id: "gpt-4o-mini".to_string(),
    };

    let response = llm_client
        .chat(request.message.clone(), request.system_message.clone())
        .await
        .unwrap();

    let chat_response = ChatResponse { response };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(chat_response)
}

/// Default endpoint for testing
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
