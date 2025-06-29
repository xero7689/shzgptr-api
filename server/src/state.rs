use std::sync::Mutex;

pub struct AppState {
    pub visit_count: Mutex<i32>,
    pub system_openai_api_key: String,
}
