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
#[description="`tg!clear <count> [userId]`\n\
                Will attempt to clear `count` message. If `userId` is provided, will only remove that members messages."]
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
    
    match &id {
        Some(_id) => {
            let mut msg_by_auth = vec![];
            for message in messages {
                if message.author.id == _id.to_owned() {
                    if count < delete_count {
                        //let _ = message.delete(ctx).await;
                        msg_by_auth.insert(msg_by_auth.len()+1, message.id);
                        count = count + 1;
                    }
                }
            }
            let _ = msg.channel_id.delete_messages(&ctx.http, msg_by_auth).await;
        }, None => {
            let _ = msg.channel_id.delete_messages(&ctx.http, messages).await;
        }
    }

    let _ = msg.delete(ctx).await;

    Ok(())
}