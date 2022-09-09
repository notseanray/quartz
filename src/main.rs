use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::sync::Arc;
use tokio::sync::RwLock;

use firebase::Firebase;
use serde::Deserialize;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::framework::standard::StandardFramework;
use lazy_static::lazy_static;
use serenity::framework::standard::macros::group;
mod firebase;
use std::error::Error;
use async_once::AsyncOnce;

mod commands;
use commands::*;
mod config;

#[group]
#[commands(ping, uptime)]
pub(crate) struct General;

lazy_static! {
    pub(crate) static ref START_TIMESTAMP: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_secs();
    pub(crate) static ref CONFIG: Config = envy::from_env::<Config>().expect("failed to get config values");
    pub(crate) static ref FIREBASE_SESSION: AsyncOnce<Arc<RwLock<Option<Firebase>>>> = AsyncOnce::new(async {
        let fb = Firebase::new(&CONFIG.firebase_credentials).ok();
        Arc::new(RwLock::new(fb))
    });
}

#[derive(Deserialize, Debug)]
struct Config {
    token: String,
    prefix: String,
    firebase_credentials: String,
}


struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let framework = StandardFramework::new()
        .normal_message(generic_message_handler)
        .configure(|c| c.prefix(&CONFIG.prefix)) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);
    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&CONFIG.token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    if let Some(v) = &*(FIREBASE_SESSION.get()).await.read().await {
        v.fetch_registration().unwrap();
    }
    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
    Ok(())
}
