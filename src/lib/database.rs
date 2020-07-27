use mongodb::{Client, Collection, Database};

use bson::*;
use once_cell::sync::OnceCell;
use serenity::{
    model::{
        gateway::Ready
    }
};

static MONGO: OnceCell<Client> = OnceCell::new();

fn get_client() -> &'static Client {
    MONGO.get().expect("Client is not setup.")
}

pub async fn db_setup() {
    println!("Setting up DB.");

    let client = match Client::with_uri_str("mongodb://localhost:27017").await {
        Ok(client) => {
            println!("Connected to DB.");
            Some(client)
        }
        Err(why) => {
            println!("Failed to setup DB. {:?}", why);
            None
        }
    };

    if client.is_none() {
        //handle no client connection to db.
        println!("No DB connection");
    }

    MONGO.set(client.unwrap()).unwrap();
}

pub fn get_database(name: &str) -> Database {
    let cl = get_client();

    cl.database(name)
}

pub async fn get_document_from_collection(col: Collection, filter: Document) -> Option<Document> {
    match col.find_one(filter, None).await {
        Ok(opt_doc) => opt_doc,
        Err(_why) => None,
    }
}

pub async fn get_value_for_key(doc: &Option<Document>, field: String, def: String) -> String {
    let value = match &doc {
        Some(_doc) => {
            let res = match _doc.get(field.as_str()).and_then(Bson::as_str) {
                Some(s) => {
                    s.to_string()
                }, None => def
            };
            res
        }, None => def,
    };
    value
}

pub async fn contains_key(doc: &Option<Document>, field: &String) -> bool {
    let value = match &doc {
        Some(_doc) => {
            doc_contains_key(_doc, field.as_str()).await
        }, None => false
    };
    value
}

pub async fn doc_contains_key(doc: &Document, field: &str) -> bool {
    doc.contains_key(field)
}

pub async fn set_default(guild_id: &String, field: &str, val: &str) {
    let settings_db: Database = get_database("guild_settings");
    let settings_update: Collection = settings_db.collection("guild_settings");  
    let query_update = doc! {"_id" : guild_id};
    let update = doc! {"$set" : {field : val}};
    let _ = settings_update.update_one(query_update, update, None).await;
}

pub async fn validate(bot_data: &Ready) { 
    let guilds = &bot_data.guilds;

    for guild in guilds.iter() {
        let guild_id = guild.id().to_string();

        let query = doc! {"_id" : &guild_id};
        let settings_db: Database = get_database("guild_settings");
        let settings: Collection = settings_db.collection("guild_settings");  
        let guild_settings = get_document_from_collection(settings, query).await;
        match guild_settings {
            Some(doc) => {
                //Validate it has default features
                println!("Validate");
                if doc_contains_key(&doc, "active_welcome").await == false { set_default(&guild_id, "active_welcome", "false").await; };
                if doc_contains_key(&doc, "assign_new_member").await == false { set_default(&guild_id, "assign_new_member", "false").await; };
                if doc_contains_key(&doc, "welcome_message").await == false { set_default(&guild_id, "welcome_message", "Welcome to the guild!").await; };
                if doc_contains_key(&doc, "new_member_role").await == false { set_default(&guild_id, "new_member_role", "").await; };
                if doc_contains_key(&doc, "prefix").await == false { set_default(&guild_id, "prefix", "tg!").await; };
                if doc_contains_key(&doc, "staff_id").await == false { set_default(&guild_id, "staff_id", "").await; };
            }, None => {
                //create document
                println!("Create");
                let guild_doc = doc! {"_id" : &guild_id};
                let settings: Collection = settings_db.collection("guild_settings");  
                let _ = settings.insert_one(guild_doc, None).await;
                set_default(&guild_id, "active_welcome", "false").await;
                set_default(&guild_id, "assign_new_member", "false").await;
                set_default(&guild_id, "welcome_message", "Welcome to the guild!").await;
                set_default(&guild_id, "new_member_role", "").await;
                set_default(&guild_id, "prefix", "tg!").await;
                set_default(&guild_id, "staff_id", "").await;
            }
        }
    }
}
