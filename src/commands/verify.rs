use serenity::{
    prelude::Context,
    model::{
        channel::{
            Message
        }
    },
    framework::standard::{
        Args, CommandOptions, CommandResult, CheckResult, macros::{command, check}, 
    }
};
use bson::*;
use mongodb::{Collection, Database, options::UpdateOptions};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[check]
#[name = "Guild"]
async fn guild_check(_: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    (msg.guild_id.unwrap() == 165202235226062848).into()
}

pub async fn verify_call(_ctx: &Context, msg: &Message) {
    //let v = match args.single::<String>() {
    //    Ok(c) => {
    //        UserId::from(c.parse::<u64>().unwrap())
    //    }, Err(_why) => {
    //        println!("Errored: {:?}", _why);
    //        msg.author.id
    //    }
    //};

    let author_id = msg.author.id;

    let settings_db: Database = crate::lib::database::get_database("verify_track");
    let settings: Collection = settings_db.collection("verify_track");
    let filter = doc! {"_id" : &author_id.to_string()};
    let user_track = crate::lib::database::get_document_from_collection(settings, filter).await;
    let last_check = crate::lib::database::get_value_for_key(&user_track, String::from("last_check"), String::from("0")).await.parse::<u64>().unwrap();
    let previous = Duration::new(last_check, 0).as_secs();
    let current = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    let mut can_verify = false;

    let check_zero: u64 = 0;
    let check_two_hour: u64 = 7200;

    if last_check.eq(&check_zero) {
        can_verify = true;
    }

    if (current-previous).gt(&check_two_hour) {
        can_verify = true;
    }

    println!("Previous: {}, Current: {}, Can_Verify: {}", previous, current, can_verify);

    if can_verify {
        let settings = settings_db.collection("verify_track");
        let update = settings.update_one(doc! {"_id" : &author_id.to_string()}, doc! {"last_check" : current.to_string()}, UpdateOptions::builder().upsert(true).build()).await.unwrap();
        println!("{:?}", update);
        
        println!("Checking updates for: {}", &msg.author.id.to_string());
        let guild_id = msg.guild_id.unwrap();
        let guild = guild_id.to_partial_guild(&_ctx).await.unwrap();
        let member = guild.member(_ctx, &msg.author.id).await.unwrap();
        let res = crate::lib::shmanager::update_member_roles(_ctx, msg.author.id.to_string().as_str(), guild, member, msg.channel_id).await;

        match res {
            Ok(_) => {
                println!("Success");
            }, Err(_why) => {
                println!("Failed {:?}", _why);
            }
        }
    }
}

#[command]
//#[description="Command used to associate ScriptingHelpers rank to discord account within [Scripting helpers discord](https://discord.gg/WHTAYrK)."]
#[checks(Guild)]
async fn verify(_ctx: &Context, msg: &Message) -> CommandResult {
    verify_call(_ctx, msg).await;

    Ok(())
}