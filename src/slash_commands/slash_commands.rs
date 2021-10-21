use crate::Context;
use serenity::model::interactions::{Interaction, application_command::*, InteractionResponseType};
use serde_json::json;
use tracing::info;
use crate::geoguessr_api::api_request::*;

pub async fn handle_interaction(ctx: Context, interaction: Interaction) {
    let response;
    if let Interaction::ApplicationCommand(command) = interaction {
        let content = match command.data.name.as_str() {
            "geo" => {
                match command.data.options[0].options[0].name.as_str() {
                    "classic" => {
                        let mut mapname = json!("None");
                        let mut moving = json!("false");
                        let mut panning = json!("false");
                        let mut zooming = json!("false");
                        let mut time = 0;
                        for a in &command.data.options[0].options[0].options {
                            match a.name.as_str() {
                                "map" => {
                                    mapname = match a.value.clone() {
                                        Some(v) => v,
                                        None => {
                                            json!("None")
                                        }
                                    }
                                }
                                "moving" => {
                                    moving = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("false"),
                                    };
                                },
                                "panning" => {
                                    panning = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("false"),
                                    };
                                },
                                "zooming" => {
                                    zooming = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("false"),
                                    };
                                },
                                "time" => {
                                    time = match a.value.clone() {
                                        Some(v) => v.as_i64().unwrap().try_into().unwrap(),
                                        None => 0,
                                    }
                                },
                                _ => (),
                            }
                        }
                        response = get_classic_challenge(mapname.as_str().unwrap(), moving.as_str().unwrap(), panning.as_str().unwrap(), zooming.as_str().unwrap(), time)
                            .await.unwrap();
                    },
                    "streaks" => {
                        let mut moving = json!("false");
                        let mut panning = json!("false");
                        let mut zooming = json!("false");
                        let mut streaktype = json!("CountryStreak");
                        let mut time = 0;
                        for a in &command.data.options[0].options[0].options {
                            match a.name.as_str() {
                                "streaktype" => {
                                    streaktype = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("CountryStreak"),
                                    };
                                },
                                "moving" => {
                                    moving = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("false"),
                                    };
                                },
                                "panning" => {
                                    panning = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("false"),
                                    };
                                },
                                "zooming" => {
                                    zooming = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("false"),
                                    };
                                },
                                "time" => {
                                    time = match a.value.clone() {
                                        Some(v) => v.as_i64().unwrap().try_into().unwrap(),
                                        None => 0,
                                    }
                                },
                                _ => (),
                            }
                        }
                        response = get_streaks_challenge(streaktype.as_str().unwrap(), moving.as_str().unwrap(), panning.as_str().unwrap(), zooming.as_str().unwrap(), time).await.unwrap();
                    },
                    "battle-royale" => {
                        let mut gametype = json!("BattleRoyaleCountries");
                        let mut lobby = json!("None");
                        let mut moving = json!("false");
                        let mut panning = json!("false");
                        let mut zooming = json!("false");
                        let mut fiftyfifty = json!("false");
                        let mut spy = json!("false");
                        let mut create = true;
                        for a in &command.data.options[0].options[0].options {
                            match a.name.as_str() {
                                "gametype" => {
                                    gametype = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("BattleRoyaleCountries")
                                    }
                                }
                                "lobby" => {
                                    lobby = match a.value.clone() {
                                        Some(v) => {
                                            create = false;
                                            v
                                        },
                                        None => json!("None"),
                                    }
                                }
                                "moving" => {
                                    moving = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("false"),
                                    };
                                },
                                "panning" => {
                                    panning = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("false"),
                                    };
                                },
                                "zooming" => {
                                    zooming = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("false"),
                                    };
                                },
                                "5050" => {
                                    fiftyfifty = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("false"),
                                    };
                                }
                                "spy" => {
                                    spy = match a.value.clone() {
                                        Some(v) => v,
                                        None => json!("false"),
                                    };
                                }
                                _ => continue
                            }
                        }
                        if create {
                            response = get_battleroyale_lobby().await.unwrap();
                        } else {
                            start_battleroyale(gametype.as_str().unwrap(), lobby.as_str().unwrap(), moving.as_str().unwrap(), panning.as_str().unwrap(),
                                zooming.as_str().unwrap(), fiftyfifty.as_str().unwrap(), spy.as_str().unwrap()).await;
                            response = "Starting game...".to_string()
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
    ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
        commands
            .create_application_command(|command| {
                command.name("geo").description("Get GeoGuessr challenge link")
                    .create_option(|option| {
                        option.name("mode")
                            .description("The challenge's gamemode")
                            .kind(ApplicationCommandOptionType::SubCommandGroup)
                            .create_sub_option(|subopt| {
                                subopt.name("classic")
                                    .description("The default gamemode")
                                    .kind(ApplicationCommandOptionType::SubCommand)
                                    .create_sub_option(|subopt1| {
                                        subopt1.name("map")
                                            .required(true)
                                            .description("The challenge's map")
                                            .kind(ApplicationCommandOptionType::String)
                                    })
                                .create_sub_option(|subopt2| {
                                    subopt2.name("moving")
                                        .description("The challenge's ruleset")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Moving is allowed", "false")
                                        .add_string_choice("Moving is not allowed", "true")
                                })
                                .create_sub_option(|subopt2| {
                                    subopt2.name("panning")
                                        .description("The challenge's ruleset")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Panning is allowed", "false")
                                        .add_string_choice("Panning is not allowed", "true")
                                })
                                .create_sub_option(|subopt2| {
                                    subopt2.name("zooming")
                                        .description("The challenge's ruleset")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Zooming is allowed", "false")
                                        .add_string_choice("Zooming is not allowed", "true")
                                })
                                .create_sub_option(|subopt3| {
                                    subopt3.name("time")
                                        .description("The challenge's per-round time limit")
                                        .kind(ApplicationCommandOptionType::Integer)
                                })
                            })
                        .create_sub_option(|subopt| {
                            subopt.name("battle-royale")
                                .description("Battle Royale")
                                .kind(ApplicationCommandOptionType::SubCommand)
                                .create_sub_option(|subopt1| {
                                    subopt1.name("gametype")
                                        .description("Type of the game")
                                        .required(false)
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Battle-Royale Countries", "BattleRoyaleCountries")
                                        .add_string_choice("Battle-Royale Distance", "BattleRoyaleDistance")
                                })
                                .create_sub_option(|subopt1| {
                                    subopt1.name("lobby")
                                        .description("Link to lobby")
                                        .required(false)
                                        .kind(ApplicationCommandOptionType::String)
                                })
                                .create_sub_option(|subopt2| {
                                    subopt2.name("moving")
                                        .description("The challenge's ruleset")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Moving is allowed", "false")
                                        .add_string_choice("Moving is not allowed", "true")
                                })
                                .create_sub_option(|subopt2| {
                                    subopt2.name("panning")
                                        .description("The challenge's ruleset")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Panning is allowed", "false")
                                        .add_string_choice("Panning is not allowed", "true")
                                })
                                .create_sub_option(|subopt2| {
                                    subopt2.name("zooming")
                                        .description("The challenge's ruleset")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Zooming is allowed", "false")
                                        .add_string_choice("Zooming is not allowed", "true")
                                })
                                .create_sub_option(|subopt3| {
                                    subopt3.name("5050")
                                        .description("The 5050 powerup")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("The 5050-powerup is available", "true")
                                        .add_string_choice("The 5050-powerup is not available", "false")
                                })
                                .create_sub_option(|subopt3| {
                                    subopt3.name("spy")
                                        .description("The Spy powerup")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("The Spy-powerup is available", "true")
                                        .add_string_choice("The Spy-powerup is not available", "false")
                                })
                        })
                        .create_sub_option(|subopt| {
                            subopt.name("streaks")
                                .description("Streaks-gamemode")
                                .kind(ApplicationCommandOptionType::SubCommand)
                                .create_sub_option(|subopt2| {
                                    subopt2.name("streaktype")
                                        .description("The streaktype")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Country-Streak", "CountryStreak")
                                        .add_string_choice("US state streak", "UsStateStreak")
                                })
                                .create_sub_option(|subopt2| {
                                    subopt2.name("moving")
                                        .description("The challenge's ruleset")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Moving is allowed", "false")
                                        .add_string_choice("Moving is not allowed", "true")
                                })
                                .create_sub_option(|subopt2| {
                                    subopt2.name("panning")
                                        .description("The challenge's ruleset")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Panning is allowed", "false")
                                        .add_string_choice("Panning is not allowed", "true")
                                })
                                .create_sub_option(|subopt2| {
                                    subopt2.name("zooming")
                                        .description("The challenge's ruleset")
                                        .kind(ApplicationCommandOptionType::String)
                                        .add_string_choice("Zooming is allowed", "false")
                                        .add_string_choice("Zooming is not allowed", "true")
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
