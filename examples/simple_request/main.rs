#[macro_use]
extern crate log;

use crate::requests::send_hello_world;
use block_kit::config::SlackConfig;
use envy::from_env;
use std::process;

mod requests;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let config = match from_env::<SlackConfig>() {
        Ok(val) => val,
        Err(err) => {
            error!("{}", err);
            process::exit(1);
        }
    };

    let _ = send_hello_world(&config).await;
    Ok(())
}
