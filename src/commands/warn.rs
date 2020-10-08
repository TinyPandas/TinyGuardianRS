use serenity::{
    prelude::Context,
    model::{
        channel::{
            Message, ChannelType, PermissionOverwrite, PermissionOverwriteType
        }, 
        id::{
            UserId, RoleId, ChannelId
        }, 
        permissions::{
            Permissions
        }
    },
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },  
};
use bson::*;
use crate::lib;
use lib::util::*;
use lib::database::*;
use mongodb::options::UpdateOptions;

#[command]
#[checks(Staff)]
#[min_args(2)]
async fn warn(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_query = args.single::<String>();

    match user_query {
        Ok(query) => {
            let user_id_result = get_user_id_from_query(&ctx, msg.guild_id.unwrap(), &query).await;
            let user_id = user_id_result.0;
            let reason = args.remains().unwrap_or("No reason provided.");
            let moderator = msg.author.id;

            let infraction_db = get_database(format!("{}_infractions", msg.guild_id.unwrap().to_string()).as_str());
            let infraction_col = infraction_db.collection(user_id.to_string().as_str());
            let infraction_count = infraction_col.estimated_document_count(None).await.unwrap_or(0) + 1;

            let infraction_record = doc! { "$set" : {"_id" : infraction_count, "type" : "warn", "reason" : reason, "moderator_id" : moderator.to_string(), "removed" : "false"} };
            let _res = infraction_col.update_one(doc! {"_id" : infraction_count}, infraction_record, UpdateOptions::builder().upsert(true).build()).await.unwrap();

            let user_name = UserId::from(user_id).to_user(&ctx.http).await.unwrap().name;
            
            let settings_db = get_database("guild_settings");
            let settings = settings_db.collection("guild_settings");
            let filter = doc! {"_id" : &msg.guild_id.unwrap().to_string()};
            let guild_settings = get_document_from_collection(settings, filter).await;
            let log_channel = get_value_for_key(&guild_settings, String::from("infraction_log_channel"), String::from("0")).await.parse::<u64>().unwrap_or(0);
            let mut create = false;
            let guild_id = msg.guild_id.unwrap();
            let infraction_channel = ChannelId(log_channel).to_channel(&ctx.http).await;
            if log_channel == 0 {
                create = true;
            } else {
                match infraction_channel {
                    Ok(i_c) => {
                        let _f = i_c.id().send_message(&ctx.http, |f|{
                            f.content(format!("Warned {} for {}. [Infractions: {}]", user_name, reason, infraction_count));
                            f
                        }).await;
                    }, Err(_why) => {
                        create = true;
                    }
                }
            }

            if create {
                let staff_id = get_value_for_key(&guild_settings, String::from("staff_id"), String::from("0")).await.parse::<u64>().unwrap_or(0);
                let res = guild_id.create_channel(&ctx.http, |f|{
                    f.name("infraction-log");
                    f.kind(ChannelType::Text);
                    if staff_id != 0 {
                        let staff_permissions = Some(PermissionOverwrite {
                            allow: Permissions::READ_MESSAGES,
                            deny: Permissions::SEND_MESSAGES,
                            kind: PermissionOverwriteType::Role(RoleId(staff_id)),
                        });
                        let everyone_permissions = Some(PermissionOverwrite {
                            allow: Permissions::empty(),
                            deny: Permissions::READ_MESSAGES,
                            kind: PermissionOverwriteType::Role(RoleId(guild_id.as_u64().to_owned())),
                        });

                        f.permissions(staff_permissions);
                        f.permissions(everyone_permissions);
                    }
                    f
                }).await;

                match res {
                    Ok(channel) => {
                        let new_id = channel.id;
                        let settings_update = settings_db.collection("guild_settings");
                        let query = doc! {"_id" : &msg.guild_id.unwrap().to_string()};
                        let update = doc! {"$set" : {"infraction_log_channel" : new_id.to_string()}};
                        let _ = settings_update.update_one(query, update, None).await;

                        let _f = channel.send_message(&ctx.http, |f|{
                            f.content(format!("Warned {} for {}. [Infractions: {}]", user_name, reason, infraction_count));
                            f
                        }).await;
                    }, Err(_why) => {
                        println!("Failed to create infraction channel. {:?}", _why);
                    }
                }
            }
        }, Err(why) => {
            let _ = msg.reply(&ctx.http, format!("Failed to warn user: {}", why.to_string()));
        }
    }

    let _ = msg.delete(&ctx.http).await;
    Ok(())
}