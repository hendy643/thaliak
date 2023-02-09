use serenity_commands::macros::{Command};
use serenity_commands::serenity::builder::{CreateEmbed};
use serenity_commands::serenity::client::Context;
use serenity_commands::serenity::model::interactions::InteractionResponseType;
use serenity_commands::serenity::model::prelude::application_command::ApplicationCommandInteraction;

/// Get a characters lodestone profile.
#[derive(Debug, Command)]
#[command(name = "get_lodestone_profile")]
pub(crate) struct GetLodestoneProfile {
    /// The ID of the profile.
    #[option(integer)]
    pub id: i64,
}

impl GetLodestoneProfile {
    pub async fn respond(interaction: ApplicationCommandInteraction, ctx: Context, id: i64) -> serenity_commands::serenity::Result<()> {
        interaction.create_interaction_response(&ctx, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| {
                    let lds = crate::lodestone::Lodestone::new();
                    let profile = lds.get_profile(id as u64);
                    let prof = futures::executor::block_on(profile);
                    let mut embed = CreateEmbed::default();
                    embed.title(prof.name);
                    embed.url(prof.url);
                    embed.field("ID", prof.id, false);
                    m.add_embed(embed);
                    m.content("Lodestone Profile")
                })
        })
            .await
    }
}