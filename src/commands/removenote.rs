use serenity::{
    prelude::Context,
    model::{channel::{Message}},
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },  
};
use bson::*;
use crate::lib;
use lib::util::*;

#[command]
#[checks(Staff)]
async fn removenote(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}