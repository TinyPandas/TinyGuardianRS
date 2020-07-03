use bson::*;
use mongodb::{Collection, Database};
use serenity::{
    prelude::Context,
    model::channel::{
        Message
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
async fn enable(_ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let selection = args.single::<String>().unwrap();
    let value = args.single::<String>().unwrap();
    let guild_id = &msg.guild_id;
    let settings_db: Database = lib::database::get_database("guild_settings");
    let settings: Collection = settings_db.collection("guild_settings");

    match guild_id {
        Some(id) => {
            let query = doc! {"_id" : &id.to_string()};
            let update = doc! {"$set" : {selection : value}};
            let _ = settings.update_one(query, update, None).await;
            true
        }, None => false,
    };

    Ok(())
}