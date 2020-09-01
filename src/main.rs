use bson::*;
use mongodb::{Collection, Database};
use std::{collections::{HashSet}, env, sync::Arc};
use serenity::{
    async_trait,
    framework::standard::{
        Args, CommandResult, CommandGroup,
        HelpOptions, help_commands, StandardFramework,
        macros::{group, help, hook},
    },
    http::Http,
    model::{channel::{Message}, gateway::{Ready, Activity}, id::{UserId, GuildId, RoleId, MessageId}, guild::Member},
    prelude::*,
};



mod lib;

mod commands;
use commands::{set::*, ping::*, holdlast::*, lua::*, verify::*, support::*, clear::*, search::*, whois::*};

struct Handler;

const MAJOR: i64 = 1;
const MINOR: i64 = 1;
const PATCH: i64 = 1;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let _ = ctx.set_activity(Activity::playing(format!("{}.{}.{}", MAJOR, MINOR, PATCH).as_str())).await;

        //lib::database::validate(&ready).await;
        //println!("Validated DB");
    }

    async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, mut new_member: Member) {
        let _guild_name = match guild_id.name(&ctx).await {
            Some(p) => p,
            None => guild_id.to_string(),
        };

        let settings_db: Database = lib::database::get_database("guild_settings");
        let settings: Collection = settings_db.collection("guild_settings");
        let filter = doc! {"_id" : &guild_id.to_string()};
        let guild_settings = lib::database::get_document_from_collection(settings, filter).await;
        let welcome_active = lib::database::get_value_for_key(&guild_settings, String::from("active_welcome"), String::from("false")).await.eq("true");
        let member_active = lib::database::get_value_for_key(&guild_settings, String::from("assign_new_member"), String::from("false")).await.eq("true");
        let welcome_msg = lib::database::get_value_for_key(&guild_settings, String::from("welcome_message"), String::from("Welcome to the guild!")).await;
        let member_role_id = lib::database::get_value_for_key(&guild_settings, String::from("new_member_role"), String::from("")).await;

        if welcome_active {
            let dm = new_member.user.dm(&ctx, |m|{
                m.content(welcome_msg);
                m
            }).await;
    
            if let Err(why) = dm {
                println!("Error when direct messaging user: {:?}", why);
            }
        }

        if member_active {
            if member_role_id.len() > 0 {
                let res = new_member.add_role(&ctx, member_role_id.parse::<RoleId>().unwrap()).await;
                if res.is_err() {
                    println!("{:?}", res.err());
                }
            }
        }
    }
}

#[group]
#[owners_only]
#[only_in(guilds)]
#[commands(set)]
struct Owner;

#[group]
#[only_in(guilds)]
#[commands(clear, holdlast, whois)]
struct Staff;

#[group]
#[commands(ping, verify, support, search, lua)]
struct General;

#[help]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn my_help(context: &Context, msg: &Message, args: Args, help_options: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn dynamic_prefix(_ctx: &Context, msg: &Message) -> Option<String> {
    let guild_id = &msg.guild_id.unwrap();
    let filter = doc! {"_id" : &guild_id.to_string()};
    let settings_db: Database = lib::database::get_database("guild_settings");
    let settings: Collection = settings_db.collection("guild_settings");
    let guild_settings = lib::database::get_document_from_collection(settings, filter).await;
    let prefix = lib::database::get_value_for_key(&guild_settings, String::from("prefix"), String::from("tg!")).await;
    
    Some(prefix)
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
async fn normal_message(ctx: &Context, msg: &Message) {
    if msg.author.bot { return; } 
    let guild_id = &msg.guild_id.unwrap();
    let filter = doc! {"_id" : &guild_id.to_string()};
    let settings_db: Database = lib::database::get_database("guild_settings");
    let settings: Collection = settings_db.collection("guild_settings");
    let guild_settings = lib::database::get_document_from_collection(settings, filter).await;
    let channel_id = &msg.channel_id;
    let key = format!("holdlast_{}", channel_id.to_string());

    if lib::database::contains_key(&guild_settings, &key).await {
        let prev = crate::lib::redisdb::get_value(&channel_id.to_string().as_str()).await;
        if prev.len() > 0 {
            let m_id: u64 = prev.parse::<u64>().unwrap();
            let _ = channel_id.delete_message(&ctx, MessageId::from(m_id)).await;
        }

        let message = lib::database::get_value_for_key(&guild_settings, key, String::from("")).await;
        if message.len() > 0 {
            let new_message = channel_id.send_message(&ctx, |f| {
                f.content(message);
                f
            }).await.unwrap();
            let _ = crate::lib::redisdb::set_value(&channel_id.to_string().as_str(), &new_message.id.to_string().as_str()).await;
        }
    }

    if msg.content.contains("!verify") {
        crate::commands::verify::verify_call(&ctx, &msg).await;
    }
}

#[tokio::main]
async fn main() {
    kankyo::load().expect("Failed to load .env file");

    lib::database::db_setup().await;

    let token = env::var("DISCORD_TOKEN_TEST").expect("Expected a token in the environment.");

    let http = Http::new_with_token(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .dynamic_prefix(dynamic_prefix)
            .owners(owners))
        .unrecognised_command(unknown_command)
        .normal_message(normal_message)
        .help(&MY_HELP)
        .group(&OWNER_GROUP)
        .group(&STAFF_GROUP)
        .group(&GENERAL_GROUP);

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<lib::util::ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}