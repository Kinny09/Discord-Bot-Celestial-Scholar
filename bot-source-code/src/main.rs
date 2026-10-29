// ----- Library Imports -----
use dotenv::dotenv;
use std::env;

// ----- Serenity Imports -----
use serenity::all::GatewayIntents;
use serenity::all::{ChannelId, Context, CreateMessage, EditMessage, GetMessages, Ready, RoleId};
use serenity::async_trait;
use serenity::prelude::*;

// ----- Internal Imports -----
mod discord_api_requests;
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

// ------------------------------------
// ---------- Other Functions ---------
// ------------------------------------

// /// Updated the faction counter in the faction counter channel. Will edit the counter message if it exists, if not, it will post a new message.
// async fn update_faction_counter(ctx: Context, ready: Ready) {
//     let bot_id = ctx.cache.current_user().id;
//     let guild_id = ready.guilds[0].id;
//
//     let most_recent_message_in_faction_counter_channel = FACTION_COUNT_CHANNEL_ID
//         .messages(&ctx.http, GetMessages::new().limit(1))
//         .await
//         .unwrap();
//
//     if most_recent_message_in_faction_counter_channel.len() > 0
//         && most_recent_message_in_faction_counter_channel[0].author.id == bot_id
//     {
//         // Edits the message embed if it exists
//         let message_id = most_recent_message_in_faction_counter_channel[0].id;
//         if let Err(error) = FACTION_COUNT_CHANNEL_ID
//             .edit_message(
//                 &ctx.http,
//                 message_id,
//                 EditMessage::new().embed(
//                     FactionMemberCounter::create_faction_counter_embed(
//                         &ctx,
//                         guild_id,
//                         ROLES_TO_KEEP_TRACK_OF,
//                     )
//                     .await,
//                 ),
//             )
//             .await
//         {
//             eprintln!("Failed to edit faction counter message: {error}");
//         }
//     } else {
//         // Posts a new message with an embed if it dosen't already exist
//         if let Err(error) = FACTION_COUNT_CHANNEL_ID
//             .send_message(
//                 &ctx.http,
//                 CreateMessage::new().embed(
//                     FactionMemberCounter::create_faction_counter_embed(
//                         &ctx,
//                         guild_id,
//                         ROLES_TO_KEEP_TRACK_OF,
//                     )
//                     .await,
//                 ),
//             )
//             .await
//         {
//             eprintln!("Failed to send new faction counter message: {error}");
//         }
//     }
// }
