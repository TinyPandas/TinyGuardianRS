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

#[command]
async fn whois(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let author_id = &msg.author.id;
    let author_str = &author_id.to_string();
    let query = args.remains().unwrap_or(&author_str.as_str()).to_string();
    let guild = msg.guild_id.unwrap();

    let target_id = crate::lib::util::get_user_id_from_query(ctx, guild, &query).await;
    let roblox_name = crate::lib::shmanager::get_roblox_name(target_id).await;

    if target_id > 0 {
        let member = guild.member(&ctx.http, target_id).await;

        match member {
            Ok(member_in) => {
                let name_db = crate::lib::database::get_database("name_history");
                let history_for_guild = name_db.collection(guild.to_string().as_str());

                let query = doc! {"_id" : &member_in.user.id.as_str()};
                let user_history = crate::lib::database::get_document_from_collection(history_for_guild, query).await;
                let past_usernames = crate::lib::database::get_value_for_key(&user_history, String::from("previous_usernames"), String::from("None")).await;
                let past_nicknames = crate::lib::database::get_value_for_key(&user_history, String::from("previous_nicknames"), String::from("None")).await;

                let _ = msg.channel_id.send_message(ctx, |m |{
                    m.embed(|e|{
                        e.title(format!("Information for {}", query));
                        e.field("Current Username", format!("{}#{}", &member_in.user.name, &member_in.user.discriminator), false);
                        e.field("Current Nickname", &member_in.display_name(), false);
                        e.field("Roblox Account", roblox_name, false);
                        e.field("Past Usernames", past_usernames, false);
                        e.field("Past Nicknames", past_nicknames, false);
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