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
#[description= "The set command is used to toggle features and assign properties on a guild basis. Any member/role with the ADMINISTRATOR permission will be able to utilize this command.\n\
                The following settings and their expected values are below.\n\n\
                ```\n\
                active_welcome:    default: false      [true/false]\n\
                welcome_message:   default: \"Welcome!\" [string]\n\
                assign_new_member: default: false      [true/false]\n\
                new_member_role:   default: 0          [roleId]\n\
                prefix:            default: \"tg!\"      [string]\n\
                staff_id:          default: 0          [roleId]\n\
                ```\n\n\
                For more information please visit [here](https://github.com/TinyPandas/TinyGuardianRS/wiki/Set-Command)"]
#[min_args(2)]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
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
            let upsert = new_setting.eq("staff_id") || new_setting.eq("prefix");
            let res = settings.update_one(query, update, UpdateOptions::builder().upsert(upsert).build()).await.unwrap();
            println!("Modified: {}", res.modified_count);
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