// ----- Serenity Imports -----
use serenity::all::{Context, GuildId};
use serenity::builder::{CreateEmbed, CreateMessage};

// ----- Rust Imports -----

// ----- Internal Imports -----
use crate::discord_api_requests::DiscordAPIRequests;

// --------------------------------------------
// ---------- Faction Member Counter ----------
// --------------------------------------------
/// A struct that handles putting together the faction member counter embed
pub struct FactionMemberCounter;

impl FactionMemberCounter {
    /// Gatheres the data for, and then creates theCreates the faction member counter
    pub async fn create_faction_counter_embed(ctx: &Context, guild_id: GuildId) -> CreateEmbed {
        let role_count = DiscordAPIRequests::get_member_counts(&ctx, guild_id).await;

        let mut embed_description: String = String::new();

        for role in role_count.values() {
            embed_description.push_str(&role.0.to_string());
            embed_description.push_str(&role.1.to_string());
        }

        let embed = CreateEmbed::new()
            .title("Current Faction Playercounts")
            .description(embed_description);

        return embed;
    }
}
