use serde::{Deserialize, Serialize};

use crate::dtos::telegrams::{location::Location, telegram_request::TelegramRequest, callback_query::CallbackQuery};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelegramHelper {
    pub request: TelegramRequest,
}

impl TelegramHelper {
    pub fn new(request: &TelegramRequest) -> Self {
        Self {
            request: request.clone(),
        }
    }

    pub fn get_chat_id(&self) -> i64 {
        if let Some(message) = &self.request.message {
            return message.chat.id;
        }

        if let Some(callback_query) = &self.request.callback_query {
            if let Some(message) = &callback_query.message {
                return message.chat.id;
            }
        }

        0
    }

    pub fn callback_query(&self) -> Option<CallbackQuery> {
        if let Some(callback_query) = &self.request.callback_query {
            return Some(callback_query.clone());
        }

        None
    }

    pub fn get_text(&self) -> Option<String> {
        if let Some(message) = &self.request.message {
            if let Some(command) = &message.text {
                return Some(command.to_string());
            }
        }

        if let Some(_callback_query) = &self.request.callback_query {
            return Some("/callback_query".to_string());
        }

        None
    }

    pub fn get_location(&self) -> Option<Location> {
        if let Some(message) = &self.request.message {
            if let Some(location) = message.location {
                return Some(location);
            }
        }

        None
    }

    pub fn get_user_name(&self) -> String {
        let mut user_name = "stranger".to_string();
        if let Some(message) = &self.request.message {
            if let Some(user) = &message.from {
                user_name = user.first_name.clone();
            }
        }

        if let Some(callback_query) = &self.request.callback_query {
            user_name = callback_query.from.first_name.clone();
        }

        user_name
    }
}
