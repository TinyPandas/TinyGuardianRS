use bson::*;
use std::{sync::Arc};
use mongodb::{Collection, Database};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{Args, CommandOptions, CheckResult, macros::{check}},
    model::{channel::{Message}},
};
use serenity::prelude::*;
use tokio::sync::Mutex;

use crate::lib;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[check]
#[name="Staff"]
async fn staff_check(_: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    println!("Staff check begin");
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

    CheckResult::from(success)
}