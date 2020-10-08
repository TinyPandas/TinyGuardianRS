use serenity::{
    prelude::Context,
    model::{channel::{Message}},
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },  
};

use crate::lib;
use lib::util::*;

#[command]
#[checks(Staff)]
async fn removenote(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
    Ok(())
}