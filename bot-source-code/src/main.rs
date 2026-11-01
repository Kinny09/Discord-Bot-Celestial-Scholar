// ----- Library Imports -----
use chrono::{DateTime, Local, TimeZone};
use dotenv::dotenv;
use rand::Rng;
use std::env;
use std::fs;
use std::ops::Add;

// ----- Serenity Imports -----
use serenity::all::{
    ChannelId, Context, CreateMessage, GatewayIntents, GuildId, Member, Message, Ready, RoleId,
    Timestamp, User,
};
use serenity::async_trait;
use serenity::prelude::*;
use tokio::sync::Mutex;
use tokio::time;

// ----- Internal Imports -----
mod faction_member_counter;
use faction_member_counter::FactionMemberCounter;
mod lore_snippet_importer;

// -------------------------------
// ---------- Constants ----------
// -------------------------------
// For now these are constants for debugging, but eventually all these variables will be imported from a .config file
// lore_snippets
const LORE_SNIPPET_JSON_PATH: &'static str = "lore-snippets.json";
const LORE_SNIPPET_COOLDOWN_TIME_MINUTES: u64 = 5;
const LORE_SNIPPET_COOLDOWN_MESSAGE: &'static str =
    "Please be patient {mention_user}, I'm currently going through my notes.";

// Faction counting
const FACTION_COUNT_CHANNEL_ID: ChannelId = ChannelId::new(1564657237023129650);
const ROLES_TO_KEEP_TRACK_OF: &[&RoleId] = &[&RoleId::new(1565349971035693216)];
const FACTION_COUNTER_UPDATE_INTERVAL_HOURS: u64 = 5;

// Member joining/leaving
const CHANNEL_TO_SEND_JOIN_MESSAGES_TO: ChannelId = ChannelId::new(1565403260435566713);
const CHANNEL_TO_SEND_LEAVE_MESSAGES_TO: ChannelId = ChannelId::new(1565403449502339153);
const JOIN_MESSAGE: &'static str = "Welcome {mention_user} ({username}) to Cosmoteer: The Celestial Throne, a RP/RTS server combination.";
const LEAVE_MESSAGE: &'static str = "Goodbye {mention_user} ({username}) it was nice knowing you.";

// ---------------------------------------
// ---------- The event handler ----------
// ---------------------------------------
struct Handler {
    last_time_lore_snippet_was_posted: Mutex<Timestamp>,
}

impl Handler {
    async fn is_lore_snippet_on_cooldown(&self) -> bool {
        let when_cooldown_ends =
            self.last_time_lore_snippet_was_posted
                .lock()
                .await
                .add(time::Duration::from_secs(
                    LORE_SNIPPET_COOLDOWN_TIME_MINUTES,
                ));

        if when_cooldown_ends < Timestamp::now().to_utc() {
            return true;
        }
        return false;
    }
}

#[async_trait]
impl EventHandler for Handler {
    /// Posts a lore snippet when someone @'s the bot
    async fn message(&self, ctx: Context, msg: Message) {
        let bot_id = ctx.cache.current_user().id;

        if msg.mentions_user_id(bot_id) {
            if Self::is_lore_snippet_on_cooldown(&self).await {
                let lore_snippets: Vec<String> =
                    lore_snippet_importer::read_lore_snippet_json(LORE_SNIPPET_JSON_PATH)
                        .await
                        .unwrap();

                let random_snippet_index = rand::thread_rng().gen_range(0..lore_snippets.len());
                let random_snippet: String = lore_snippets[random_snippet_index].clone();

                if let Err(why) = msg.channel_id.say(&ctx.http, random_snippet).await {
                    eprintln!("Error sending lore snippet: {why:?}");
                }
                *self.last_time_lore_snippet_was_posted.lock().await = Timestamp::now();
            } else {
                let cooldown_message = LORE_SNIPPET_COOLDOWN_MESSAGE
                    .replace("{mention_user}", &msg.author.to_string());

                if let Err(why) = msg.channel_id.say(&ctx.http, cooldown_message).await {
                    eprintln!("Error sending lore snippet cooldown message: {why:?}");
                }
            }
        }
    }

    /// Posts a joining message when a user joins the discord
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let mut join_message = JOIN_MESSAGE.replace("{mention_user}", &new_member.user.to_string());
        join_message = join_message.replace("{username}", &new_member.user.name);

        if let Err(why) = CHANNEL_TO_SEND_JOIN_MESSAGES_TO
            .send_message(&ctx.http, CreateMessage::new().content(join_message))
            .await
        {
            eprintln!("Error sending welcome message: {why:?}");
        }
    }

    /// Posts a leaving message when a user leaves the discord
    async fn guild_member_removal(
        &self,
        ctx: Context,
        _guild_id: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        let mut leave_message = LEAVE_MESSAGE.replace("{mention_user}", &user.to_string());
        leave_message = leave_message.replace("{username}", &user.name);

        if let Err(why) = CHANNEL_TO_SEND_LEAVE_MESSAGES_TO
            .send_message(&ctx.http, CreateMessage::new().content(leave_message))
            .await
        {
            eprintln!("Error sending welcome message: {why:?}");
        }
    }

    /// Handles the updating of the faction counter at specific intervals
    async fn ready(&self, ctx: Context, ready: Ready) {
        let mut faction_counter_update_interval = time::interval(time::Duration::from_hours(
            FACTION_COUNTER_UPDATE_INTERVAL_HOURS,
        ));

        loop {
            faction_counter_update_interval.tick().await;
            FactionMemberCounter::update_faction_counter(
                ctx.clone(),
                ready.clone(),
                FACTION_COUNT_CHANNEL_ID,
                ROLES_TO_KEEP_TRACK_OF,
            )
            .await;
        }
    }
}

// ---------------------------------------
// ---------- The main functiion ---------
// ---------------------------------------
#[tokio::main]
async fn main() {
    // Read the enviromental file
    dotenv().ok();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            last_time_lore_snippet_was_posted: Mutex::new(Timestamp::now()),
        })
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
}
