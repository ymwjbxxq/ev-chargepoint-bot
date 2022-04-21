use ev_chargepoint_bot::{
    dtos::telegrams::{
        telegram_request::TelegramRequest
    },
    services::{
        telegram_helper::TelegramHelper, bot::{Bot, BotCommands},
    },
};
use lambda_http::{
    http::StatusCode, service_fn, Error, IntoResponse, Request, RequestExt, Response,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    lambda_http::run(service_fn(|event: Request| execute(event))).await?;

    Ok(())
}

pub async fn execute(event: Request) -> Result<impl IntoResponse, Error> {
    println!("Input {:?}", event);
    let body = event.payload::<TelegramRequest>()?;

    if let Some(body) = body {
        let telegram_helper = TelegramHelper::new(&body);
        let bot = Bot::new(&telegram_helper);
        bot.command_handler().await?;
        bot.location_handler().await?;
    }

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(format!("done"))
        .unwrap();

    Ok(response)
}
