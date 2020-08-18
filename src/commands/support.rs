use serenity::{
    prelude::Context,
    model::{
        channel::{
            Message
        }
    },
    framework::standard::{
        Args, CommandOptions, CommandResult, CheckResult, macros::{command, check}
    }
};

#[check]
#[name = "NotGuild"]
async fn not_guild_check(_: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    (msg.guild_id.unwrap() != 546033322401464320).into()
}

#[command]
#[description="Provides invite for TinyGuardian support server."]
#[checks(NotGuild)]
async fn support(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(ctx, |f| {
        f.content("Join the TinyGuardian discord: discord.gg/x4J7zhx");
        f
    }).await;

    Ok(())
}