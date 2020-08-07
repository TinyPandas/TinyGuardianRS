use serenity::{
    prelude::Context,
    model::channel::{Message},
    framework::standard::{
        Args, CommandResult, macros::{command}
    }
};
use rlua::{Lua, Result, Variadic};

#[command]
#[min_args(1)]
pub async fn lua(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let lua = Lua::new();
    let src = args.remains().unwrap_or("print('NoCode')");
    
    let mut results: Vec<String> = vec![];

    let _: Result<()> = lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        lua_ctx.scope(|scope| {
            let func = scope.create_function_mut(|_, strings: Variadic<String>| {
                results.push(strings.join(" "));
                Ok(())
            })?;

            globals.set(
                "print",
                func,
            )?;

            lua_ctx.load(src)
            .set_name("test")?
            .exec()?;

            Ok(())
        })?;

        Ok(())
    });

    let nick = msg.author.id.to_string();
    let res_temp = results.join("\n");
    let mut res = res_temp.as_str();
    if res.len() > 1500 {
        res = &res[1..1500];
    }

    let _ = msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title(format!("Code Eval for <@{}>", nick));
            e.field("Eval", res, false);

            e
        });

        m
    }).await;

    Ok(())
}