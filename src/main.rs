mod commands;
mod schemas;

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use shuttle_secrets::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use mongodb::{Client, Database};

struct Data {
    database_client: Client
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn connect_to_mongodb(uri: &str, database_name: &str) -> Result<(Client, Database), Error> {
    let client = Client::with_uri_str(uri).await?;
    let database = client.database(database_name);

    Ok((client, database))
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttleSerenity {

    // Fetch env secrets
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let mongo_uri = secret_store
        .get("MONGO_URI")
        .context("'MONGO_URI' was not found")?;

    let mongo_database = secret_store
        .get("MONGO_DATABASE")
        .context("'MONGO_DATABASE' was not found")?;

    // Connect to MongoDB and create the client
    let (db_client, _db) = connect_to_mongodb(&mongo_uri, &mongo_database).await
        .expect("Failed to connect to MongoDB");

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let command_vec = vec![
        commands::misc::ping::ping(),
        commands::misc::age::age(),
        commands::misc::help::help(),
    ];

    async fn pre_command_handler(ctx: Context<'_>) {
        println!("Executing command {} for user {}", ctx.command().qualified_name, ctx.author().name);

        // TODO: check if the user is in the database
        //       if the user is not in the database, add them to the database
        //       use schemas::user::User::new() to create a new user



    }

    async fn post_command_handler(ctx: Context<'_>) {
        println!("Executed command {} for user {}", ctx.command().qualified_name, ctx.author().name);
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: command_vec,
            event_handler: |_ctx, event, framework, _data| {
                Box::pin(event_handler(event, framework))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("🦀".into()),
                ..Default::default()
            },
            pre_command: |ctx| {
                Box::pin(pre_command_handler(ctx))
            },
            post_command: |ctx| {
                Box::pin(post_command_handler(ctx))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { database_client: db_client })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}

async fn event_handler(
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {} with ID {}", data_about_bot.user.name, data_about_bot.user.id);
        }
        serenity::FullEvent::Message { new_message } => {

            // skip bots because bots kinda silly tbh
            if new_message.author.bot {
                return Ok(());
            }

        }
        _ => {}
    }
    Ok(())
}