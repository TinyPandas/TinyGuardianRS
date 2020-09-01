use serenity::{
    prelude::Context,
    model::{channel::{Message}},
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },
    
};

#[command]
async fn whois(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let author_id = &msg.author.id;
    let author_str = &author_id.to_string();
    let query = args.remains().unwrap_or(&author_str.as_str()).to_string();
    
    let target_id = crate::lib::util::get_user_id_from_query(ctx, msg.guild_id.unwrap(), &query).await;
    let roblox_name = crate::lib::shmanager::get_roblox_name(target_id).await;

    if target_id > 0 {
        let _ = msg.channel_id.send_message(ctx, |m |{
            m.embed(|e|{
                e.title(format!("Information for {}", query));
                e.field("Current Username", "username", false);
                e.field("Current Nickname", "nickname", false);
                e.field("Roblox Account", roblox_name, false);
                e.field("Past Usernames", "None", false);
                e.field("Past Nicknames", "None", false);
                e
            });
            m
        }).await;
    } else {
        let _ = msg.reply(&ctx, "No members were found with the provided query.").await;
    }

    Ok(())
}