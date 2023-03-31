use serenity::model::prelude::*;
use serenity::model::user::User;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tokio::sync::RwLock;

use firebase::Firebase;
use lazy_static::lazy_static;
use serde::Deserialize;
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::model::prelude::Channel;
use serenity::prelude::*;
mod firebase;
use async_once::AsyncOnce;
use std::error::Error;

mod models;

mod markov;

mod commands;
use commands::*;
mod config;

#[group]
#[commands(ping, uptime, frcstats)]
pub(crate) struct General;

lazy_static! {
    pub(crate) static ref START_TIMESTAMP: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_secs();
    pub(crate) static ref CONFIG: Config =
        envy::from_env::<Config>().expect("failed to get config values");
    pub(crate) static ref FIREBASE_SESSION: AsyncOnce<Arc<RwLock<Option<Firebase>>>> =
        AsyncOnce::new(async {
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
impl EventHandler for Handler {
    fn cache_ready<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _guilds: Vec<GuildId>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn channel_create<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _channel: &'life1 GuildChannel,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn category_create<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _category: &'life1 ChannelCategory,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn category_delete<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _category: &'life1 ChannelCategory,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn channel_delete<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _channel: &'life1 GuildChannel,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn channel_pins_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _pin: ChannelPinsUpdateEvent,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }

    fn channel_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _old: Option<Channel>,
        _new: Channel,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_ban_addition<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _guild_id: GuildId,
        _banned_user: User,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_ban_removal<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _guild_id: GuildId,
        _unbanned_user: User,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_create<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _guild: Guild,
        _is_new: bool,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }

    fn guild_delete<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _incomplete: UnavailableGuild,
        _full: Option<Guild>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_emojis_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _guild_id: GuildId,
        _current_state: HashMap<EmojiId, Emoji>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_integrations_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _guild_id: GuildId,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_member_addition<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _new_member: Member,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_member_removal<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _guild_id: GuildId,
        _user: User,
        _member_data_if_available: Option<Member>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_member_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _old_if_available: Option<Member>,
        _new: Member,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_role_create<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _new: Role,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_role_delete<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _guild_id: GuildId,
        _removed_role_id: RoleId,
        _removed_role_data_if_available: Option<Role>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_role_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _old_data_if_available: Option<Role>,
        _new: Role,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_stickers_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _guild_id: GuildId,
        _current_state: HashMap<StickerId, Sticker>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _old_data_if_available: Option<Guild>,
        _new_but_incomplete: PartialGuild,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn invite_create<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _data: InviteCreateEvent,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn invite_delete<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _data: InviteDeleteEvent,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn message_delete<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _channel_id: ChannelId,
        _deleted_message_id: MessageId,
        _guild_id: Option<GuildId>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn message_delete_bulk<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _channel_id: ChannelId,
        _multiple_deleted_messages_ids: Vec<MessageId>,
        _guild_id: Option<GuildId>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn message_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _old_if_available: Option<Message>,
        _new: Option<Message>,
        _event: MessageUpdateEvent,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn reaction_add<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _add_reaction: Reaction,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn reaction_remove<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _removed_reaction: Reaction,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn reaction_remove_all<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _channel_id: ChannelId,
        _removed_from_message_id: MessageId,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn typing_start<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        __arg2: TypingStartEvent,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn voice_state_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _old: Option<VoiceState>,
        _new: VoiceState,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn webhook_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _guild_id: GuildId,
        _belongs_to_channel_id: ChannelId,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn integration_create<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _integration: Integration,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn integration_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _integration: Integration,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn integration_delete<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _integration_id: IntegrationId,
        _guild_id: GuildId,
        _application_id: Option<ApplicationId>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn stage_instance_create<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _stage_instance: StageInstance,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn stage_instance_update<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _stage_instance: StageInstance,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn stage_instance_delete<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _stage_instance: StageInstance,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn thread_create<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _thread: GuildChannel,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn thread_delete<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _thread: PartialGuildChannel,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_scheduled_event_create<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _event: ScheduledEvent,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
    fn guild_scheduled_event_delete<'life0, 'async_trait>(
        &'life0 self,
        _ctx: Context,
        _event: ScheduledEvent,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        unimplemented!();
    }
}

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
