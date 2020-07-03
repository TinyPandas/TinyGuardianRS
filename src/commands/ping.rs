use serenity::{
    client::bridge::gateway::{ShardId},
    prelude::Context,
    model::channel::{
        Message
    },
    framework::standard::{
        CommandResult,
        macros::command,
    },
};

use crate::lib;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
    let data = ctx.data.read().await;

    let shard_manager = match data.get::<lib::util::ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = msg.reply(ctx, "There was a problem getting the shard manager").await;

            return Ok(());
        },
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = msg.reply(ctx,  "No shard found").await;

            return Ok(());
        },
    };

    let _ = msg.reply(ctx, &format!("The shard latency is {:?}", runner.latency)).await;

    Ok(())
}