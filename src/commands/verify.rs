use serenity::{
    prelude::Context,
    model::{
        channel::{
            Message
        }
    },
    framework::standard::{
        CommandResult, macros::{command}, 
    }
};

pub async fn verify_call(_ctx: &Context, msg: &Message) {
    if msg.guild_id.unwrap() == 165202235226062848 {
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
#[description="Command used to associate ScriptingHelpers rank to discord account within [Scripting helpers discord](https://discord.gg/WHTAYrK).\n\
               Command used to associate Support roles within [Panda Guard](https://discord.gg/x4J7zhx)"]
async fn verify(_ctx: &Context, msg: &Message) -> CommandResult {
    verify_call(_ctx, msg).await;

    Ok(())
}