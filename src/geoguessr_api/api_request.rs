use dotenv;
use reqwest;
use serde::Deserialize;
use serde_json;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use tracing::info;
#[derive(Deserialize)]
struct Res {
    token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateBr {
    game_lobby_id: String,
}

pub async fn get_streaks_challenge(
    streaktype: &str,
    moving: bool,
    panning: bool,
    zooming: bool,
    time: i32,
) -> Result<String, anyhow::Error> {
    dotenv::dotenv().expect("Failed to load .env file");
    let cookie = env::var("GEOGUESSR_AUTH_TOKEN").expect("Expected geoguessr cookies");
    let req = json!({ "forbidMoving": moving, "forbidRotating": panning, "forbidZooming": zooming, "streakType": streaktype, "timeLimit": time});
    let client = reqwest::Client::new();
    let res = client
        .post("https://www.geoguessr.com/api/v3/challenges/streak")
        .header(reqwest::header::COOKIE, cookie)
        .json(&req)
        .send()
        .await?;

    Ok(format!(
        "https://geoguessr.com/challenge/{}",
        res.json::<Res>().await?.token
    ))
}

async fn search_for_map(mapname: &str, cookie: &str) -> Result<String, anyhow::Error> {
    let mapname = mapname.replace(" ", "+");
    info!("Querying for map: {}", mapname);
    let client = reqwest::Client::new();
    let res = if mapname != "random" {
        client
            .get(format!(
                "https://www.geoguessr.com/api/v3/search/map?page=0&count=1&q={}",
                mapname
            ))
            .header(reqwest::header::COOKIE, cookie)
            .send()
            .await?
            .text()
            .await?
    } else {
        client
            .get("https://www.geoguessr.com/api/v3/social/maps/browse/random?count=1")
            .header(reqwest::header::COOKIE, cookie)
            .send()
            .await?
            .text()
            .await?
    };
    let v: Value = serde_json::from_str(&res)?;
    let id = if let Some(varray) = v.as_array() {
        if varray.is_empty() {
            anyhow::bail!("No map found map!");
        }
        varray[0]["id"].to_string().replace('"', "")
    } else {
        v.as_object().ok_or(anyhow::anyhow!("Unknown API error"))?["id"]
            .to_string()
            .replace('"', "")
    };
    Ok(id)
}

pub async fn get_classic_challenge(
    mapname: &str,
    moving: bool,
    panning: bool,
    zooming: bool,
    time: i32,
) -> Result<String, anyhow::Error> {
    dotenv::dotenv().expect("Failed to load .env file");
    let cookie = env::var("GEOGUESSR_AUTH_TOKEN").expect("Expected geoguessr cookie");

    let mapid = search_for_map(mapname, &cookie).await?;

    let req = json!({ "forbidMoving": moving, "forbidRotating": panning, "forbidZooming": zooming, "map": mapid, "timeLimit": time});

    let client = reqwest::Client::new();
    let res = client
        .post("https://www.geoguessr.com/api/v3/challenges")
        .header(reqwest::header::COOKIE, cookie)
        .json(&req)
        .send()
        .await?;

    Ok(format!(
        "https://geoguessr.com/challenge/{}",
        res.json::<Res>().await?.token
    ))
}

pub async fn get_battleroyale_lobby() -> Result<String, anyhow::Error> {
    let cookie = env::var("GEOGUESSR_AUTH_TOKEN").expect("Expected geoguessr cookie");
    let mut map = HashMap::new();
    map.insert("gameType", "BattleRoyaleCountries");

    let client = reqwest::Client::new();
    let res = client
        .post("https://game-server.geoguessr.com/api/lobby")
        .header(reqwest::header::COOKIE, cookie)
        .json(&map)
        .send()
        .await?;

    Ok(format!(
        "https://www.geoguessr.com/battle-royale/{}",
        res.json::<CreateBr>().await?.game_lobby_id
    ))
}

pub async fn start_battleroyale(
    gametype: &str,
    lobby: &str,
    moving: &str,
    panning: &str,
    zooming: &str,
    fiftyfifty: &str,
    spy: &str,
) -> Result<(), anyhow::Error> {
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

    let lobbyid = lobby
        .rsplit('/')
        .next()
        .ok_or(anyhow::anyhow!("Unknown API error"))?;

    let client = reqwest::Client::new();
    client
        .put(format!(
            "https://game-server.geoguessr.com/api/lobby/{}/options",
            lobbyid
        ))
        .header(reqwest::header::COOKIE, &cookie)
        .json(&map)
        .send()
        .await?;

    client
        .post(format!(
            "https://game-server.geoguessr.com/api/lobby/{}/join",
            lobbyid
        ))
        .header(reqwest::header::COOKIE, &cookie)
        .json(&json!(null))
        .send()
        .await?;

    client
        .post(format!(
            "https://game-server.geoguessr.com/api/lobby/{}/start",
            lobbyid
        ))
        .header(reqwest::header::COOKIE, &cookie)
        .json(&json!(null))
        .send()
        .await?;

    Ok(())
}
