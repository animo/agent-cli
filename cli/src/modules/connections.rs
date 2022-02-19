use agent::modules::connections::{ConnectionCreateInvitationConfig, ConnectionModule};
use clap::{Args, Subcommand};

use crate::utils::logger::Log;

#[derive(Args)]
pub struct ConnectionOptions {
    #[clap(subcommand)]
    pub commands: ConnectionSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum ConnectionSubcommands {
    Invite {
        #[clap(long, short)]
        auto_accept: bool,
        #[clap(long, short)]
        qr: bool,
        #[clap(long, short)]
        toolbox: bool,
        #[clap(long, short)]
        multi_use: bool,
        #[clap(long, short = 'l')]
        alias: Option<String>,
    },
}

// TODO: Can we just send a dereferenced struct directly?
pub async fn parse_connection_args(
    commands: &ConnectionSubcommands,
    agent: impl ConnectionModule,
    logger: Log,
) {
    match commands {
        ConnectionSubcommands::Invite {
            auto_accept,
            qr,
            toolbox,
            multi_use,
            alias,
        } => {
            let config = ConnectionCreateInvitationConfig {
                alias: alias.as_deref().map(|a| a.to_string()),
                auto_accept: *auto_accept,
                multi_use: *multi_use,
                qr: *qr,
                toolbox: *toolbox,
            };
            match agent.create_invitation(config).await {
                Ok(invite_url) => logger.log(invite_url),
                Err(e) => logger.error(format!("{:?}", e.to_string())),
            }
        }
    }
}