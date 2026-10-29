// ----- Serenity Imports -----
use serenity::all::{Context, GuildId, RoleId};

// ----- Rust Imports -----
use std::collections::HashMap;

// --------------------------------------------
// ---------- Discord API Requests ------------
// --------------------------------------------
/// A struct that handles any API requests the bot makes to Discord
pub struct DiscordAPIRequests;

impl DiscordAPIRequests {
    /// Finds all the roles in the guild the bot is in, and returns a hashmap of format {RoleId, (RoleName, NumberOfRoleMembers)}
    pub async fn get_member_counts(
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
