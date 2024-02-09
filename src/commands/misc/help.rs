use poise::{builtins::HelpConfiguration};
use crate::{Context, Error};


/// Replies with a list of all available commands or help for a specific command.
///
/// If no command is specified, it will list all available commands. Otherwise, it will show help for the specified
/// command, including usage, a description, any subcommands and arguments it takes in.
#[poise::command(prefix_command, slash_command, track_edits, category = "Utility")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[rest]
    mut command: Option<String>,
) -> Result<(), Error> {
    if ctx.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!("{} {}", ctx.invoked_command_name(), c)),
            None => Some(ctx.invoked_command_name().to_string()),
        };
    }
    let extra_text_at_bottom = "You can also use `</help:1205612973264212001> <command>` to get help for a specific command.";

    let config = HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        extra_text_at_bottom,

        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}