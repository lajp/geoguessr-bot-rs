# GeoGuessr-bot-rs

This is a simple implementation of a discord bot that send GeoGuessr-challenge links on demand.

## Features:
* Slash-commands

## Requirements:
* 1 GeoGuessr pro account

## Usage:

### An example of the .env file
```env
DISCORD_TOKEN=YourBotsToken
APPLICATION_ID=YourBotsAppId

GEOGUESSR_AUTH_TOKEN=YourLoginCookies

RUST_LOG=info
```

Run the compiled binary (**Remember to setup your .env**)
```sh
./target/release/geoguessr-bot-rs
```

In discord, just start typing `/geo` on a channel on which the bot has access to and follow the suggested entries.
