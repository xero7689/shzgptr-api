use super::Handler;
use serde::{Deserialize, Serialize};

/// LLM Client for OpenAI gpt model
pub struct ChatGPTClient<'a> {
    pub api_key: &'a String,
    pub model_id: String,
}

const DEFAULT_SYSTEM_PROMPT: &str = "You're a helpful assistant.";

impl<'a> Handler for ChatGPTClient<'a> {
    async fn chat(
        &self,
        user_message: String,
        system_message: Option<String>,
    ) -> Result<String, reqwest::Error> {
        let system_prompt = match system_message {
            Some(msg) => msg,
            None => DEFAULT_SYSTEM_PROMPT.to_string(),
        };

        let messages = vec![
            ChatGPTMessage {
                role: "system".into(),
                content: system_prompt.into(),
            },
            ChatGPTMessage {
                role: "user".into(),
                content: user_message,
            },
        ];

        let request = ChatGPTChatCompletionsRequest {
            model: self.model_id.clone(),
            messages,
            max_tokens: 1024,
        };
        let response_json = reqwest::Client::new()
            .post("https://api.openai.com/v1/chat/completions")
            .json(&request)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .text()
            .await?;

        let response: ChatGPTChatCompletionsResponse =
            serde_json::from_str(&response_json).unwrap();
        Ok(response.choices[0].message.content.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGPTChatCompletionsRequest {
    pub model: String,
    pub messages: Vec<ChatGPTMessage>,
    pub max_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGPTChatCompletionsResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatCompletionsChoice>,
    pub usage: Usage,
    pub system_fingerprint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGPTMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionsChoice {
    finish_reason: String,
    index: i32,
    pub message: ChatGPTMessage,
    logprobs: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    completion_tokens: i32,
    prompt_tokens: i32,
    total_tokens: i32,
}
