use bson::*;
use std::{sync::Arc};
use mongodb::{Collection, Database};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{Args, CommandOptions, CheckResult, macros::{check}},
    model::{
        channel::{
            Message
        },
        id::{
            GuildId
        }
    },
    futures::StreamExt
};
use serenity::prelude::*;
use tokio::sync::Mutex;

use crate::lib;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub async fn is_staff(msg: &Message) -> bool {
    let guild_id = &msg.guild_id;
    let settings_db: Database = lib::database::get_database("guild_settings");
    let settings: Collection = settings_db.collection("guild_settings");
    let success = match guild_id {
        Some(id) => {
            let query = doc! {"_id" : &id.to_string()};
            let guild_settings = lib::database::get_document_from_collection(settings, query).await;
            let staff_id = lib::database::get_value_for_key(&guild_settings, String::from("staff_id"), String::from("")).await;

            let member = match &msg.member {
                Some(mem) => {
                    let mut pass = false;
                    for role in &mem.roles {
                        if !pass {
                            pass = role.as_u64().to_string().eq(&staff_id);
                        }
                    }
                    pass
                }, None => false
            };

            member
        }, None => false,
    };

    success
}

#[check]
#[name="Staff"]
async fn staff_check(_: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    CheckResult::from(is_staff(msg).await)
}

pub async fn get_user_id_from_query(ctx: &Context, guild_id: GuildId, query: &String) -> (u64, usize, String) {
    let mut target_id: u64;

    let guild = guild_id.to_guild_cached(&ctx).await.unwrap();

    let members = guild.members_containing(query, false, true).await;

    println!("Members Found: {}", members.len());

    let mut list = vec![];

    for index in &members {
        let user = index.0.user.id;
        let userr = user.to_user(&ctx.http).await.unwrap();
        //list.insert(list.len(), format!("{}#{} ({})", userr.name, userr.discriminator, index.0.display_name().to_string().replace("@", "")));
        list.insert(list.len(), format!("{}#{}", userr.name, userr.discriminator));
    }

    if members.len() > 1 {
        target_id = u64::MAX;
    } else if members.len() == 1 {
        let first = members.first().unwrap().0.user.id;
        target_id = first.as_u64().to_owned();
    } else {
        target_id = 0;
    }

    (target_id, members.len(), list.join(", "))
}

/*
pub async fn get_user_id_from_query(ctx: &Context, guild_id: GuildId, query: &String) -> u64 {
    let possible_mention = query.replace("!", "");

    let mut target_id: u64 = 0;
    let mut ids: Vec<u64> = vec![];
    let mut count = 0;

    match query.parse::<u64>() {
        Ok(user_id) => {
            target_id = user_id;
        },
        Err(_why) => {
            let mut members = guild_id.members_iter(&ctx.http).boxed();
            while let Some(member_result) = members.next().await {
                match member_result {
                    Ok(member) => {
                        count += 1;
                        let display_name: String = member.display_name().into_owned();
                        let username = &member.user.name;
                        let discrim = &member.user.discriminator;
                        let distinct: String = format!("{}#{}", username, discrim);
                        let member_temp = format!("{}", member);
                        let mut flag = false;
                        if display_name.eq(&query.to_string()) {
                            flag = true;
                            break;
                        }
                        if username.eq(&query.to_string()) {
                            flag = true;
                            break;
                        }
                        if distinct.eq(&query.to_string()) {
                            flag = true;
                            break;
                        }
                        if member_temp.eq(&possible_mention.to_string()) {
                            flag = true;
                            break;
                        }

                        if flag {
                            println!("Found match. inserting");
                            ids.insert(ids.len()+1, member.user.id.into());
                        }
                    },
                    Err(error) => eprintln!("Uh oh!  Error: {}", error),
                }
            }
        }
    }

    println!("IDS: {}", ids.len());
    println!("Checked: {}", count);

    if ids.len() > 1 {
        target_id = u64::MAX;
    } else {
        if ids.len() == 1 {
            target_id = match ids.get(0) {
                Some(id) => {
                    id.to_owned()
                }, None => {
                    u64::MIN
                }
            }
        }
    }

    target_id
}
*/