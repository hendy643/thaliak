use serenity_commands::macros::{Command};
use serenity_commands::serenity::client::Context;
use serenity_commands::serenity::model::interactions::InteractionResponseType;
use serenity_commands::serenity::model::prelude::application_command::ApplicationCommandInteraction;

/// Play a little game called Ping Pong!
#[derive(Debug, Command)]
#[command(name = "ping")]
pub(crate) struct Ping {}

impl Ping {
    pub async fn respond(interaction: ApplicationCommandInteraction, ctx: Context) -> serenity_commands::serenity::Result<()> {
        interaction
            .create_interaction_response(&ctx, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|m| m.content(format!("Pong!")))
            })
            .await
    }
}