mod commands;

use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_secrets::SecretStore;
use shuttle_serenity::ShuttleSerenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttleSerenity {

    // Get DISCORD_TOKEN from `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let command_vec = vec![
        commands::misc::ping::ping(),
        commands::misc::age::age(),
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: command_vec,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
