use async_trait::async_trait;
use crate::{error::ApplicationError, dtos::telegrams::sendmessage_params::SendMessageParams};

#[async_trait]
pub trait BotApi {
    fn new() -> Self;
    async fn send_message(&self, params: &SendMessageParams) -> Result<(), ApplicationError>;
}

#[derive(Debug)]
pub struct TelegramApi {
    base_url: String,
}

#[async_trait]
impl BotApi for TelegramApi {
    fn new() -> Self {
        let api_key = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
        let base_url = format!("{}{}", "https://api.telegram.org/bot", api_key);

        Self { base_url }
    }

    async fn send_message(&self, params: &SendMessageParams) -> Result<(), ApplicationError> {
        let url = format!("{}{}", self.base_url, "/sendMessage");

        let client = reqwest::Client::new();
        let res = client.post(url)
            .json(params)
            .send()
            .await?;
        
        if res.status() != 200 {
            return Err(ApplicationError::InternalError(format!("cannot post {}", res.status())));
        }

        Ok(())
    }
}
