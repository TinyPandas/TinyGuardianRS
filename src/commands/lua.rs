use std::time::{Duration, Instant};
use serenity::{
    prelude::Context,
    model::channel::{Message},
    framework::standard::{
        Args, CommandResult, macros::{command}
    }
};
use rlua::{Lua, Result, Variadic, Value::Nil, Table, HookTriggers, Error};

#[command]
#[min_args(1)]
pub async fn lua(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let lua = Lua::new();
    let src = args.remains().unwrap_or("print('NoCode')");
    
    let start = Instant::now();

    lua.set_hook(HookTriggers {every_line: true, ..Default::default()}, move |_lua_context, _debug| {
        let now = Instant::now();
        let dif = now.duration_since(start);
        println!("{:?}", dif);
        let mut err = false;

        if dif.gt(&Duration::new(30, 0)) {
            err = true
        }

        if err {
            Err(Error::MemoryError(String::from("Instruction time exceeded.")))
        } else {
            Ok(())
        }
    });

    let result: Result<(String, String, String)> = lua.context(move |lua_ctx| {
        let globals = lua_ctx.globals();
        let mut res: Vec<String> = vec![];
        let mut err: Vec<String> = vec![];

        let eval = lua_ctx.scope(|scope| {
            let func = scope.create_function_mut(|_, strings: Variadic<String>| {
                res.push(format!("[out]: {}", strings.join(" ")));
                Ok(())
            })?;

            let err_func = scope.create_function_mut(|_, strings: Variadic<String>| {
                err.push(format!("[err]: {}", strings.join(" ")));
                Ok(())
            })?;

            let os: Table = globals.get("os")?;
            os.set("execute", Nil)?;
            os.set("exit", Nil)?;
            os.set("getenv", Nil)?;
            os.set("remove", Nil)?;
            os.set("rename", Nil)?;
            os.set("setlocale", Nil)?;
            os.set("tmpname", Nil)?;
            globals.set("os", os)?;
            globals.set("io", Nil)?;
            globals.set("debug", Nil)?;
            globals.set("dofile", Nil)?;
            globals.set("load", Nil)?;
            globals.set("collectgarbage", Nil)?;
            globals.set("require", Nil)?;
            globals.set("loadfile", Nil)?;
            globals.set("package", Nil)?;

            globals.set(
                "print",
                func,
            )?;

            globals.set(
                "error",
                err_func,
            )?;

            let c = lua_ctx.load(src)
            .set_name("test")?
            .exec();

            let err_cap = match c {
                Ok(_) => {
                    String::from("No errors.")
                },
                Err(why) => {
                    why.to_string()
                }
            };

            Ok(err_cap)
        })?;

        Ok((res.join("\n"), err.join("\n"), eval))
    });

    
    let r = match result {
        Ok(r) => {
            r
        }, Err(why) => {
            (String::from(""), String::from(""), why.to_string())
        }
    };
    let out = r.0.as_str();
    let user_err = r.1.as_str();
    let comb = format!("{}\n{}", out, user_err);
    let mut env_out = comb.as_str();
    let mut env_err = r.2.as_str();

    let nick = msg.author.id.to_string();
    if env_out.len() > 1500 {
        env_out = &env_out[1..1500];
    }
    if env_err.len() > 1500 {
        env_err = &user_err[1..1500];
    }

    if env_out.len() == 0 {
        env_out = "No output.";
    }

    if env_out.eq("") {
        env_out = "No output.";
    }
    
    if env_err.len() == 0 {
        env_err = "No errors.";
    }

    println!("{} | {}", env_out, env_err);

    let _ = msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title(format!("Code Eval for <@{}>", nick));
            e.field("Eval", env_out, false);
            e.field("Errors", env_err, false);

            e
        });

        m
    }).await;

    Ok(())
}