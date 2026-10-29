// ----- Library Imports -----
use dotenv::dotenv;
use std::env;

// ----- Serenity Imports -----
use serenity::all::GatewayIntents;
use serenity::all::{ChannelId, Context, Ready, RoleId};
use serenity::async_trait;
use serenity::prelude::*;

// ----- Internal Imports -----
mod faction_member_counter;
use faction_member_counter::FactionMemberCounter;

// -------------------------------
// ---------- Constants ----------
// -------------------------------
// For now these are constants for debugging, but eventually all these variables will be imported from a .config file
const FACTION_COUNT_CHANNEL_ID: ChannelId = ChannelId::new(1564657237023129650);
const ROLES_TO_KEEP_TRACK_OF: &[&RoleId] = &[&RoleId::new(1565349971035693216)];

// ---------------------------------------
// ---------- The event handler ----------
// ---------------------------------------
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // async fn message(&self, ctx: Context, msg: Message) {
    //     if msg.content == "!ping" {
    //         if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
    //             println!("Error sending message: {why:?}");
    //         }
    //     }
    // }

    async fn ready(&self, ctx: Context, ready: Ready) {
        FactionMemberCounter::update_faction_counter(
            ctx,
            ready,
            FACTION_COUNT_CHANNEL_ID,
            ROLES_TO_KEEP_TRACK_OF,
        )
        .await;
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
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
