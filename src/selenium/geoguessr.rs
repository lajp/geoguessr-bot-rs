use crate::WebDriverContainer;
use serenity::prelude::*;
use thirtyfour::prelude::*;
use std::env;
use dotenv;
use std::time::Duration;
use std::path::Path;

//BIG ASS FIXME: Do error handling instead of just unwrapping

pub async fn initialize_geoguessr() -> WebDriver {
    dotenv::dotenv().expect("Failed to load .env file");

    let email = env::var("GEOGUESSR_EMAIL").expect("Expected geoguessr email");
    let password = env::var("GEOGUESSR_PASSWORD").expect("Expected geoguessr password");

    let mut caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new_with_timeout("http://localhost:4444", &caps, Some(Duration::from_secs(10))).await.unwrap();

    driver.get("https://geoguessr.com/signin").await.unwrap();

    let email_field = driver.find_element(By::Name("email")).await.unwrap();
    let password_field = driver.find_element(By::Name("password")).await.unwrap();

    let login_button = driver.find_element(By::XPath("/html/body/div/div[2]/div/main/div/div/form/div/div[3]/div/button")).await.unwrap();

    email_field.send_keys(email).await.unwrap();
    password_field.send_keys(password).await.unwrap();
    login_button.click().await.unwrap();

    driver
}

async fn set_rules(driver: &WebDriver, rules: &str) {
    match rules {
        "default" => {
            driver.query(By::ClassName("radio-button")).first().await.unwrap().click().await.unwrap();
        },
        "nm" => {
            driver.query(By::ClassName("radio-button")).all().await.unwrap()[1].click().await.unwrap();
        },
        "nz" => {
            driver.query(By::ClassName("radio-button")).all().await.unwrap()[2].click().await.unwrap();
        },
        "nmz" => {
            driver.query(By::ClassName("radio-button")).all().await.unwrap()[3].click().await.unwrap();
        },
        "nmpz" => {
            driver.query(By::ClassName("radio-button")).all().await.unwrap()[4].click().await.unwrap();
        },
        _ => (),
    }
}

async fn set_time(driver: &WebDriver, time: Option<i64>) {
    let mut t = match time {
        Some(t) => t,
        None => 0,
    };
    let slider = driver.find_element(By::ClassName("rangeslider")).await.unwrap();
    if t > 600 {
        t = 600;
    }
    let amount: i32 = (t/10).try_into().unwrap();
    let width = slider.rect().await.unwrap().width;
    slider.click().await.unwrap();
    driver.action_chain().move_to_element_with_offset(&slider, ((-width/2.0) as i32)+12+(amount*5), 0).click().perform().await.unwrap();
}

pub async fn create_brc(ctx: &Context) -> String {
    let mut data = ctx.data.write().await;
    let driver = data.get_mut::<WebDriverContainer>().unwrap();
    driver.get("https://www.geoguessr.com/play-with-friends").await.unwrap();
    let lobby_button = driver.find_element(By::XPath("/html/body/div/div[2]/div/main/div[1]/div/div/div[2]/div/div[1]")).await.unwrap();
    lobby_button.click().await.unwrap();
    lobby_button.wait_until().ignore_errors(true).stale().await.unwrap();
    let url = driver.current_url().await.unwrap();
    driver.get("https://www.geoguessr.com/").await.unwrap();
    url
}

pub async fn create_brd(ctx: &Context) -> String {
    let mut data = ctx.data.write().await;
    let driver = data.get_mut::<WebDriverContainer>().unwrap();
    driver.get("https://www.geoguessr.com/play-with-friends").await.unwrap();
    let lobby_button = driver.find_element(By::XPath("/html/body/div/div[2]/div/main/div[1]/div/div/div[2]/div/div[2]")).await.unwrap();
    lobby_button.click().await.unwrap();
    lobby_button.wait_until().ignore_errors(true).stale().await.unwrap();
    let url = driver.current_url().await.unwrap();
    driver.get("https://www.geoguessr.com/").await.unwrap();
    url
}

pub async fn start_brc(ctx: &Context, lobby: &str, rules: &str, powerups: &str) {
    let mut data = ctx.data.write().await;
    let driver = data.get_mut::<WebDriverContainer>().unwrap();
    driver.get(lobby).await.unwrap();
    match rules {
        "default" => {
            let buttons = &driver.query(By::ClassName("lobby-options_option__2uuDh")).all().await.unwrap()[2..5];
            let images = &driver.query(By::ClassName("rule-icons_icon__3De99")).all().await.unwrap()[0..3];
            for i in 0..3 {
                if images[i].get_attribute("alt").await.unwrap().unwrap().contains("not") {
                    buttons[i].click().await.unwrap();
                }
            }
        },
        _ => {
            for a in rules.chars() {
                match a {
                    'm' => {
                        let button = &driver.query(By::ClassName("lobby-options_option__2uuDh")).all().await.unwrap()[2];
                        let img = &driver.query(By::ClassName("rule-icons_icon__3De99")).all().await.unwrap()[0];
                        if !img.get_attribute("alt").await.unwrap().unwrap().contains("not") {
                            button.click().await.unwrap();
                        }
                    },
                    'p' => {

                        let button = &driver.query(By::ClassName("lobby-options_option__2uuDh")).all().await.unwrap()[3];
                        let img = &driver.query(By::ClassName("rule-icons_icon__3De99")).all().await.unwrap()[1];
                        if !img.get_attribute("alt").await.unwrap().unwrap().contains("not") {
                            button.click().await.unwrap();
                        }
                    },
                    'z' => {
                        let button = &driver.query(By::ClassName("lobby-options_option__2uuDh")).all().await.unwrap()[4];
                        let img = &driver.query(By::ClassName("rule-icons_icon__3De99")).all().await.unwrap()[2];
                        if !img.get_attribute("alt").await.unwrap().unwrap().contains("not") {
                            button.click().await.unwrap();
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
                    driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button")).await.unwrap().click().await.unwrap();
                }
            }
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button/div/img")).await {
                Err(_) => (),
                Ok(_) => {
                    driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button")).await.unwrap().click().await.unwrap();
                }
            }
        },
        "Spy" => {
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button/div/img")).await {
                Ok(_) => (),
                Err(_) => {
                    driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button")).await.unwrap().click().await.unwrap();
                }
            }
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button/div/img")).await {
                Err(_) => (),
                Ok(_) => {
                    driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button")).await.unwrap().click().await.unwrap();
                }
            }
        }
        "All" => {
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button/div/img")).await {
                Ok(_) => (),
                Err(_) => {
                    driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button")).await.unwrap().click().await.unwrap();
                }
            }
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button/div/img")).await {
                Ok(_) => (),
                Err(_) => {
                    driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button")).await.unwrap().click().await.unwrap();
                }
            }
        }
        "None" => {
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button/div/img")).await {
                Err(_) => (),
                Ok(_) => {
                    driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[1]/button")).await.unwrap().click().await.unwrap();
                }
            }
            match driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button/div/img")).await {
                Err(_) => (),
                Ok(_) => {
                    driver.find_element(By::XPath("/html/body/div/main/div/div[2]/div[3]/div/div/div[2]/div[2]/button")).await.unwrap().click().await.unwrap();
                }
            }
        }
        _ => ()
    }
    driver.find_element(By::XPath("/html/body/div/main/div/div[3]/button")).await.unwrap().click().await.unwrap();
    driver.get("https://www.geoguessr.com/").await.unwrap();
}

pub async fn get_map(ctx: &Context, map: &str, rules: &str, time: Option<i64>) -> String {
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
    let driver = data.get_mut::<WebDriverContainer>().unwrap();
    driver.get(mapurl).await.unwrap();
    match driver.find_element(By::ClassName("rangeslider")).await {
        Ok(_) => (),
        Err(_) => {
            let no_default = driver.find_element(By::ClassName("checkbox")).await.unwrap();
            no_default.click().await.unwrap();
        }
    };
    set_rules(driver, rules).await;
    set_time(driver, time).await;

    let challenge_button = driver.find_element(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/article/div[2]/div/div[2]/label/div[1]")).await.unwrap();
    challenge_button.click().await.unwrap();

    let invite_button = driver.find_element(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/article/div[4]/button")).await.unwrap();
    invite_button.click().await.unwrap();

    let start_button = driver.query(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/article/div[2]/div[2]/button")).first().await.unwrap();
    start_button.click().await.unwrap();

    // A hacky way to get the challenge url (wait_until() will return on first error AKA when the
    // element stops existing)
    start_button.wait_until().ignore_errors(true).stale().await.unwrap();
    let url = driver.current_url().await.unwrap();
    driver.get("https://www.geoguessr.com/").await.unwrap();
    url
}

pub async fn get_cs(ctx: &Context, rules: &str, time: Option<i64>) -> String {
    let mut data = ctx.data.write().await;
    let driver = data.get_mut::<WebDriverContainer>().unwrap();
    driver.get("https://www.geoguessr.com/country-streak").await.unwrap();

    let challenge_button = driver.find_element(By::XPath("//*[@id=\"__next\"]/div/main/div/div/div/div/div/div/div[2]/article/div[2]/div/div[2]/label/div[1]")).await.unwrap();
    challenge_button.click().await.unwrap();

    match driver.find_element(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/div[2]/article/div[3]/div/div/div[2]/div[2]")).await {
        Ok(_) => (),
        Err(_) => {
            let no_default = driver.find_element(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/div[2]/article/div[3]/div/div/div/label/span[2]")).await.unwrap();
            no_default.click().await.unwrap();
        }
    };

    set_rules(driver, rules).await;
    set_time(driver, time).await;

    let invite_button = driver.find_element(By::XPath("//*[@id=\"__next\"]/div/main/div/div/div/div/div/div/div[2]/article/div[4]/button")).await.unwrap();
    invite_button.click().await.unwrap();

    let start_button = driver.query(By::XPath("/html/body/div/div/main/div/div/div/div/div/div/div[2]/article/div[3]/button/div")).first().await.unwrap();
    start_button.click().await.unwrap();

    // A hacky way to get the challenge url (wait_until() will return on first error AKA when the
    // element stops existing)
    start_button.wait_until().ignore_errors(true).stale().await.unwrap();
    let url = driver.current_url().await.unwrap();
    driver.get("https://www.geoguessr.com/").await.unwrap();
    url
}
