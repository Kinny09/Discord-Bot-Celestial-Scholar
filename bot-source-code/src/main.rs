// ----- Library Imports -----
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use rand::Rng;
use std::env;

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
mod live_data_importer_exporter;
mod lore_snippet_importer;

// -------------------------------
// ---------- Constants ----------
// -------------------------------
// For now these are constants for debugging, but eventually all these variables will be imported from a .config file
// lore_snippets
const LORE_SNIPPET_JSON_PATH: &'static str = "lore-snippets.json";
const LORE_SNIPPET_COOLDOWN_TIME_MINUTES: i64 = 5;
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
    last_time_lore_snippet_was_posted: Mutex<DateTime<Utc>>,
}

/// An implementation of the Handler that contains some functions the bot needs
impl Handler {
    /// Checks if the current time has or hasn't passed the cooldown time. If it hasn't, it returns true, if it has, it returns false.
    async fn is_lore_snippet_on_cooldown(&self) -> bool {
        let when_cooldown_ends = self.last_time_lore_snippet_was_posted.lock().await.to_utc()
            + chrono::Duration::minutes(LORE_SNIPPET_COOLDOWN_TIME_MINUTES);

        if when_cooldown_ends < Timestamp::now().to_utc() {
            return true;
        }
        return false;
    }

    async fn format_message(message_to_format: &String, user: User) -> String {
        let mut formatted_message: String = message_to_format.clone();

        if message_to_format.contains("{mention_user}") {
            formatted_message = message_to_format.replace("{mention_user}", &user.to_string());
        }

        if message_to_format.contains("{username}") {
            formatted_message = formatted_message.replace("{username}", &user.name);
        }

        return formatted_message;
    }
}

/// Code for all the serenity specific events
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
                *self.last_time_lore_snippet_was_posted.lock().await = Timestamp::now().to_utc();
            } else {
                let cooldown_message =
                    Self::format_message(&LORE_SNIPPET_COOLDOWN_MESSAGE.to_string(), msg.author)
                        .await;

                if let Err(why) = msg.channel_id.say(&ctx.http, cooldown_message).await {
                    eprintln!("Error sending lore snippet cooldown message: {why:?}");
                }
            }
        }
    }

    /// Posts a joining message when a user joins the discord
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let join_message = Self::format_message(&JOIN_MESSAGE.to_string(), new_member.user).await;

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
        let leave_message = Self::format_message(&LEAVE_MESSAGE.to_string(), user).await;

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
            last_time_lore_snippet_was_posted: Mutex::new(
                Timestamp::now().to_utc()
                    - chrono::Duration::minutes(LORE_SNIPPET_COOLDOWN_TIME_MINUTES),
            ),
        })
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
}
