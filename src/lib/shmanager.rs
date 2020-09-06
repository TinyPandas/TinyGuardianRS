use serenity::{
    prelude::Context,
    model::{
        guild::{PartialGuild, Member},
        id::ChannelId,
    },
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RobloxAccount {
    status: String,
    robloxUsername: String,
    robloxId: i32,
}

#[derive(Serialize, Deserialize)]
struct Permissions {
    ADMIN: bool,
    SUPER_ADMIN: bool,
    BLOG_EDITOR: bool,
    BLOG_AUTHOR: bool,
    MODERATE: bool,
    EDIT_GLOSSARY: bool,
    COMMUNITY_MODERATOR: bool,
    CHAT_ADMIN: bool,
}

//"stats":{"questions":22,"answers":18,"accepted_answers":9,"upvotes_received":7}}
#[derive(Serialize, Deserialize)]
struct Stats {
    questions: i64,
    answers: i64,
    accepted_answers: i64,
    upvotes_received: i64
}

#[derive(Serialize, Deserialize)]
struct SHAccount {
    id: i32,
    roblox_username: String,
    roblox_userid: i64,
    roblox_age: i64,
    join_date: i64,
    reputation: i32,
    bio: String,
    is_suspended: bool,
    last_seen_date: i64,
    rank: String,
    is_donator: bool,
    permissions: Permissions,
    stats: Stats
}

#[derive(Serialize, Deserialize)]
struct Rank {
    reputation: i64,
    questions_asked: i64,
    questions_answered: i64,
    accepted_answers: i64,
    upvotes_received: i64
}

pub async fn get_roblox_name(discord_id: u64) -> String {
    let roblox_acc = get_roblox_account(discord_id.to_string().as_str()).await;

    let res = match roblox_acc {
        Some(r) => {
            format!("{} [{}]", r.robloxUsername, r.robloxId)
        }, None => {
            String::from("No associated roblox account.")
        }
    };

    res
}

async fn get_roblox_account(discord_id: &str) -> Option<RobloxAccount> {
    let res = match reqwest::get(format!("https://verify.eryn.io/api/user/{}", discord_id).as_str()).await {
        Ok(r) => {
            match r.json::<RobloxAccount>().await {
                Ok(r) => {
                    Some(r)
                }, Err(_why) => {
                    None
                }
            }
        }, Err(_why) => {
            None
        }
    };

    res
}

async fn get_sh_account(roblox_acc: RobloxAccount) -> Option<SHAccount> {
    let res = match reqwest::get(format!("https://scriptinghelpers.org/resources/get_profile_by_roblox_id/{}", roblox_acc.robloxId).as_str()).await {
        Ok(r) => {
            match r.json::<SHAccount>().await {
                Ok(s) => {
                    Some(s)
                }, Err(_why) => {
                    None
                }
            }
        }, Err(_why) => {
            None
        }
    };

    res
}

pub async fn update_member_roles(_ctx: &Context, discord_id: &str, guild: PartialGuild, mut member: Member, channel_id: ChannelId) -> Result<(), Box<dyn std::error::Error>> {
    let beginner = Rank{ reputation: 0, questions_asked: 0, questions_answered: 0, accepted_answers: 0, upvotes_received: 0 };
    let asker = Rank{ reputation: 5, questions_asked: 1, questions_answered: 0, accepted_answers: 0, upvotes_received: 0 };
    let inquisitor = Rank{ reputation: 15, questions_asked: 5, questions_answered: 0, accepted_answers: 0, upvotes_received: 0 };
    let contributor = Rank{ reputation: 25, questions_asked: 0, questions_answered: 5, accepted_answers: 0, upvotes_received: 0 };
    let researcher = Rank{ reputation: 45, questions_asked: 0, questions_answered: 10, accepted_answers: 0, upvotes_received: 0 };
    let academic = Rank{ reputation: 50, questions_asked: 0, questions_answered: 15, accepted_answers: 1, upvotes_received: 0 };
    let educator = Rank{ reputation: 75, questions_asked: 0, questions_answered: 25, accepted_answers: 2, upvotes_received: 10 };
    let professor = Rank{ reputation: 125, questions_asked: 0, questions_answered: 35, accepted_answers: 3, upvotes_received: 20 };
    let intellectual = Rank{ reputation: 500, questions_asked: 0, questions_answered: 50, accepted_answers: 7, upvotes_received: 35 };
    let scholar = Rank{ reputation: 1000, questions_asked: 0, questions_answered: 100, accepted_answers: 15, upvotes_received: 50 };
    let expert = Rank{ reputation: 2000, questions_asked: 0, questions_answered: 150, accepted_answers: 35, upvotes_received: 100 };
    let master = Rank{ reputation: 5000, questions_asked: 0, questions_answered: 300, accepted_answers: 50, upvotes_received: 200 };

    let rank_vec = vec![beginner, asker, inquisitor, contributor, researcher, academic, educator, professor, intellectual, scholar, expert, master];

    println!("Fetching data for {}", &discord_id);
    let resp = get_roblox_account(&discord_id).await;
    match resp {
        Some(roblox_acc) => {
            println!("Roblox account");
            let data = get_sh_account(roblox_acc).await;
    
            match data {
                Some(sh_acc) => {
                    println!("SH account");
                    let sh_roles = vec!["Beginner", "Asker", "Inquisitor", "Contributor", "Researcher", "Academic", "Educator", "Professor", "Intellectual", "Scholar", "Expert", "Master"];
                    let mut current = false;
                    let next_role_index = &sh_roles.iter().position(|&x| x.eq(sh_acc.rank.as_str())).unwrap_or(0)+1;
                    let next_role_str = &sh_roles.get(next_role_index.to_owned()).unwrap_or(&"Beginner").to_owned();
                    let next_role = rank_vec.get(next_role_index.to_owned()).unwrap();
        
                    println!("User {} is rank {} with {} rep", sh_acc.roblox_username, sh_acc.rank, sh_acc.reputation);
                    for role in sh_roles {
                        if !role.eq(sh_acc.rank.as_str()) {
                            match guild.role_by_name(role) {
                                Some(g_role) => {
                                    if member.roles.contains(&g_role.id) {
                                        let _ = member.remove_role(&_ctx, g_role.id).await;
                                    }
                                }, None => {}
                            }
                        } else {
                            match guild.role_by_name(sh_acc.rank.as_str()) {
                                Some(a_role) => {
                                    if member.roles.contains(&a_role.id) {
                                        current = true;
                                    } else {
                                        let _ = member.add_role(&_ctx, a_role.id).await;
                                    }
                                    
                                    ()
                                }, None => ()
                            }
                        }
                    }
        
                    let _ = channel_id.send_message(&_ctx, |m| {
                        if current {
                            let progress = format!("Reputation: {}/{}\n\
                            Questions Asked: {}/{}\n\
                            Questions Answerd: {}/{}\n\
                            Accepted Answers: {}/{}\n\
                            Upvotes Received: {}/{}",
                            &sh_acc.reputation, &next_role.reputation,
                            &sh_acc.stats.questions, &next_role.questions_asked,
                            &sh_acc.stats.answers, &next_role.questions_answered,
                            &sh_acc.stats.accepted_answers, &next_role.accepted_answers,
                            &sh_acc.stats.upvotes_received, &next_role.upvotes_received);

                            println!("{}", progress);
                            m.embed(|e| {
                                e.title("Rank Information");
                                e.field("Current Rank", &sh_acc.rank, false);
                                e.field("Next Rank", next_role_str, false);
                                e.field("Progress to next rank", progress, false);

                                e
                            });
                        } else {
                            m.content(format!("Updated roles! New Role: {} [Rep: {}]", sh_acc.rank, sh_acc.reputation));
                        }
                        m
                    }).await;
        
                    println!("User {} has Moderation Voter role", sh_acc.roblox_username);
                    if sh_acc.permissions.MODERATE {
                        match guild.role_by_name("Moderation Voter") {
                            Some(role) => {
                                if !member.roles.contains(&role.id) {
                                    let _ = member.add_role(&_ctx, role.id).await;
                                    ()
                                }
                            }, None => ()
                        }
                    }
                }, None => {
        
                }
            }
        }, None => {

        }
    }
    

    println!("Completed.");
    Ok(())
}