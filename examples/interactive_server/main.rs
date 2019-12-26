extern crate actix_web;
extern crate envy;
#[macro_use]
extern crate log;

use crate::requests::{send_actions, send_complete, send_static_select};
use actix_web::middleware::Logger;
use actix_web::{post, App, Error, HttpServer};
use block_kit::config::SlackConfig;
use block_kit::payload::interaction::block_actions::{Actions, BlockActionsPayload};
use block_kit::payload::slash_command::SlashCommandPayload;
use std::collections::HashMap;
use std::{io, process};

mod requests;

#[post("/")]
async fn index(req: String) -> Result<&'static str, Error> {
    let map: HashMap<String, String> = serde_urlencoded::from_str(&req).unwrap_or(HashMap::new());
    info!("{:?}", map);
    let payload = map
        .get("payload")
        .map(|payload| serde_json::from_str::<BlockActionsPayload>(payload).unwrap());

    let config = match envy::from_env::<SlackConfig>() {
        Ok(val) => val,
        Err(err) => {
            error!("{:?}", err);
            process::exit(1);
        }
    };

    if let Some(payload) = payload {
        match &payload.actions[0] {
            Actions {
                selected_option: Some(selected_option),
                ..
            } => {
                if selected_option.value == "deploy" {
                    send_actions(&config).await.unwrap_or(());
                }
            }
            Actions { action_id, .. } => {
                if action_id == "start" {
                    send_complete(&config, payload.response_url.as_str())
                        .await
                        .unwrap_or(());
                }
            }
        };
    }

    Ok("Success")
}

#[post("/slack/receive")]
async fn slash(req: String) -> Result<&'static str, Error> {
    let command: SlashCommandPayload = serde_urlencoded::from_str(&req).unwrap();
    info!("{:?}", command);

    // export SLACK_BOT_TOKEN=xoxb-your-bot-token
    // export SLACK_CHANNEL=random
    let config = match envy::from_env::<SlackConfig>() {
        Ok(val) => val,
        Err(err) => {
            error!("{:?}", err);
            process::exit(1);
        }
    };

    if command.text == "hello" {
        send_static_select(&config).await.unwrap_or(());
    }
    Ok("Success")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(slash)
    })
    .bind("localhost:8000")
    .unwrap()
    .run()
    .await
}
