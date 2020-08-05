use serenity::{
    prelude::Context,
    model::channel::{Embed, Message},
    framework::standard::{
        Args, CommandResult, macros::{command}
    }
};

//String API_request = "https://api.swiftype.com/api/v1/public/engines/search.json?callback=jQuery33107513732778347277_1562848671664&q=";
//String API_request_end = "&engine_key=PcoWSkbVqDnWTu_dm2ix&page=1&per_page=5&fetch_fields%5Bpage%5D%5B%5D=title&fetch_fields%5Bpage%5D%5B%5D=body&fetch_fields%5Bpage%5D%5B%5D=category&fetch_fields%5Bpage%5D%5B%5D=url&fetch_fields%5Bpage%5D%5B%5D=segment&fetch_fields%5Bpage%5D%5B%5D=summary&spelling=strict&highlight_fields%5Bpage%5D%5Bbody%5D%5Bfallback%5D=false&_=1562848671665";
//String API_call = API_request + urlSearchTerm + API_request_end; 

const API_BEGIN: &str = "https://api.swiftype.com/api/v1/public/engines/search.json?callback=jQuery33107513732778347277_1562848671664&q=";
const API_END: &str = "&engine_key=PcoWSkbVqDnWTu_dm2ix&page=1&per_page=5&fetch_fields%5Bpage%5D%5B%5D=title&fetch_fields%5Bpage%5D%5B%5D=body&fetch_fields%5Bpage%5D%5B%5D=category&fetch_fields%5Bpage%5D%5B%5D=url&fetch_fields%5Bpage%5D%5B%5D=segment&fetch_fields%5Bpage%5D%5B%5D=summary&spelling=strict&highlight_fields%5Bpage%5D%5Bbody%5D%5Bfallback%5D=false&_=1562848671665";

async fn roblox_wiki(query: &str) -> Option<Embed> {
    let api_call = format!("{}{}{}", API_BEGIN, query, API_END);

    let mut body = reqwest::get(&api_call).await.unwrap().text().await.unwrap();

    //todo split body to 47?
    //todo force body to JSON (removing first and last '()', and invalid '\' chars.)
    //todo split to map?
    //todo parse results
    //todo return as embed

    println!("body = {:?}", body);

    None
}

#[command]
#[min_args(2)]
async fn search(_ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let loc = args.single::<String>().unwrap();
    let query = args.remaining().to_string();

    if loc.eq("wiki") || loc.eq("roblox") {
        let _ = roblox_wiki(&query).await;
    }

    Ok(())
}