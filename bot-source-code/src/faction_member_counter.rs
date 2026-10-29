// ----- Serenity Imports -----
use serenity::all::{
    ChannelId, Context, CreateMessage, EditMessage, GetMessages, GuildId, Ready, RoleId, Timestamp,
};
use serenity::builder::CreateEmbed;

// ----- Rust Imports -----
use std::collections::HashMap;

// --------------------------------------------
// ---------- Faction Member Counter ----------
// --------------------------------------------
/// A struct that handles all the faction counter logic and code
pub struct FactionMemberCounter;

impl FactionMemberCounter {
    /// Updated the faction counter in the faction counter channel. Will edit the counter message if it exists, if not, it will post a new message.
    pub async fn update_faction_counter(
        ctx: Context,
        ready: Ready,
        faction_count_channel_id: ChannelId,
        roles_to_keep_track_of: &[&RoleId],
    ) {
        let bot_id = ctx.cache.current_user().id;
        let guild_id = ready.guilds[0].id;

        let most_recent_message_in_faction_counter_channel = faction_count_channel_id
            .messages(&ctx.http, GetMessages::new().limit(1))
            .await
            .unwrap();

        if most_recent_message_in_faction_counter_channel.len() > 0
            && most_recent_message_in_faction_counter_channel[0].author.id == bot_id
        {
            // Edits the message embed if it exists
            let message_id = most_recent_message_in_faction_counter_channel[0].id;
            if let Err(error) = faction_count_channel_id
                .edit_message(
                    &ctx.http,
                    message_id,
                    EditMessage::new().embed(
                        Self::create_faction_counter_embed(&ctx, guild_id, roles_to_keep_track_of)
                            .await,
                    ),
                )
                .await
            {
                eprintln!("Failed to edit faction counter message: {error}");
            }
        } else {
            // Posts a new message with an embed if it dosen't already exist
            if let Err(error) = faction_count_channel_id
                .send_message(
                    &ctx.http,
                    CreateMessage::new().embed(
                        Self::create_faction_counter_embed(&ctx, guild_id, roles_to_keep_track_of)
                            .await,
                    ),
                )
                .await
            {
                eprintln!("Failed to send new faction counter message: {error}");
            }
        }
    }

    /// Gatheres the data for, and then creates the faction member counter
    async fn create_faction_counter_embed(
        ctx: &Context,
        guild_id: GuildId,
        roles_to_track: &[&RoleId],
    ) -> CreateEmbed {
        let role_count = Self::get_member_counts(&ctx, guild_id, roles_to_track).await;

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

    /// Finds all the roles in the guild the bot is in, and returns a hashmap of format {RoleId, (RoleName, NumberOfRoleMembers)}
    async fn get_member_counts(
        ctx: &Context,
        guild_id: GuildId,
        roles_to_track: &[&RoleId],
    ) -> HashMap<RoleId, (String, usize)> {
        let mut role_counts: HashMap<RoleId, (String, usize)> = HashMap::new();

        // Fetch guild information.
        let guild = match guild_id.to_partial_guild(&ctx.http).await {
            Ok(guild) => guild,
            Err(error) => {
                eprintln!("Failed to fetch guild: {error}");
                return role_counts;
            }
        };

        // Fetch members from Discord.
        let members = match guild_id.members(&ctx.http, None, None).await {
            Ok(members) => members,
            Err(error) => {
                eprintln!("Failed to fetch members: {error}");
                return role_counts;
            }
        };

        // Initialize every role with a count of zero.
        for role in guild.roles {
            if roles_to_track.contains(&&role.0) {
                role_counts.insert(role.0, (role.1.name, 0));
            }
        }

        // Count assigned roles.
        for member in &members {
            for role_id in member.roles.clone() {
                if roles_to_track.contains(&&role_id) {
                    let values_of_entry = role_counts.get(&role_id);
                    let role_name = values_of_entry.unwrap().0.clone();
                    let role_count = values_of_entry.unwrap().1.clone();
                    role_counts.insert(role_id, (role_name, role_count + 1));
                }
            }
        }

        return role_counts;
    }
}
