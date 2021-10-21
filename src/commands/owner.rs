use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::ShardManagerContainer;

#[command]
#[owners_only]
pub async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.write().await;
    //let driver = data.get_mut::<WebDriverContainer>().unwrap();
    //driver.quit().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "Shutting down").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "There was a problem getting the shard manager")
            .await?;

        return Ok(());
    }

    Ok(())
}

