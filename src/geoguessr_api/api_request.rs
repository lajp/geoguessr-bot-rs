use reqwest;
use dotenv;
use serde_json;
use serde_json::Value;
use serde_json::json;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use tracing::info;
#[derive(Deserialize)]
struct Res {
    token: String,
}

#[derive(Deserialize)]
struct CreateBr {
    gameLobbyId: String,
}

pub async fn get_streaks_challenge(streaktype: &str, moving: &str, panning: &str, zooming: &str, time: i32) -> Result<String, ()>{
    dotenv::dotenv().expect("Failed to load .env file");
    let cookie = env::var("GEOGUESSR_AUTH_TOKEN").expect("Expected geoguessr cookies");
    let timestr = time.to_string();

    let mut map = HashMap::new();
    map.insert("forbidMoving", moving);
    map.insert("forbidRotating", panning);
    map.insert("forbidZooming", zooming);
    map.insert("streakType", streaktype);
    map.insert("timeLimit", &timestr);
    let client = reqwest::Client::new();
    let res = client.post("https://www.geoguessr.com/api/v3/challenges/streak")
        .header(reqwest::header::COOKIE, cookie)
        .json(&map)
        .send()
        .await.unwrap();

    Ok(format!("https://geoguessr.com/challenge/{}", res.json::<Res>().await.unwrap().token))
}

async fn search_for_map(mapname: &str, cookie: &str) -> Result<String, ()> {
    let mapname = mapname.replace(" ", "+");
    info!("Querying for map: {}", mapname);
    let client = reqwest::Client::new();
    let res;
    if mapname != "random" {
        res = client.get(format!("https://www.geoguessr.com/api/v3/search/map?page=0&count=1&q={}", mapname))
            .header(reqwest::header::COOKIE, cookie)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    }
    else {
        res = client.get("https://www.geoguessr.com/api/v3/social/maps/browse/random?count=1")
            .header(reqwest::header::COOKIE, cookie)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    }

    let v: Value =serde_json::from_str(&res).unwrap();
    let id;
    if let Some(varray) = v.as_array() {
        if varray.is_empty() {
            info!("No map found!");
            return Err(());
        }
        id = varray[0]["id"].to_string().replace('"', "");
    }
    else {
        id = v.as_object().unwrap()["id"].to_string().replace('"', "");
    }
    println!("{:?}", id);
    Ok(id)
}

pub async fn get_classic_challenge(mapname: &str, moving: &str, panning: &str, zooming: &str, time: i32) -> Result<String, ()> {
    dotenv::dotenv().expect("Failed to load .env file");
    let cookie = env::var("GEOGUESSR_AUTH_TOKEN").expect("Expected geoguessr cookie");
    let timestr = time.to_string();

    let mapid = match search_for_map(mapname, &cookie).await {
        Ok(m) => m,
        Err(_) => return Err(())
    };

    let mut map = HashMap::new();
    map.insert("forbidMoving", moving);
    map.insert("forbidRotating", panning);
    map.insert("forbidZooming", zooming);
    map.insert("map", &mapid);
    map.insert("timeLimit", &timestr);

    let client = reqwest::Client::new();
    let res = client.post("https://www.geoguessr.com/api/v3/challenges")
        .header(reqwest::header::COOKIE, cookie)
        .json(&map)
        .send()
        .await.unwrap();

    Ok(format!("https://geoguessr.com/challenge/{}", res.json::<Res>().await.unwrap().token))
}

pub async fn get_battleroyale_lobby() -> Result<String, ()> {
    dotenv::dotenv().expect("Failed to load .env file");
    let cookie = env::var("GEOGUESSR_AUTH_TOKEN").expect("Expected geoguessr cookie");
    let mut map = HashMap::new();
    map.insert("gameType", "BattleRoyaleCountries");

    let client = reqwest::Client::new();
    let res = client.post("https://game-server.geoguessr.com/api/lobby")
        .header(reqwest::header::COOKIE, cookie)
        .json(&map)
        .send()
        .await.unwrap();

    Ok(format!("https://www.geoguessr.com/battle-royale/{}", res.json::<CreateBr>().await.unwrap().gameLobbyId))
}

pub async fn start_battleroyale(gametype: &str, lobby: &str, moving: &str, panning: &str, zooming: &str, fiftyfifty: &str, spy: &str) {
    dotenv::dotenv().expect("Failed to load .env file");
    let cookie = env::var("GEOGUESSR_AUTH_TOKEN").expect("Expected geoguessr cookie");
    let mut map = HashMap::new();

    let game_type_bool = match gametype {
        "BattleRoyaleDistance" => "true",
        _ => "false",
    };

    map.insert("forbidMoving", moving);
    map.insert("forbidPanning", panning);
    map.insert("forbidZooming", zooming);
    map.insert("powerUp5050", fiftyfifty);
    map.insert("powerUpSpy", spy);
    map.insert("isDistanceGame", game_type_bool);

    // Potentially modify these for some wacky games..?
    map.insert("initialLives", "3");
    map.insert("extraLivesEachRound", "0");
    map.insert("guessCooldown", "0");
    map.insert("firstRoundStartDelay", "6");
    map.insert("mapSlug", "world");
    map.insert("reservationWindowTime", "15");
    map.insert("resetLivesEachRound", "true");
    map.insert("roundStartDelay", "6");
    map.insert("roundTime", "90");

    let lobbyid = lobby.rsplit('/').next().unwrap();

    let client = reqwest::Client::new();
    client.put(format!("https://game-server.geoguessr.com/api/lobby/{}/options", lobbyid))
        .header(reqwest::header::COOKIE, cookie.clone())
        .json(&map)
        .send()
        .await.unwrap();

    client.post(format!("https://game-server.geoguessr.com/api/lobby/{}/join", lobbyid))
        .header(reqwest::header::COOKIE, cookie.clone())
        .json(&json!(null))
        .send()
        .await.unwrap();

    client.post(format!("https://game-server.geoguessr.com/api/lobby/{}/start", lobbyid))
        .header(reqwest::header::COOKIE, cookie)
        .json(&json!(null))
        .send()
        .await.unwrap();

}
