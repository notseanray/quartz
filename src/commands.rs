use crate::config::DiscordServerConfig;
use crate::START_TIMESTAMP;
use chrono::{Utc, Datelike};
use serenity::framework::standard::macros::{command, hook};
use serenity::framework::standard::{CommandResult, Args};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

// fn query_frc() {
//
// }

#[command]
async fn frcstats(_ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let district = args.raw().find(|x| x == &"district").unwrap_or("PCH");
    let teamNumber = args.raw().find(|x| x == &"team").unwrap_or("9293");
    let body = reqwest::get(format!("https://frc-api.firstinspires.org/v3.0/{}/rankings/district?districtCode={district}", Utc::now().year()))
        .await?
        .text()
        .await?;
    Ok(())
}

#[command]
async fn confignew(ctx: &Context, msg: &Message) -> CommandResult {
    let id = msg.guild_id.unwrap().0;
    let result = DiscordServerConfig::create(id);
    match result {
        Ok(_) => msg.reply(ctx, format!("created config for {id}")),
        Err(m) => msg.reply(ctx, format!("Error occured for {id}: {m}")),
    }
    .await?;
    Ok(())
}

#[command]
async fn clearconfig(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    if let Ok(v) = SystemTime::now().duration_since(UNIX_EPOCH) {
        msg.reply(
            ctx,
            format!("Uptime in seconds: {}", v.as_secs() - *START_TIMESTAMP),
        )
        .await?;
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
