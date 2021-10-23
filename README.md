# GeoGuessr-bot-rs

This is a simple implementation of a discord bot that send GeoGuessr-challenge links on demand.

## Features:
* Slash-commands
* Lightning-fast response times
* Minimal resources used

## Requirements:
* 1 GeoGuessr pro account

## Setup:

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

## How the Bot works

The bot works by calling the (undocumented) GeoGuessr-API with reverse-engineered http-request-calls.
This allows the bot to be lightning-fast and reliable even in multi-user applications.
As a side product, the bot is able to generate challenges that are not possible to be created with
the GeoGuessr-UI but that are still allowed by the API (such as challenges with massive/tiny per-round time limits).
The bot authenticates with cookies specified in the `.env`-file.

## Usage

### Basically:
In discord, just start typing `/geo` on a channel on which the bot has access to and follow the suggested entries.

### Walkthrough of different subcommands and ther suboptions:

There are 3 main subcommands (modes) of the `/geo` slash-command.
Those are used to specify the gamemode. Available gamemodes at this point are `streaks`, `classic` and `battle-royale`.

These subcommands have their own suboptions that are specified as follows:

#### `/geo mode streaks`

##### `streaktype`:
* Optional, defaults to `CountryStreak`
* Available `streaktype`s are `CountryStreak` and `UsStateStreak`

##### `moving`:
* Optional, defaults to `Moving is allowed`
* Specifies whether moving will be allowed in the generated challenge.
* Available options for `moving` are `Moving is allowed` and `Moving is not allowed`

##### `panning`:
* Optional, defaults to `Panning is allowed`
* Specifies whether panning will be allowed in the generated challenge
* Available options for `panning` are `Panning is allowed` and `Panning is not allowed`

##### `zooming`:
* Optional, defaults to `Zooming is allowed`
* Specifies whether zooming will be allowed in the generated challenge
* Available options for `zooming` are `Zooming is allowed` and `Zooming is not allowed`

##### `time`:
* Optional, defaults to `0` (No time limit)
* Specifies the per-round time limit for the generated challenge in seconds
* Has to be an integer between `0` and `(2^31)-1`

I don't know if the GeoGuessr-API actually imposes any limitations for the
time-limit, but in my code the value for it is stored in an `i32` and therefore it can't be greated than `2147483647` ~ 24855 days

#### `/geo mode classic`

##### `map`:
* Required
* Specifies the name of the map for the generated challenge.

Internally the bot queries the GeoGuessr-API and chooses the 1st result. The results are the same that show up
while using the search function on the GeoGuessr website.

##### `moving`:
* Optional, same as in `/geo mode streaks`

##### `panning`:
* Optional, same as in `/geo mode streaks`

##### `zooming`:
* Optional, same as in `/geo mode streaks`

##### `time`:
* Optional, same as in `/geo mode streaks`

#### `/geo mode battle-royale`

##### `lobby`:
* Optional
* Specifies the url to the the `lobby` that's game should be started.

If no `lobby` is specified all the other options will be ignored and a lobby will be created.
The url for the created lobby will then be returned as an answer to the slash-command.

##### `gametype`:
* Optional, defaults to `Battle-Royale Countries`
* Specifies the type of the game that should be started.
* Available options are `Battle-Royale Countries` and `Battle-Royale Distance`.
* Will be ignored if no `lobby` is specified.

##### `moving`:
* Optional, same as in `/geo mode streaks`
* Will be ignored if no `lobby` is specified

##### `panning`:
* Optional, same as in `/geo mode streaks`
* Will be ignored if no `lobby` is specified

##### `zooming`:
* Optional, same as in `/geo mode streaks`
* Will be ignored if no `lobby` is specified

##### `time`:
* Optional, same as in `/geo mode streaks`
* Will be ignored if no `lobby` is specified

##### `spy`:
* Optional, defaults to `The Spy-powerup is available`
* Specifies whether the Spy-powerup will be available in the started match
* Available options are `The Spy-powerup is available` and `The Spy-powerup is not available`
* Will be ignored if no `lobby` is specified

##### `5050`:
* Optional, defaults to `The 5050-powerup is available`
* Specifies whether the 5050-powerup will be available in the started match
* Available options are `The 5050-powerup is available` and `The 5050-powerup is not available`
* Will be ignored if no `lobby` is specified
