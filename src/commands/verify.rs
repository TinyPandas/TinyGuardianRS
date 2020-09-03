use serenity::{
    prelude::Context,
    model::{
        channel::{
            Message
        },
        id::UserId,
    },
    framework::standard::{
        Args, CommandOptions, CommandResult, CheckResult, macros::{command, check}, 
    }
};
use bson::*;
use mongodb::{Collection, Database, options::UpdateOptions};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub async fn verify_call(_ctx: &Context, msg: &Message) {
    //let v = match args.single::<String>() {
    //    Ok(c) => {
    //        UserId::from(c.parse::<u64>().unwrap())
    //    }, Err(_why) => {
    //        println!("Errored: {:?}", _why);
    //        msg.author.id
    //    }
    //};
    if msg.guild_id.unwrap() == 165202235226062848 {
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

        if can_verify {
            let settings = settings_db.collection("verify_track");
            let _update = settings.update_one(doc! {"_id" : &author_id.to_string()}, doc! {"last_check" : current.to_string()}, UpdateOptions::builder().upsert(true).build()).await.unwrap();
            
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

    //Code for home guild support roles.
    //Currently inactive due to cache implementations.
    /*
    631209796971921438 - 10
    631209853863460884 - 75
    631209893650497576 - 200
    631209929012936725 - 1000
    */
    /*
    if msg.guild_id.unwrap() == 546033322401464320 {
        let guilds = _ctx.cache.guilds().await;
        let author = msg.author.id;

        if author.eq(&UserId::from(169208961533345792)) {
            let home = msg.guild_id.unwrap().to_guild_cached(&_ctx).await.unwrap();
            println!("Home has {} members", home.members.len());
            for map in home.members {
                let user_id = map.0;
                let member = map.1;
                let mut found = false;
                for guild in &guilds {
                    let owner = guild.to_partial_guild(&_ctx.http).await.unwrap().owner_id;
                    if owner.eq(&user_id) {
                        found = true;
                        let member_count = guild.to_guild_cached(&_ctx).await.unwrap().member_count;
    
                        println!("{} has {} members", member.display_name(), member_count);
                    }
                }

                if !found {
                    println!("{} has 0 members", member.display_name());
                }
            }
        } else {
            for guild in guilds {
                let owner = guild.to_partial_guild(&_ctx.http).await.unwrap().owner_id;
                if owner.eq(&author) {
                    let member_count = guild.to_guild_cached(&_ctx).await.unwrap().member_count;

                    println!("{} has {} members", msg.author.name, member_count);
                }
            }
        }
    }
    */
}

#[command]
#[description="Command used to associate ScriptingHelpers rank to discord account within [Scripting helpers discord](https://discord.gg/WHTAYrK).\n\
               Command used to associate Support roles within [Panda Guard](https://discord.gg/x4J7zhx)"]
async fn verify(_ctx: &Context, msg: &Message) -> CommandResult {
    verify_call(_ctx, msg).await;

    Ok(())
}