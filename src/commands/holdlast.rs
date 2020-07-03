use bson::*;
use mongodb::{Collection, Database};
use serenity::{
    prelude::Context,
    model::{
        channel::{
            Message, ReactionType
        }
    },
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    }
};

use crate::lib;

#[command]
async fn holdlast(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let message = args.remains().unwrap();
    let guild_id = &msg.guild_id.unwrap();
    let channel_id = &msg.channel_id.to_string();
    let settings_db: Database = lib::database::get_database("guild_settings");
    let settings: Collection = settings_db.collection("guild_settings");
    let delete = message.eq("disable");
    let key = format!("holdlast_{}", channel_id);

    if delete {
        let query = doc! {"_id": &guild_id.to_string(), &key: {"$exists": true}};
        let update = doc! {"$unset" : {key : ""}};
        let res = settings.update_one(query, update, None).await.unwrap();
        if res.modified_count > 0 {
            let _ = msg.react(ctx, ReactionType::Unicode(String::from("\u{2705}"))).await;
        } else {
            let _ = msg.react(ctx, ReactionType::Unicode(String::from("\u{26A0}"))).await;
        }
    } else {
        let query = doc! {"_id": &guild_id.to_string()};
        let update = doc! {"$set" : {key : message}};
        let res = settings.update_one(query, update, None).await.unwrap();
        if res.modified_count > 0 {
            let _ = msg.react(ctx, ReactionType::Unicode(String::from("\u{2705}"))).await;
        } else {
            let _ = msg.react(ctx, ReactionType::Unicode(String::from("\u{26A0}"))).await;
        }
    }

    Ok(())
}