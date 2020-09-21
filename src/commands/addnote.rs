use serenity::{
    prelude::Context,
    model::{channel::{Message}},
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },  
};
use bson::*;
use mongodb::{options::UpdateOptions};
use crate::lib;
use lib::util::*;

#[command]
#[checks(Staff)]
#[min_args(2)]
async fn addnote(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild_id.unwrap();
    let user_query = args.single::<String>().unwrap();
    let note = args.remains().unwrap_or("");

    if note.len() == 0 {
        let _ = msg.reply(&ctx, "Did not add note.").await;
    } else {
        let target_id = get_user_id_from_query(ctx, guild, &user_query).await;
        if target_id > 0 {
            let name_db = crate::lib::database::get_database("name_history");
            let history_for_guild = name_db.collection(guild.to_string().as_str());
            let filter = doc! {"_id" : target_id.to_string()};
            let user_history = crate::lib::database::get_document_from_collection(history_for_guild, filter).await;
            let mut notes = crate::lib::database::get_value_for_key(&user_history, String::from("notes"), String::from("")).await;
            notes = format!("{}\n{}", notes, note);

            let update_col = name_db.collection(guild.to_string().as_str());
            let query = doc! {"_id" : target_id.to_string()};
            let update = doc! {"$set" : {"notes" : notes}};
            let res = update_col.update_one(query, update, UpdateOptions::builder().upsert(true).build()).await.unwrap();

            let _ = msg.channel_id.send_message(&ctx.http, |f|{ 
                f.content("Successfully added note to user.");
                f
            }).await;
        }
    }


    Ok(())
}