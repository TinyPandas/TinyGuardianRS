use serenity::{
    prelude::Context,
    model::channel::{Message},
    framework::standard::{
        Args, CommandResult, macros::{command}
    }
};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use select::{
    document::Document,
    predicate::{
        Predicate,
        Attr,
        Name
    }
};

const API_BEGIN: &str = "https://api.swiftype.com/api/v1/public/engines/search.json?callback=jQuery33107513732778347277_1562848671664&q=";
const API_END: &str = "&engine_key=PcoWSkbVqDnWTu_dm2ix&page=1&per_page=5&fetch_fields%5Bpage%5D%5B%5D=title&fetch_fields%5Bpage%5D%5B%5D=body&fetch_fields%5Bpage%5D%5B%5D=category&fetch_fields%5Bpage%5D%5B%5D=url&fetch_fields%5Bpage%5D%5B%5D=segment&fetch_fields%5Bpage%5D%5B%5D=summary&spelling=strict&highlight_fields%5Bpage%5D%5Bbody%5D%5Bfallback%5D=false&_=1562848671665";

const LUA_PIL_PREFIX: &str = "https://www.lua.org/manual/5.4/manual.html";

#[derive(Serialize, Deserialize)]
struct RobloxResult {
    record_count: usize,
    records: Value,
    info: Value,
    errors: Value
}

async fn lua_pil(_ctx: &Context, _msg: &Message, query: &str) {
    println!("{}", query);
    let name = format!("pdf-{}", query);
    println!("ATTR: {}", name);

    let document = Document::from(LUA_PIL_PREFIX);
    

    for node in document.find(Name(name.as_str())) {
        println!("{} ({:?})", node.text(), node.attr("href").unwrap());
    }

    for node in document.find(Attr("name", name.as_str()).descendant(Name("h3"))) {
        println!("{} ({:?})", node.text(), node.attr("href").unwrap());
    }

    println!("Completed.");
}

async fn roblox_wiki(ctx: &Context, msg: &Message, query: &str) {
    println!("{}", query);
    let api_call = format!("{}{}{}", API_BEGIN, query, API_END);

    let nick = &msg.author_nick(ctx).await.unwrap_or(String::from("invalid nick"));
    let body = reqwest::get(&api_call).await.unwrap().text().await.unwrap();
    let json_string = (body[45..body.len()-1]).to_string();
    let v: RobloxResult = serde_json::from_str(json_string.as_str()).unwrap();  
    let page = v.records.get("page").unwrap();
    let mut records = 5;
    if records > v.record_count {
        records = v.record_count
    }

    let mut results = vec![];

    for i in 0..records {
        let cur_page = &page[i];
        let url = cur_page.get("url").unwrap().to_string();
        let title = cur_page.get("title").unwrap().to_string();
        let mut desc = cur_page.get("body").unwrap().to_string();        

        if desc.len() > 100 {
            desc = desc[1..100].to_string();
        }

        results.push((format!("[{}]", title.replace("\"", "")), format!("{}... \n[Visit page]({})\n", desc, url.replace("\"", "")), false));
    }

    let _ = msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title(format!("Roblox result for {}", query));
            e.description(format!("Searched by {}", nick));
            e.fields(results);
            e
        });
        m
    }).await;
}

#[command]
#[description="Searches provided engine for query.\n\
               To properly use these, pass the value in `()` to the command. `tg!search roblox <query>`\n\n```\n\
               Current Engines:\n\
               ----------------\n\
               Roblox Wiki (roblox)\n\n```"]
#[min_args(2)]
async fn search(_ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let loc = args.single::<String>().unwrap().to_lowercase();
    let query = args.remains().unwrap();

    if loc.eq("wiki") || loc.eq("roblox") {
        let _ = roblox_wiki(&_ctx, &msg, &query).await;
    } else if loc.eq("lua") || loc.eq("pil") {
        let _ = lua_pil(&_ctx, &msg, &query).await;
    }

    Ok(())
}