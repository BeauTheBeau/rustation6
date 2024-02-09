use crate::{Context, Error};
use anyhow::Result;
use poise::serenity_prelude::User;

/// Replies with the age of your or another user's account.
#[poise::command(slash_command, category = "Utility")]
pub async fn age(
    ctx: Context<'_>,
    #[description = "The user to get the age of. Defaults to yourself."] user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let age = u.created_at();
    let age_epoch = age.timestamp();

    let message = if u.id == ctx.author().id {
        format!("Your account was created <t:{}:R> (<t:{}:F>)", age_epoch, age_epoch)
    } else {
        format!("**{}**'s account was created <t:{}:R> (<t:{}:F>)", u.name, age_epoch, age_epoch)
    };

    ctx.reply(message).await?;

    Ok(())
}