use crate::{Context, Error};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;

/// Replies with the bot's current latency and round trip time in milliseconds.
#[poise::command(slash_command, category = "Utility")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = ctx.created_at().timestamp_millis() as u128;
    let now = get_current_millis()?;
    let diff = now - start;

    ctx.say(format!("Pong! Latency: {:.2}ms", diff)).await?;

    // then get the round trip time
    let end = get_current_millis()?;
    let rtt = end - start;
    ctx.say(format!("Round trip time: {:.2}ms", rtt)).await?;
    Ok(())
}

fn get_current_millis() -> Result<u128, Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?;

    Ok(now.as_millis())
}