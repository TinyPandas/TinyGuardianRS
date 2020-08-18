use std::time::{Duration, Instant};
use serenity::{
    prelude::Context,
    model::{
        channel::{Message},
        id::ChannelId
    },
    framework::standard::{
        Args, CommandResult, macros::{command}
    }
};
use rlua::{Lua, Variadic, prelude::LuaValue, Value::Nil, Table, HookTriggers, Error};
use tokio::task;

async fn execute(ctx: &Context, channel_id: ChannelId, src: String, author: String) {
    let thread_out = task::spawn_blocking(move || {
        let lua = Lua::new();
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

        let result: rlua::Result<(String, String, String)> = lua.context(move |lua_ctx| {
            let globals = lua_ctx.globals();
            let mut res: Vec<String> = vec![];
            let mut err: Vec<String> = vec![];

            let eval = lua_ctx.scope(|scope| {
                let func = scope.create_function_mut(|_, vals: Variadic<LuaValue>| {
                    let mut result: Vec<String> = vec![];
                    for val in vals.iter() {
                        let as_str = match val {
                            LuaValue::Nil => {
                                String::from("nil")
                            },
                            LuaValue::Boolean(b) => {
                                format!("{}", b)
                            },
                            LuaValue::LightUserData(l) => {
                                format!("lightuserdata [{:?}]", l)
                            },
                            LuaValue::Integer(i) => {
                                format!("{}", i)
                            },
                            LuaValue::Number(n) => {
                                format!("{}", n)
                            },
                            LuaValue::String(s) => {
                                let f = s.to_str().unwrap();
                                
                                f.to_string()
                            }
                            LuaValue::Table(t) => {
                                format!("table: [{:?}]", t)
                            },
                            LuaValue::Function(f) => {
                                format!("function: [{:?}]", f)
                            },
                            LuaValue::Thread(th) => {
                                format!("function: [{:?}]", th)
                            }
                            LuaValue::UserData(u) => {
                                format!("userdata [{:?}]", u)
                            }
                            LuaValue::Error(e) => {
                                format!("Error [{:?}]", e)
                            }
                        };
                        result.push(as_str);
                    }

                    res.push(format!("[out]: {}", result.join(" ")));
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

                globals.set("print", func,)?;
                globals.set("error", err_func,)?;

                let c = lua_ctx.load(&src).exec();

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

            if res.len() == 0 {
                res.push(String::from("No output"));
            }

            Ok((res.join("\n"), err.join("\n"), eval))
        });

        
        let r = match result {
            Ok(r) => {
                r
            }, Err(why) => {
                (String::from(""), String::from(""), why.to_string())
            }
        };

        let comb = format!("{}\n{}", r.0, r.1);
        let mut env_out = comb.as_str();
        let mut env_err = r.2.as_str();

        if env_out.len() > 1500 {
            env_out = &env_out[1..1500];
        }
        if env_err.len() > 1500 {
            env_err = &env_err[1..1500];
        }

        (env_out.to_owned(), env_err.to_owned())
    }).await;

    let off_out = match thread_out {
        Ok(r) => { r },
        Err(why) => {
            (String::from(""), why.to_string())
        }
    };

    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title(format!("Code Eval for {}", author));
            e.field("Eval", off_out.0, false);
            e.field("Errors", off_out.1, false);

            e
        });

        m
    }).await;
}

#[command]
#[description="Will attempt to compile into Lua and provide the output per standard compiler."]
#[min_args(1)]
pub async fn lua(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let src = args.remains().unwrap_or("print('NoCode')");
    let _ = execute(ctx, msg.channel_id, src.to_string(), msg.author.name.to_string().to_owned()).await;
    Ok(())
}