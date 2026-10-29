// ----- Serenity Imports -----
use serenity::all::{Context, GuildId, Timestamp};
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::timestamp;

// ----- Rust Imports -----

// ----- Internal Imports -----
use crate::discord_api_requests::DiscordAPIRequests;

// --------------------------------------------
// ---------- Faction Member Counter ----------
// --------------------------------------------
/// A struct that handles putting together the faction member counter embed
pub struct FactionMemberCounter;

impl FactionMemberCounter {
    /// Gatheres the data for, and then creates the faction member counter
    pub async fn create_faction_counter_embed(ctx: &Context, guild_id: GuildId) -> CreateEmbed {
        let role_count = DiscordAPIRequests::get_member_counts(&ctx, guild_id).await;

        let mut embed_description: String = String::new();

        let timestamp_for_post = format!(
            "-# Last updated on <t:{unix_timestamp}:f> which was <t:{unix_timestamp}:R>. \n",
            unix_timestamp = Timestamp::now().unix_timestamp()
        );
        embed_description.push_str(&timestamp_for_post);

        for role in role_count.values() {
            let description_for_post = format!(
                "## {} \n
                Current Members: {}
                \n\n",
                &role.0, &role.1
            );
            embed_description.push_str(&description_for_post)
        }

        let embed = CreateEmbed::new()
            .title("Current Faction Playercounts")
            .description(embed_description);

        return embed;
    }
}
