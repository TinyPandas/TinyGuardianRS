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
use crate::lib;
use lib::util::*;

#[command]
#[checks(Staff)]
async fn whois(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut display_current = false;
    let mut display_history = false;
    let mut user_query = String::from("");

    for arg in args.iter::<String>() {
        let arg = arg.unwrap_or(String::from(""));
        if arg.len() > 0 {
            if arg.eq("-a") { display_history = true; }
            else if arg.eq("-c") { display_current = true; }
            else { user_query = arg }
        }
    }

    let guild = msg.guild_id.unwrap();

    let target_id = crate::lib::util::get_user_id_from_query(ctx, guild, &user_query).await;
    let roblox_name = crate::lib::shmanager::get_roblox_name(target_id).await;

    if target_id > 0 {
        let member = guild.member(&ctx.http, target_id).await;

        match member {
            Ok(member_in) => {
                let name_db = crate::lib::database::get_database("name_history");
                let history_for_guild = name_db.collection(guild.to_string().as_str());

                let filter = doc! {"_id" : &member_in.user.id.to_string()};
                let user_history = crate::lib::database::get_document_from_collection(history_for_guild, filter).await;
                let past_usernames = crate::lib::database::get_value_for_key(&user_history, String::from("previous_usernames"), String::from("None")).await;
                let past_nicknames = crate::lib::database::get_value_for_key(&user_history, String::from("previous_nicknames"), String::from("None")).await;

                let _ = msg.channel_id.send_message(ctx, |m |{
                    m.embed(|e|{
                        e.title(format!("Information for {}", user_query));
                        if display_current {
                            e.field("Current Username", format!("{}#{}", &member_in.user.name, &member_in.user.discriminator), false);
                            e.field("Current Nickname", &member_in.display_name(), false);
                        }
                        e.field("Roblox Account", roblox_name, false);
                        if display_history {
                            e.field("Past Usernames", past_usernames, false);
                            e.field("Past Nicknames", past_nicknames, false);
                        }
                        e
                    });
                    m
                }).await;
            }, Err(_why) => {
                let _err_reply = msg.reply(&ctx, "Failed member lookup.").await;
            }
        } 
    } else {
        let _ = msg.reply(&ctx, "No members were found with the provided query.").await;
    }

    Ok(())
}