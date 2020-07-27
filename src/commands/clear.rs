use serenity::{
    prelude::Context,
    model::{
        channel::{
            Message
        }
    },
    framework::standard::{
        Args, CommandResult, macros::{command}, 
    }
};

use crate::lib::util::*;

#[command]
#[min_args(1)]
#[checks(Staff)]
async fn clear(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let delete_count = args.single::<u64>().unwrap();
    let id = if let Ok(id) = args.single::<u64>() { Some(id) } else { None };

    let channel = msg.channel_id;
    let messages = channel.messages(&ctx, |retriever| {
        let r = match &id {
            Some(_id) => {
                retriever.before(msg.id).limit(25)
            }, None => {
                retriever.before(msg.id).limit(delete_count)
            }
        };

        r
    }).await?;

    let mut count = 0;

    for message in messages {
        match &id {
            Some(_id) => {
                if message.author.id == _id.to_owned() {
                    if count < delete_count {
                        let _ = message.delete(ctx).await;
                        count = count + 1;
                    }
                }
            }, None => {
                let _ = message.delete(ctx).await;
            }
        };
    }

    let _ = msg.delete(ctx).await;

    Ok(())
}