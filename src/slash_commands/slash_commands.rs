use crate::Context;
use serenity::model::interactions::{Interaction, application_command::*, InteractionResponseType};
use serde_json::json;
use tracing::info;
use crate::geoguessr;

pub async fn handle_interaction(ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
        let mut response = "ok".to_string();
        let content = match command.data.name.as_str() {
            "geo" => {
                match command.data.options[0].options[0].name.as_str() {
                    "normal" => {
                        let mut map = json!("None");
                        let mut rules = json!("None");
                        let mut time = json!("None");
                        for a in &command.data.options[0].options[0].options {
                            match a.name.as_str() {
                                "map" => {
                                    map = match a.value.clone() {
                                        Some(v) => v,
                                        None => {
                                            response = format!("Unable to process request! Empty value on {} was given", a.name).clone();
                                            json!("None")
                                        }
                                    }
                                }
                                "rules" => {
                                    rules = match a.value.clone() {
                                        Some(v) => v,
                                        None => {
                                            response = format!("Unable to process request! Empty value on {} was given", a.name).clone();
                                            json!("None")
                                        }
                                    }
                                }
                                "time" => {
                                    time = match a.value.clone() {
                                        Some(v) => v,
                                        None => {
                                            json!("None")
                                        }
                                    }
                                }
                                _ => continue
                            }
                        }
                        info!("Calling get_map with following arguments: map={}, rules={}, time={}", map, &rules, &time);
                        response = format!("Processing request for `Normal` with arguments: `map={}`, `rules={}`, `time={}`", map, rules, time);
                        let ctx_2 = ctx.clone();
                        tokio::spawn(async move {
                            let url = geoguessr::get_map(&ctx_2, map.as_str().unwrap(), rules.as_str().unwrap(), time.as_i64()).await;
                            command.channel_id.say(&ctx_2.http, url).await.unwrap();
                        });
                    },
                    "cs" => {
                        let mut rules = json!("None");
                        let mut time = json!("None");
                        for a in &command.data.options[0].options[0].options {
                            match a.name.as_str() {
                                "rules" => {
                                    rules = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("None"),
                                    }
                                }
                                "time" => {
                                    time = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("None"),
                                    }
                                }
                                _ => continue
                            }
                        }
                        info!("Calling get_cs with following arguments: rules={}, time={}", &rules, &time);
                        response = format!("Processing request for `Country-Streak` with arguments: `rules={}`, `time={}`", rules, time);
                        let ctx_2 = ctx.clone();
                        tokio::spawn(async move {
                            let url = geoguessr::get_cs(&ctx_2, rules.as_str().unwrap(), time.as_i64()).await;
                            command.channel_id.say(&ctx_2.http, url).await.unwrap();
                        });
                    },
                    "brc" => {
                        let mut lobby = json!("None");
                        let mut rules = json!("None");
                        let mut powerups = json!("None");
                        let mut create = true;
                        for a in &command.data.options[0].options[0].options {
                            match a.name.as_str() {
                                "lobby" => {
                                    lobby = match a.value.clone() {
                                        Some(v) => {
                                            create = false;
                                            v
                                        },
                                        None => json!("None"),
                                    }
                                }
                                "rules" => {
                                    rules = match a.value.clone() {
                                        Some(v) => v,
                                        None => {
                                            json!("None")
                                        }
                                    }
                                }
                                "powerups" => {
                                    powerups = match a.value.clone() {
                                        Some(v) => v,
                                        None => {
                                            json!("None")
                                        }
                                    }
                                },
                                _ => continue
                            }
                        }
                        if create {
                            info!("Calling create_brc");
                            response = "Processing request for `Create Battle-Royale Countries`".to_string();
                            let ctx_2 = ctx.clone();
                            tokio::spawn(async move {
                                let url = geoguessr::create_brc(&ctx_2).await;
                                command.channel_id.say(&ctx_2.http, url).await.unwrap();
                            });
                        }
                        if lobby != json!("None") {
                            info!("Calling start_brc with following arguments: lobby={}, rules={}, powerups={}", lobby, rules, powerups);
                            response = "Starting match!".to_string();
                            let ctx_2 = ctx.clone();
                            tokio::spawn(async move {
                                geoguessr::start_brc(&ctx_2, lobby.as_str().unwrap(), rules.as_str().unwrap(), powerups.as_str().unwrap()).await;
                            });
                        }
                    },
                    "brd" => {
                        let mut lobby = json!("None");
                        let mut rules = json!("None");
                        let mut create = true;
                        for a in &command.data.options[0].options[0].options {
                            match a.name.as_str() {
                                "lobby" => {
                                    lobby = match a.value.clone() {
                                        Some(v) => {
                                            create = false;
                                            v
                                        },
                                        None => json!("None"),
                                    }
                                }
                                "rules" => {
                                    rules = match a.value.clone() {
                                        Some(v) => v,
                                        None => {
                                            json!("None")
                                        }
                                    }
                                },
                                _ => continue
                            }
                        }
                        if create {
                            info!("Calling create_brd");
                            response = "Processing request for `Create Battle-Royale Distance`".to_string();
                            let ctx_2 = ctx.clone();
                            tokio::spawn(async move {
                                let url = geoguessr::create_brd(&ctx_2).await;
                                command.channel_id.say(&ctx_2.http, url).await.unwrap();
                            });
                        }
                        if lobby != json!("None") {
                            info!("Calling start_brd with following arguments: lobby={}, rules={}", lobby, rules);
                        }
                    },
                    _ => {
                        response = "Unknown command! Skipping...".to_string()
                    }
                }
                if response == *"ok" {
                    info!("{}", command.data.options[0].options[0].name.as_str())
                }
                response
            }
            _ => "not implemented :(".to_string(),
        };
        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            }).await
        {
            println!("Cannot respond to slash command: {}", why);
        }
    }
}

pub async fn create_slash_commands(ctx: &Context) {
    let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
        commands
            .create_application_command(|command| {
                command.name("geo").description("Get GeoGuessr challenge link")
                    .create_option(|option| {
                        option.name("mode")
                            .description("The challenge's gamemode")
                            .kind(ApplicationCommandOptionType::SubCommandGroup)
                            .create_sub_option(|subopt| {
                                subopt.name("normal")
                                    .description("The default gamemode")
                                    .kind(ApplicationCommandOptionType::SubCommand)
                                    .create_sub_option(|subopt1| {
                                        subopt1.name("map")
                                            .required(true)
                                            .description("The challenge's map")
                                            .kind(ApplicationCommandOptionType::String)
                                    })
                                .create_sub_option(|subopt2| {
                                    subopt2.name("rules")
                                        .description("The challenge's ruleset")
                                        .required(true)
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Everything is allowed", "default")
                                        .add_string_choice("No moving", "nm")
                                        .add_string_choice("No zooming", "nz")
                                        .add_string_choice("No moving or zooming", "nmz")
                                        .add_string_choice("No moving, panning or zooming", "nmpz")
                                })
                                .create_sub_option(|subopt3| {
                                    subopt3.name("time")
                                        .description("The challenge's per-round time limit")
                                        .required(false)
                                        .kind(ApplicationCommandOptionType::Integer)
                                })
                            })
                        .create_sub_option(|subopt| {
                            subopt.name("brc")
                                .description("Battle-Royale Counries")
                                .kind(ApplicationCommandOptionType::SubCommand)
                                .create_sub_option(|subopt1| {
                                    subopt1.name("lobby")
                                        .description("Link to lobby")
                                        .required(false)
                                        .kind(ApplicationCommandOptionType::String)
                                })
                                .create_sub_option(|subopt2| {
                                    subopt2.name("rules")
                                        .description("The games' ruleset")
                                        .required(false)
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Everything is allowed", "default")
                                        .add_string_choice("No moving", "nm")
                                        .add_string_choice("No zooming", "nz")
                                        .add_string_choice("No moving or zooming", "nmz")
                                        .add_string_choice("No moving, panning or zooming", "nmpz")
                                })
                                .create_sub_option(|subopt3| {
                                    subopt3.name("powerups")
                                        .description("Powerups available in the game")
                                        .required(false)
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Both 5050 and spy are available", "All")
                                        .add_string_choice("No powerups are available", "None")
                                        .add_string_choice("The 5050-powerup is available", "5050")
                                        .add_string_choice("The Spy-powerup is available", "Spy")
                                })
                        })
                        .create_sub_option(|subopt| {
                            subopt.name("brd")
                                .description("Battle-Royale Distance")
                                .kind(ApplicationCommandOptionType::SubCommand)
                                .create_sub_option(|subopt1| {
                                    subopt1.name("lobby")
                                        .description("Link to lobby")
                                        .required(false)
                                        .kind(ApplicationCommandOptionType::String)
                                })
                            .create_sub_option(|subopt2| {
                                subopt2.name("rules")
                                    .description("The games' ruleset")
                                    .required(false)
                                    .kind(ApplicationCommandOptionType::String)
                                    .add_string_choice("Everything is allowed", "default")
                                    .add_string_choice("No moving", "nm")
                                    .add_string_choice("No zooming", "nz")
                                    .add_string_choice("No moving or zooming", "nmz")
                                    .add_string_choice("No moving, panning or zooming", "nmpz")
                            })
                        })
                        .create_sub_option(|subopt| {
                            subopt.name("cs")
                                .description("Country-Streak")
                                .kind(ApplicationCommandOptionType::SubCommand)
                                .create_sub_option(|subopt2| {
                                    subopt2.name("rules")
                                        .description("The challenge's ruleset")
                                        .required(true)
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Everything is allowed", "default")
                                        .add_string_choice("No moving", "nm")
                                        .add_string_choice("No zooming", "nz")
                                        .add_string_choice("No moving or zooming", "nmz")
                                        .add_string_choice("No moving, panning or zooming", "nmpz")
                                })
                            .create_sub_option(|subopt3| {
                                subopt3.name("time")
                                    .description("The challenge's per-round time limit")
                                    .kind(ApplicationCommandOptionType::Integer)
                            })
                        })
                    })
            })
    }).await.unwrap();
}
