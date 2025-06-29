pub mod claude;
pub mod openai;

pub trait Handler {
    fn chat(
        &self,
        user_message: String,
        system_message: Option<String>,
    ) -> impl std::future::Future<Output = Result<String, reqwest::Error>> + Send;
}
