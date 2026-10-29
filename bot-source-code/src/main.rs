// ----- Library Imports -----
use dotenv::dotenv;
use std::env;

// ----- Serenity Imports -----
// use serenity::all::{Channel, GatewayIntents};
use serenity::all::{
    ChannelId, Context, CreateMessage, GatewayIntents, GuildId, Member, Mention, Ready, RoleId,
    User,
};
use serenity::async_trait;
use serenity::prelude::*;
use tokio::time;

// ----- Internal Imports -----
mod faction_member_counter;
use faction_member_counter::FactionMemberCounter;

// -------------------------------
// ---------- Constants ----------
// -------------------------------
// For now these are constants for debugging, but eventually all these variables will be imported from a .config file
// Faction counting
const FACTION_COUNT_CHANNEL_ID: ChannelId = ChannelId::new(1564657237023129650);
const ROLES_TO_KEEP_TRACK_OF: &[&RoleId] = &[&RoleId::new(1565349971035693216)];
const FACTION_COUNTER_UPDATE_INTERVAL_HOURS: u64 = 5;

// Member joining/leaving
const CHANNEL_TO_SEND_JOIN_MESSAGES_TO: ChannelId = ChannelId::new(1565403260435566713);
const CHANNEL_TO_SEND_LEAVE_MESSAGES_TO: ChannelId = ChannelId::new(1565403449502339153);
// const JOIN_MESSAGE: &'static str =
//     "Welcome {user_joined_mention} to Cosmoteer: The Celestial Throne, a RP/RTS server combination";
// Maybe use replace() to set up the ability to edit the leave/join message in the config

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
    //

    /// Posts a joining message when a user joins the discord
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let join_message = format!(
            "Welcome {user_joined_mention} ({username}) to Cosmoteer: The Celestial Throne, a RP/RTS server combination.",
            user_joined_mention = Mention::from(new_member.user.id),
            username = new_member.user.name
        );

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
        let leave_message = format!(
            "Goodbye {user_joined_mention} ({username}) it was nice knowing you.",
            user_joined_mention = Mention::from(user.id),
            username = user.name
        );

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
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
}
