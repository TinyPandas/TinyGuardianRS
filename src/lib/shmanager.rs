use serenity::{
    prelude::Context,
    model::{
        guild::{PartialGuild, Member},
        id::ChannelId,
    },
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RobloxAccount {
    status: String,
    robloxUsername: String,
    robloxId: i32,
}

#[derive(Serialize, Deserialize)]
struct Permissions {
    ADMIN: bool,
    SUPER_ADMIN: bool,
    BLOG_EDITOR: bool,
    BLOG_AUTHOR: bool,
    MODERATE: bool,
    EDIT_GLOSSARY: bool,
    COMMUNITY_MODERATOR: bool,
    CHAT_ADMIN: bool,
}

#[derive(Serialize, Deserialize)]
struct SHAccount {
    id: i32,
    roblox_username: String,
    roblox_userid: i64,
    roblox_age: i64,
    join_date: i64,
    reputation: i32,
    bio: String,
    is_suspended: bool,
    last_seen_date: i64,
    rank: String,
    is_donator: bool,
    permissions: Permissions,
}

pub async fn update_member_roles(_ctx: &Context, discord_id: &str, guild: PartialGuild, mut member: Member, channel_id: ChannelId) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching data for {}", &discord_id);
    let resp: RobloxAccount = reqwest::get(format!("https://verify.eryn.io/api/user/{}", discord_id).as_str()).await?.json::<RobloxAccount>().await?;
    let data: SHAccount = reqwest::get(format!("https://scriptinghelpers.org/resources/get_profile_by_roblox_id/{}", resp.robloxId).as_str()).await?.json::<SHAccount>().await?;
    
    let sh_roles = vec!["Beginner", "Asker", "Inquisitor", "Contributor", "Researcher", "Academic", "Educator", "Professor", "Intellectual", "Scholar", "Expert", "Master"];
    let mut current = false;

    println!("User {} is rank {} with {} rep", data.roblox_username, data.rank, data.reputation);
    for role in sh_roles {
        if !role.eq(data.rank.as_str()) {
            match guild.role_by_name(role) {
                Some(g_role) => {
                    if member.roles.contains(&g_role.id) {
                        let _ = member.remove_role(&_ctx, g_role.id).await;
                    }
                }, None => {}
            }
        } else {
            match guild.role_by_name(data.rank.as_str()) {
                Some(a_role) => {
                    if member.roles.contains(&a_role.id) {
                        current = true;
                    } else {
                        let _ = member.add_role(&_ctx, a_role.id).await;
                    }
                    
                    ()
                }, None => ()
            }
        }
    }

    let _ = channel_id.send_message(&_ctx, |m| {
        if current {
            m.content(format!("Roles are current! Role: {} [Rep: {}]", data.rank, data.reputation));
        } else {
            m.content(format!("Updated roles! New Role: {} [Rep: {}]", data.rank, data.reputation));
        }
        m
    }).await;

    println!("User {} has Moderation Voter role", data.roblox_username);
    if data.permissions.MODERATE {
        match guild.role_by_name("Moderation Voter") {
            Some(role) => {
                if !member.roles.contains(&role.id) {
                    let _ = member.add_role(&_ctx, role.id).await;
                    ()
                }
            }, None => ()
        }
    }

    println!("Completed.");
    Ok(())
}