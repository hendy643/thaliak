mod commands;

use std::env;
use serenity_commands::macros::{Commands};
use serenity_commands::serenity;
use serenity_commands::serenity::Client;
use serenity_commands::serenity::client::{Context, EventHandler};
use serenity_commands::serenity::model::gateway::Ready;
use serenity_commands::serenity::model::guild::Guild;
use serenity_commands::serenity::model::id::GuildId;
use serenity_commands::serenity::model::interactions::{Interaction};
use crate::bot::commands::lodestone::GetLodestoneProfile;

use self::commands::misc::Ping;

struct Handler;

pub struct DiscordBot {}

impl DiscordBot {
    pub fn new() -> DiscordBot {
        DiscordBot {}
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let token = env!("DISCORD_TOKEN");
        let application_id = env!("DISCORD_APP_ID");
        let application_id = application_id.parse::<u64>()?;

        let mut client =
            Client::builder(&token).event_handler(Handler).application_id(application_id).await?;

        client.start_autosharded().await?;

        Ok(())
    }
}

#[derive(Commands)]
enum Command {
    Ping(Ping),
    GetLodestoneProfile(GetLodestoneProfile),
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        Command::register_commands_in_guild(&ctx, guild.id).await.unwrap();
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        let guild_id = env!("OWNER_GUILD").parse::<u64>().unwrap();
        Command::register_commands_in_guild(&ctx, GuildId(guild_id)).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let interaction = match interaction {
            Interaction::ApplicationCommand(cmd) => cmd,
            _ => return,
        };

        let command = match Command::parse(interaction.clone()) {
            Ok(cmd) => cmd,
            Err(_) => return,
        };

        match command {
            Command::Ping(Ping {}) => { Ping::respond(interaction, ctx).await }
            Command::GetLodestoneProfile(GetLodestoneProfile { id }) => { GetLodestoneProfile::respond(interaction, ctx, id).await }
        }.expect("Error executing command");
    }
}
