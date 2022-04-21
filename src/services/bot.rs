use async_trait::async_trait;
use crate::{
    dtos::{
        telegrams::sendmessage_params::{
            InlineKeyboardButton, InlineKeyboardMarkup, ParseMode, ReplyKeyboardRemove,
            ReplyMarkup, SendMessageParams,
        },
    },
    error::ApplicationError,
};
use super::{
    plugshare_api::{EvApi, PlugShareApi},
    telegram_api::{BotApi, TelegramApi},
    telegram_helper::TelegramHelper,
};

#[async_trait]
pub trait BotCommands {
    fn new(telegram_helper: &TelegramHelper) -> Self;
    async fn command_handler(&self) -> Result<(), ApplicationError>;
    async fn location_handler(&self) -> Result<(), ApplicationError>;
}

#[derive(Debug)]
pub struct Bot {
    telegram_helper: TelegramHelper,
    telegram_api: TelegramApi,
    ev_api: PlugShareApi,
}

impl Bot {
    async fn welcome(&self) -> Result<(), ApplicationError> {
        let message = format!(
            "Hello {}! Just send me your location and I will find nearby stations!",
            self.telegram_helper.get_user_name()
        );

        let reply_markup = ReplyKeyboardRemove {
            remove_keyboard: true,
            selective: None,
        };

        let send_message = SendMessageParams::new(self.telegram_helper.get_chat_id(), &message)
            .parse_mode(ParseMode::Html)
            .protect_content(true)
            .reply_markup(ReplyMarkup::ReplyKeyboardRemove(reply_markup))
            .build();

        self.telegram_api.send_message(&send_message).await?;

        Ok(())
    }

    async fn stop(&self) -> Result<(), ApplicationError> {
        let message = format!(
            "Hello {}! I have delete all your tracking!",
            self.telegram_helper.get_user_name()
        );
        let send_message = SendMessageParams::new(self.telegram_helper.get_chat_id(), &message)
            .parse_mode(ParseMode::Html)
            .protect_content(true)
            .build();

        self.telegram_api.send_message(&send_message).await?;

        Ok(())
    }

    async fn unknown(&self) -> Result<(), ApplicationError> {
        let message = format!(
            "Hello {}! I did not understand. Here the allow commands..to be implemented!",
            self.telegram_helper.get_user_name()
        );
        let send_message = SendMessageParams::new(self.telegram_helper.get_chat_id(), &message)
            .parse_mode(ParseMode::Html)
            .protect_content(true)
            .build();

        self.telegram_api.send_message(&send_message).await?;

        Ok(())
    }

    async fn details(&self, location_id: i64) -> Result<(), ApplicationError> {
        let location = self.ev_api.get_location(location_id).await?;

        let mut message = format!(
            "<b>ID:</b> {}
<b>Name:</b> {}
<b>Description:</b> {}
<b>Address:</b> {}
<b>Phone:</b> {}
<b>Hours:</b> {}\n\n",
            location.id,
            location.name,
            location.description,
            location.address,
            location.formatted_phone_number,
            location.hours
        );
message.push_str(&format!("<b><i>{} Stations found:</i></b>",location.stations.len()));
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
        for station in location.stations {
            let station_name = station.name.unwrap_or(station.id.to_string());
            message.push_str(&format!("\n\n&#128204; <b>Station Name:</b> {} with {} Outlets", station_name, station.outlets.len()));
            for connector in station.outlets {
                message.push_str(&format!("\n--<b>Connector type:</b> {}\n--<b>Status:</b> {}\n", connector.connector,connector.status.unwrap_or("Unknown".to_string())));
            }
            let button = InlineKeyboardButton {
                text: format!("Track {} - station {}", location.name, station_name),
                callback_data: Some(format!("/track {} {}", location_id, station.id)),
                ..Default::default()
            };
            keyboard.push([button].to_vec());
        }

        let reply_markup = InlineKeyboardMarkup {
            inline_keyboard: keyboard,
        };

        let send_message = SendMessageParams::new(self.telegram_helper.get_chat_id(), &message)
            .parse_mode(ParseMode::Html)
            .protect_content(true)
            .reply_markup(ReplyMarkup::InlineKeyboardMarkup(reply_markup))
            .build();

        self.telegram_api.send_message(&send_message).await?;

        Ok(())
    }

    async fn track(&self, station_id: i64, connector_id: i64) -> Result<(), ApplicationError> {
        let message = format!(
            "Hello {}! the tracking is in place! {} - {}",
            self.telegram_helper.get_user_name(),
            station_id,
            connector_id
        );
        let send_message = SendMessageParams::new(self.telegram_helper.get_chat_id(), &message)
            .parse_mode(ParseMode::Html)
            .protect_content(true)
            .build();

        self.telegram_api.send_message(&send_message).await?;

        Ok(())
    }
}

#[async_trait]
impl BotCommands for Bot {
    fn new(telegram_helper: &TelegramHelper) -> Self {
        Self {
            telegram_helper: telegram_helper.clone(),
            telegram_api: TelegramApi::new(),
            ev_api: PlugShareApi::new(),
        }
    }

    async fn command_handler(&self) -> Result<(), ApplicationError> {
        if let Some(command) = self.telegram_helper.get_text() {
            //let command: Vec<&str> = command.split(" ").collect();
            match command.as_ref() {
                "/start" => {
                    self.welcome().await?;
                }
                "/stop" => {
                    self.stop().await?;
                }
                "/callback_query" => {
                    if let Some(callback_query) = self.telegram_helper.callback_query() {
                        if let Some(command) = callback_query.data {
                            let string_slice: Vec<&str> = command.split(" ").collect();
                            let station_id: i64 = string_slice[1].parse().unwrap();
                            if let "/details" = string_slice[0] {
                                self.details(station_id).await?;
                            } else {
                              let station_id: i64 = string_slice[1].parse().unwrap();
                              let connector_id: i64 = string_slice[2].parse().unwrap();
                              self.track(station_id, connector_id).await?;
                            }
                        }
                    }
                }
                _ => {
                    self.unknown().await?;
                }
            }
        }

        Ok(())
    }

    async fn location_handler(&self) -> Result<(), ApplicationError> {
        if let Some(location) = self.telegram_helper.get_location() {
            let reply_markup = ReplyKeyboardRemove {
                remove_keyboard: true,
                selective: None,
            };

            let message = format!(
                "You are at {} {}. Loading all stations in 2 KM radius.",
                location.latitude, location.longitude
            );
            let send_message = SendMessageParams::new(self.telegram_helper.get_chat_id(), &message)
                .parse_mode(ParseMode::Html)
                .protect_content(true)
                .reply_markup(ReplyMarkup::ReplyKeyboardRemove(reply_markup))
                .build();

            self.telegram_api.send_message(&send_message).await?;

            let locations = self
                .ev_api
                .get_locations(location.latitude, location.longitude)
                .await?;

            let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
            for i in 0..locations.len() {
                let km = format!("{:.2}", locations[i].distance_meters / 1000 as f64);
                let button = InlineKeyboardButton {
                    text: format!(
                        "📍 - {}km - {} - {} - {}",
                        km,
                        locations[i].name,
                        locations[i].address,
                        locations[i].stations[0].available
                    ),
                    callback_data: Some(format!("/details {}", &locations[i].id)),
                    ..Default::default()
                };

                keyboard.push([button].to_vec());
            }

            let reply_markup = InlineKeyboardMarkup {
                inline_keyboard: keyboard,
            };
            let message = format!("Select station to see details");
            let send_message = SendMessageParams::new(self.telegram_helper.get_chat_id(), &message)
                .parse_mode(ParseMode::Html)
                .protect_content(true)
                .reply_markup(ReplyMarkup::InlineKeyboardMarkup(reply_markup))
                .build();
            self.telegram_api.send_message(&send_message).await?;
        }

        Ok(())
    }
}
