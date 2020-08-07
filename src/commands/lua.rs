use serenity::{
    prelude::Context,
    model::channel::{Message},
    framework::standard::{
        Args, CommandResult, macros::{command}
    }
};
use rlua::{Function, Lua, Result, Variadic};
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

static CAPTURES: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(String::from("13"), vec![String::from("")]);
    Mutex::new(m)
});

#[command]
#[min_args(1)]
pub async fn lua(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let lua = Lua::new();
    let src = args.remains().unwrap_or("print('NoCode')");
    let author_id = msg.author.id.to_string();
    let env_name = format!("exec_for_{}", author_id);

    let _: Result<()> = lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        let _old_print = globals.get::<_, Function>("print").unwrap();

        let new_print = lua_ctx.create_function(|_, strings: Variadic<String>| {
            let mut cap = CAPTURES.lock().unwrap();
            let mut vec = cap.get(&String::from("13")).unwrap().clone();
            vec.insert(vec.len(), strings.join(" "));
            cap.insert(String::from("13"), vec);
            Ok(())
        })?;
        globals.set("print", new_print)?;

        lua_ctx.load(src)
        .set_name(&env_name)?
        .exec()?;

        Ok(())
    });

    let _ = msg.channel_id.send_message(ctx, |m| {
        let cap = CAPTURES.lock().unwrap();
        let vec = cap.get(&String::from("13")).unwrap();
        let result = vec.join(" ");
        m.content(result);

        m
    }).await;

    let mut cap = CAPTURES.lock().unwrap();
    cap.insert(String::from("13"), vec![]);

    Ok(())
}