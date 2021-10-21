use serenity::prelude::*;
use thirtyfour::prelude::*;
use std::env;
use dotenv;
use std::time::Duration;
use tracing::error;

use crate::WebDriverContainer;
use crate::WebDriverError;

pub async fn initialize_geoguessr() -> Result<WebDriver, WebDriverError> {
    dotenv::dotenv().expect("Failed to load .env file");

    let email = env::var("GEOGUESSR_EMAIL").expect("Expected geoguessr email");
    let password = env::var("GEOGUESSR_PASSWORD").expect("Expected geoguessr password");

    let caps = DesiredCapabilities::firefox();
    let driver = match WebDriver::new_with_timeout("http://localhost:4444", &caps, Some(Duration::from_secs(10))).await {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to open browser session!");
            return Err(e);
        }
    };

    match driver.get("https://geoguessr.com/signin").await {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to `get()` signin-page!");
            return Err(e);
        }
    }

    let email_field = match driver.find_element(By::Name("email")).await {
        Ok(e) => e,
        Err(e) => {
            error!("Was unable to find email-field!");
            return Err(e);
        }
    };
    let password_field = match driver.find_element(By::Name("password")).await {
        Ok(e) => e,
        Err(e) => {
            error!("Was unable to find password-field!");
            return Err(e);
        }
    };

    let login_button = match driver.find_element(By::XPath("/html/body/div/div[2]/div/main/div/div/form/div/div[3]/div/button")).await {
        Ok(e) => e,
        Err(e) => {
            error!("Was unable to find login-button!");
            return Err(e);
        }
    };

    match email_field.send_keys(email).await {
        Ok(_) => (),
        Err(e) => {
            error!("Was unable to send email to email-field!");
            return Err(e);
        }
    }
    match password_field.send_keys(password).await {
        Ok(_) => (),
        Err(e) => {
            error!("Was unable to send password to password-field!");
            return Err(e);
        }
    }
    match login_button.click().await {
        Ok(_) => (),
        Err(e) => {
            error!("Was unable to click login-button!");
            return Err(e);
        }
    }

    Ok(driver)
}

async fn set_rules(driver: &WebDriver, rules: &str) -> Result<(), WebDriverError> {
    match rules {
        "default" => {
            match driver.query(By::ClassName("radio-button")).first().await {
                Ok(b) => match b.click().await {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Was unable to click default-rules button!");
                        return Err(e);
                    }
                }
                Err(e) => {
                    error!("Was unable to find default-rules button!");
                    return Err(e);
                }
            }
        },
        "nm" => {
            match driver.query(By::ClassName("radio-button")).all().await {
                Ok(b) => match b[1].click().await {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Was unable to click no-moving button!");
                        return Err(e);
                    }
                }
                Err(e) => {
                    error!("Was unable to find no-moving button!");
                    return Err(e);
                }
            }
        },
        "nz" => {
            match driver.query(By::ClassName("radio-button")).all().await {
                Ok(b) => match b[2].click().await {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Was unable to click no-zooming button!");
                        return Err(e);
                    }
                }
                Err(e) => {
                    error!("Was unable to find no-zooming button!");
                    return Err(e);
                }
            }
        },
        "nmz" => {
            match driver.query(By::ClassName("radio-button")).all().await {
                Ok(b) => match b[3].click().await {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Was unable to click no-moving-or-zooming button!");
                        return Err(e);
                    }
                }
                Err(e) => {
                    error!("Was unable to find no-moving-or-zooming button!");
                    return Err(e);
                }
            }
        },
        "nmpz" => {
            match driver.query(By::ClassName("radio-button")).all().await {
                Ok(b) => match b[4].click().await {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Was unable to click no-moving-panning-or-zooming button!");
                        return Err(e);
                    }
                }
                Err(e) => {
                    error!("Was unable to find no-moving-panning-or-zooming button!");
                    return Err(e);
                }
            }
        },
        _ => (),
    }
    Ok(())
}

async fn set_time(driver: &WebDriver, time: Option<i64>) -> Result<(), WebDriverError> {
    let mut t = match time {
        Some(t) => t,
        None => 0,
    };
    let slider = match driver.find_element(By::ClassName("rangeslider")).await {
        Ok(s) => s,
        Err(e) => {
            error!("Was unable to find time-slider!");
            return Err(e);
        }
    };
    if t > 600 {
        t = 600;
    }
    let amount: i32 = (t/10).try_into().unwrap();
    let width = slider.rect().await.unwrap().width;
    match slider.click().await {
        Ok(_) => (),
        Err(e) => {
            error!("Was unable to click time-slider!");
            return Err(e);
        }
    };
    match driver.action_chain().move_to_element_with_offset(&slider, ((-width/2.0) as i32)+12+(amount*5), 0).click().perform().await {
        Ok(_) => (),
        Err(e) => {
            error!("Was unable to perform time-setting actionchain!");
            return Err(e);
        }
    }
    Ok(())
}

pub async fn create_brc(ctx: &Context) -> Result<String, WebDriverError> {
    let mut data = ctx.data.write().await;
    let driver = data.get_mut::<WebDriverContainer>().unwrap().lock().await;
    match driver.get("https://www.geoguessr.com/play-with-friends").await {
        Ok(_) => (),
        Err(e) => {
            error!("Was unable to get play-with-friends page!");
            return Err(e);
        }
    };
    let lobby_button = match driver.find_element(By::XPath("/html/body/div/div[2]/div/main/div[1]/div/div/div[2]/div/div[1]")).await {
        Ok(e) => e,
        Err(e) => {
            error!("Was unable to find lobby-button");
            return Err(e);
        }
    };
    match lobby_button.click().await {
        Ok(_) => (),
        Err(e) => {
            error!("Was unable to click lobby-button");
            return Err(e);
        }
    }
    lobby_button.wait_until().ignore_errors(true).stale().await.unwrap();
    let url = driver.current_url().await.unwrap();
    driver.get("https://www.geoguessr.com/").await.unwrap();
    Ok(url)
}

pub async fn create_brd(ctx: &Context) -> Result<String, WebDriverError> {
    let mut data = ctx.data.write().await;
    let driver = data.get_mut::<WebDriverContainer>().unwrap().lock().await;
    match driver.get("https://www.geoguessr.com/play-with-friends").await {
        Ok(_) => (),
        Err(e) => {
            error!("Was unable to get play-with-friends page!");
            return Err(e);
        }
    };
    let lobby_button = match driver.find_element(By::XPath("/html/body/div/div[2]/div/main/div[1]/div/div/div[2]/div/div[2]")).await {
        Ok(e) => e,
        Err(e) => {
            error!("Was unable to click lobby-button");
            return Err(e);
        }
    };
    match lobby_button.click().await {
        Ok(_) => (),
        Err(e) => {
            error!("Was unable to click lobby-button");
            return Err(e);
        }
    }
    lobby_button.wait_until().ignore_errors(true).stale().await.unwrap();
    let url = driver.current_url().await.unwrap();
    driver.get("https://www.geoguessr.com/").await.unwrap();
    Ok(url)
}

pub async fn start_brc(ctx: &Context, lobby: &str, rules: &str, powerups: &str) -> Result<(), WebDriverError> {
    let mut data = ctx.data.write().await;
    let driver = data.get_mut::<WebDriverContainer>().unwrap().lock().await;
    match driver.get(lobby).await {
        Ok(_) => (),
        Err(e) => {
            error!("Was unable to get lobby!");
            return Err(e);
        }
    };
    let buttons = match driver.query(By::ClassName("lobby-options_option__2uuDh")).all().await {
        Ok(b) => b,
        Err(e) => {
            error!("Was unable to get rules-buttons!");
            return Err(e);
        }
    };
    let images = match driver.query(By::ClassName("rule-icons_icon__3De99")).all().await {
        Ok(i) => i,
        Err(e) => {
            error!("Was unable to get rule-images!");
            return Err(e);
        }
    };
    match rules {
        "default" => {
            for i in 0..3 {
                // FIXME: Somehow report errors here
                if images[2+i].get_attribute("alt").await.unwrap().unwrap().contains("not") {
                    buttons[i].click().await.unwrap();
                }
            }
        },
        _ => {
            for a in rules.chars() {
                match a {
                    'm' => {
                        if !images[0].get_attribute("alt").await.unwrap().unwrap().contains("not") {
                            buttons[2].click().await.unwrap();
                        }
                    },
                    'p' => {

                        if !images[1].get_attribute("alt").await.unwrap().unwrap().contains("not") {
                            buttons[3].click().await.unwrap();
                        }
                    },
                    'z' => {
                        if !images[2].get_attribute("alt").await.unwrap().unwrap().contains("not") {
                            buttons[4].click().await.unwrap();
                        }
                    },
                    _ => (),
                }
            }
        }
    };
    match powerups {
        "5050" => {
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button/div/img")).await {
                Ok(_) => (),
                Err(_) => {
                    match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button")).await {
                        Ok(b) => b.click().await.unwrap(),
                        Err(e) => {
                            error!("Unable to find 5050-button!");
                            return Err(e);
                        }
                    }
                }
            }
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button/div/img")).await {
                Err(_) => (),
                Ok(_) => {
                    match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button")).await {
                        Ok(b) => b.click().await.unwrap(),
                        Err(e) => {
                            error!("Unable to find 5050-button");
                            return Err(e);
                        }
                    }
                }
            }
        },
        "Spy" => {
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button/div/img")).await {
                Ok(_) => (),
                Err(_) => {
                    match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button")).await {
                        Ok(b) => b.click().await.unwrap(),
                        Err(e) => {
                            error!("Unable to find Spy-button");
                            return Err(e);
                        }
                    }
                }
            }
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button/div/img")).await {
                Err(_) => (),
                Ok(_) => {
                    match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button")).await {
                        Ok(b) => b.click().await.unwrap(),
                        Err(e) => {
                            error!("Unable to find Spy-button");
                            return Err(e);
                        }
                    }
                }
            }
        }
        "All" => {
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button/div/img")).await {
                Ok(_) => (),
                Err(_) => {
                    match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button")).await {
                        Ok(b) => b.click().await.unwrap(),
                        Err(e) => {
                            error!("Unable to find Spy-button");
                            return Err(e);
                        }
                    }
                }
            }
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button/div/img")).await {
                Ok(_) => (),
                Err(_) => {
                    match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button")).await {
                        Ok(b) => b.click().await.unwrap(),
                        Err(e) => {
                            error!("Unable to find Spy-button");
                            return Err(e);
                        }
                    }
                }
            }
        }
        "None" => {
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button/div/img")).await {
                Err(_) => (),
                Ok(_) => {
                    match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button")).await {
                        Ok(b) => b.click().await.unwrap(),
                        Err(e) => {
                            error!("Unable to find Spy-button");
                            return Err(e);
                        }
                    }
                }
            }
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button/div/img")).await {
                Err(_) => (),
                Ok(_) => {
                    match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button")).await {
                        Ok(b) => b.click().await.unwrap(),
                        Err(e) => {
                            error!("Unable to find Spy-button");
                            return Err(e);
                        }
                    }
                }
            }
        }
        _ => ()
    }
    match driver.find_element(By::XPath("/html/body/div/main/div/div[3]/button")).await {
        Ok(b) => b.click().await.unwrap(),
        Err(e) => {
            error!("Unable to click start-game button!");
            return Err(e);
        }
    };
    driver.get("https://www.geoguessr.com/").await.unwrap();
    Ok(())
}

pub async fn get_map(ctx: &Context, map: &str, rules: &str, time: Option<i64>) -> Result<String, WebDriverError> {
    let mut mapurl = map.to_string();
    if !map.starts_with("https://") {
        mapurl = format!("https://www.geoguessr.com/maps/{}/play", map);
    }
    else if mapurl.ends_with('/') {
        mapurl = format!("{}play", mapurl);
    }
    else {
        mapurl = format!("{}/play", mapurl);
    }
    let mut data = ctx.data.write().await;
    let driver = data.get_mut::<WebDriverContainer>().unwrap().lock().await;
    match driver.get(mapurl).await {
        Ok(_) => (),
        Err(e) => {
            error!("Unable to get mapurl!");
            return Err(e);
        }
    };
    match driver.find_element(By::ClassName("rangeslider")).await {
        Ok(_) => (),
        Err(_) => {
            let no_default = match driver.find_element(By::ClassName("checkbox")).await {
                Ok(b) => b,
                Err(e) => {
                    error!("Unable to get no-default button!");
                    return Err(e);
                },
            };
            no_default.click().await.unwrap();
        }
    };
    match set_rules(&driver, rules).await {
        Ok(_) => (),
        Err(e) => return Err(e),
    };
    match set_time(&driver, time).await {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    let challenge_button = match driver.find_element(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/article/div[2]/div/div[2]/label/div[1]")).await {
        Ok(b) => b,
        Err(e) => {
            error!("Unable to find challenge-button!");
            return Err(e);
        },
    };
    challenge_button.click().await.unwrap();

    let invite_button = match driver.find_element(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/article/div[4]/button")).await {
        Ok(b) => b,
        Err(e) => {
            error!("Unable to find invite-button!");
            return Err(e);
        },
    };
    invite_button.click().await.unwrap();

    let start_button = match driver.query(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/article/div[2]/div[2]/button")).first().await {
        Ok(b) => b,
        Err(e) => {
            error!("Unable to find start-button!");
            return Err(e);
        },
    };
    start_button.click().await.unwrap();

    // A hacky way to get the challenge url (wait_until() will return on first error AKA when the
    // element stops existing)
    start_button.wait_until().ignore_errors(true).stale().await.unwrap();
    let url = driver.current_url().await.unwrap();
    driver.get("https://www.geoguessr.com/").await.unwrap();
    Ok(url)
}

pub async fn get_cs(ctx: &Context, rules: &str, time: Option<i64>) -> Result<String, WebDriverError> {
    let mut data = ctx.data.write().await;
    let driver = data.get_mut::<WebDriverContainer>().unwrap().lock().await;
    match driver.get("https://www.geoguessr.com/country-streak").await {
        Ok(_) => (),
        Err(e) => {
            error!("Unable to get country-streak page!");
            return Err(e);
        }
    };

    let challenge_button = match driver.find_element(By::XPath("//*[@id=\"__next\"]/div/main/div/div/div/div/div/div/div[2]/article/div[2]/div/div[2]/label/div[1]")).await {
        Ok(b) => b,
        Err(e) => {
            error!("Unable to get challenge-button!");
            return Err(e);
        }
    };
    challenge_button.click().await.unwrap();

    match driver.find_element(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/div[2]/article/div[3]/div/div/div[2]/div[2]")).await {
        Ok(_) => (),
        Err(_) => {
            let no_default = match driver.find_element(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/div[2]/article/div[3]/div/div/div/label/span[2]")).await {
                Ok(b) => b,
                Err(e) => {
                    error!("Unable to get no-default button!");
                    return Err(e);
                }
            };
            no_default.click().await.unwrap();
        }
    };

    match set_rules(&driver, rules).await {
        Ok(_) => (),
        Err(e) => return Err(e),
    };
    match set_time(&driver, time).await {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    let invite_button = match driver.find_element(By::XPath("//*[@id=\"__next\"]/div/main/div/div/div/div/div/div/div[2]/article/div[4]/button")).await {
        Ok(b) => b,
        Err(e) => {
            error!("Unable to find invite-button!");
            return Err(e);
        }
    };
    invite_button.click().await.unwrap();

    let start_button = match driver.query(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/div[2]/article/div[3]/button/div")).first().await {
        Ok(b) => b,
        Err(e) => {
            error!("Unable to find start-button!");
            return Err(e);
        }
    };
    start_button.click().await.unwrap();

    // A hacky way to get the challenge url (wait_until() will return on first error AKA when the
    // element stops existing)
    start_button.wait_until().ignore_errors(true).stale().await.unwrap();
    let url = driver.current_url().await.unwrap();
    driver.get("https://www.geoguessr.com/").await.unwrap();
    Ok(url)
}
