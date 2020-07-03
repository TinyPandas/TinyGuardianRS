use bson::*;
use mongodb::{Collection, Database, options::UpdateOptions};
use serenity::{
    prelude::Context,
    model::channel::{
        Message, ReactionType
    },
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },
};

use crate::lib;

#[command]
#[min_args(2)]
async fn set(_ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let new_setting = args.single::<String>().unwrap();
    let new_value = args.remains().unwrap();
    let guild_id = &msg.guild_id;
    let settings_db: Database = lib::database::get_database("guild_settings");
    let settings: Collection = settings_db.collection("guild_settings");

    let success = match guild_id {
        Some(id) => {
            let query = doc! {"_id" : &id.to_string(), &new_setting: {"$exists": true}};
            let update = doc! {"$set" : {&new_setting : new_value}};
            let res = settings.update_one(query, update, UpdateOptions::builder().upsert(false).build()).await.unwrap();
            res.modified_count > 0
        }, None => false,
    };

    if success {
        let _ = msg.react(_ctx, ReactionType::Unicode(String::from("\u{2705}"))).await;
    } else {
        let _ = msg.react(_ctx, ReactionType::Unicode(String::from("\u{26A0}"))).await;
    }
    Ok(())
}