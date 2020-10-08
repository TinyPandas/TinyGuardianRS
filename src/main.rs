use std::{collections::{HashSet}, env, sync::Arc};
use serenity::{
    async_trait,
    framework::standard::{
        Args, CommandResult, CommandGroup,
        HelpOptions, help_commands, StandardFramework,
        macros::{group, help, hook},
    },
    http::Http,
    model::{channel::{Message}, gateway::{Ready, Activity}, id::UserId},
    prelude::*,
};



mod lib;

mod commands;
use commands::{ping::*, lua::*, verify::*, support::*, clear::*, search::*, codeblock::*};

const MAJOR: i64 = 1;
const MINOR: i64 = 1;
const PATCH: i64 = 21;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let _ = ctx.set_activity(Activity::playing(format!("{}.{}.{}", MAJOR, MINOR, PATCH).as_str())).await;
    }
}

#[group]
#[only_in(guilds)]
#[commands(clear)]
struct Staff;

#[group]
#[commands(ping, verify, support, search, lua, codeblock)]
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
async fn dynamic_prefix(_ctx: &Context, _msg: &Message) -> Option<String> {
    Some(String::from("tg!"))
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
async fn normal_message(ctx: &Context, msg: &Message) {
    if msg.author.bot { return; }

    let channel_id = msg.channel_id;
    let showcase_id: u64 = 689686353021370376;

    let ignore = lib::util::is_staff(ctx, msg).await;

    if !ignore {
        if channel_id.eq(&showcase_id) {
            if msg.attachments.len() < 1 {
                let _ = msg.delete(&ctx.http).await;
            }
        }
    }

    if msg.content.starts_with("!verify") {
        crate::commands::verify::verify_call(&ctx, &msg).await;
    }
}

#[tokio::main]
async fn main() {
    kankyo::load().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment.");

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