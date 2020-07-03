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

#[check]
#[name = "Guild"]
async fn guild_check(_: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    (msg.guild_id.unwrap() == 546033322401464320).into()
}

#[command]
#[checks(Guild)]
async fn verify(_ctx: &Context, msg: &Message) -> CommandResult {
    //let v = match args.single::<String>() {
    //    Ok(c) => {
    //        UserId::from(c.parse::<u64>().unwrap())
    //    }, Err(_why) => {
    //        println!("Errored: {:?}", _why);
    //        msg.author.id
    //    }
    //};

    println!("Checking updates for: {}", &msg.author.id.to_string());
    let guild_id = msg.guild_id.unwrap();
    let guild = guild_id.to_partial_guild(&_ctx).await.unwrap();
    let member = guild.member(_ctx, &msg.author.id).await.unwrap();
    let res = crate::lib::shmanager::update_member_roles(_ctx, msg.author.id.to_string().as_str(), guild, member).await;

    match res {
        Ok(_) => {
            println!("Success");
        }, Err(_why) => {
            println!("Failed {:?}", _why);
        }
    }

    Ok(())
}