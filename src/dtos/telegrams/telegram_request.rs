use serde::{Serialize, Deserialize};

use super::{message::Message, callback_query::CallbackQuery};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelegramRequest {
    pub update_id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,
}