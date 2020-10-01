use serenity::{
    prelude::Context,
    model::channel::Message,
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },
};

#[command]
async fn codeblock(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let result = msg.channel_id.send_message(&ctx.http, |f| {
        if args.len() > 0 {
            f.embed(|m| {
                m.title(format!("Codeblock for {}", msg.author.name));
                let mut diff_lang = false;
                let mut lang = String::from("lua");
                let mut src_str = String::from("");
                for arg in args.iter::<String>() {
                    let arg = arg.unwrap_or(String::from(""));
                    if arg.len() > 0 {
                        if arg.eq("-l") { diff_lang = true; }
                        else if diff_lang { lang = arg; diff_lang = false; }
                        else { src_str = format!("{} {}", src_str, arg) }
                    }
                }

                let src_str_2 = &str::replace(src_str.as_str(), "```lua", "");
                let src_str_3 = &str::replace(src_str_2, "```", "");
                let src_str_4 = &str::replace(src_str_3, "`", "");
                m.field("SRC", format!("```{}\n{}\n```", lang, src_str_4), false);

                m
            });
        } else {
            f.embed(|m| {
                m.title("Codeblock Example");
                m.description("This is how to properly codeblock.");
                m.field("What your message should look like:", "```\n```lua\nfunction display(str)\n\tprint(str)\nend\n\ndisplay(\"Hello World!\")\n`​``\n```", false);
                m.field("What your message will display as:", "```lua\nfunction display(str)\n\tprint(str)\nend\n\ndisplay(\"Hello World!\")\n```", false);

                m
            });
        }

        f
    }).await;

    let _ = msg.delete(&ctx.http).await;

    Ok(())
}