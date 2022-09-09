use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::{command, hook};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use crate::START_TIMESTAMP;
use crate::config::ServerConfig;


#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn confignew(ctx: &Context, msg: &Message) -> CommandResult {
    let id = msg.guild_id.unwrap().0;
    let result = ServerConfig::create(id);
    match result {
        Ok(_) => msg.reply(ctx, format!("created config for {id}")),
        Err(m) => msg.reply(ctx, format!("Error occured for {id}: {m}")),
    }.await?;
    Ok(())
}

#[command]
async fn clearconfig(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    if let Ok(v) = SystemTime::now().duration_since(UNIX_EPOCH) {
        msg.reply(ctx, format!("Uptime in seconds: {}", v.as_secs() - *START_TIMESTAMP)).await?;
    } else {
        msg.reply(ctx, "failed to retrieve uptime").await?;
    }
    Ok(())
}

#[hook]
pub(crate) async fn generic_message_handler(ctx: &Context, msg: &Message) {
    println!("{}", msg.content);
    // msg.reply(ctx, "failed to retrieve uptime").await;
}
