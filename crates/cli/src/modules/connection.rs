use crate::error::{Error, Result};
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use crate::utils::qr::print_qr_code;
use agent::modules::connection::{
    ConnectionCreateInvitationOptions, ConnectionGetAllOptions, ConnectionModule,
    ConnectionReceiveInvitationOptions,
};
use clap::{Args, Subcommand};
use logger::{copy, pretty_stringify_obj};
use std::str;

/// Connection options and flags
#[derive(Args)]
pub struct ConnectionOptions {
    /// All the subcommands of the connection cli
    #[clap(subcommand)]
    pub commands: ConnectionSubcommands,
}

/// Connection subcommands
#[derive(Subcommand, Debug)]
#[clap(about = HelpStrings::Connections)]
pub enum ConnectionSubcommands {
    /// Create an invitation
    #[clap(about = HelpStrings::ConnectionsInvite)]
    Invite {
        /// Whether it should auto accept your side of the connection flow
        #[clap(long, short, help = HelpStrings::ConnectionsInviteAutoAccept)]
        auto_accept: bool,

        /// Whether it should print a qr code
        #[clap(long, short, help = HelpStrings::ConnectionsInviteQr)]
        qr: bool,

        /// Whether it should create a specific invitation for the toolbox
        /// this comes with the admin group
        #[clap(long, short, help = HelpStrings::ConnectionsInviteToolbox)]
        toolbox: bool,

        /// Whether the invitation should be reusable
        #[clap(long, short, help = HelpStrings::ConnectionsInviteMultiUse)]
        multi_use: bool,

        /// A custom alias for that specific connection
        #[clap(long, short = 'l', help = HelpStrings::ConnectionsInviteAlias)]
        alias: Option<String>,
    },

    /// Receive an invitation via url
    #[clap(about = HelpStrings::ConnectionsReceive)]
    Receive {
        /// Invitation url
        #[clap(long, short, help = HelpStrings::ConnectionsReceiveUrl)]
        url: String,
    },

    /// List all connections
    #[clap(about = HelpStrings::ConnectionsList)]
    List {
        /// Filter on connection id
        #[clap(long, short, help = HelpStrings::ConnectionsListId)]
        id: Option<String>,

        /// Filter on connection alias
        #[clap(long, short, help = HelpStrings::ConnectionsListAlias, conflicts_with = "id")]
        alias: Option<String>,

        /// Filter on connection protocol
        #[clap(long, short, help = HelpStrings::ConnectionsListConnectionProtocol, conflicts_with = "id")]
        connection_protocol: Option<String>,

        /// Filter on invitation key
        #[clap(long, short = 'k', help = HelpStrings::ConnectionsListInvitationKey, conflicts_with = "id")]
        invitation_key: Option<String>,

        /// Filter on your did
        #[clap(long, short, help = HelpStrings::ConnectionsListMyDid, conflicts_with = "id")]
        my_did: Option<String>,

        /// Filter on the state of the connection
        #[clap(long, short, help = HelpStrings::ConnectionsListState, conflicts_with = "id")]
        state: Option<String>,

        /// Filter on their did
        #[clap(long, short = 'd', help = HelpStrings::ConnectionsListTheirDid, conflicts_with = "id")]
        their_did: Option<String>,

        /// Filter on their role
        #[clap(long, short = 'r', help = HelpStrings::ConnectionsListTheirRole, conflicts_with = "id")]
        their_role: Option<String>,
    },
}

/// Subcommand connection parser
pub async fn parse_connection_args(
    options: &ConnectionOptions,
    agent: impl ConnectionModule,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());

    match &options.commands {
        ConnectionSubcommands::Invite {
            auto_accept,
            qr,
            toolbox,
            multi_use,
            alias,
        } => {
            let options = ConnectionCreateInvitationOptions {
                alias: alias.as_deref().map(|a| a.to_string()),
                auto_accept: *auto_accept,
                multi_use: *multi_use,
                qr: *qr,
                toolbox: *toolbox,
            };
            agent.create_invitation(options).await.map(|response| {
                loader.stop();
                log_info!("Created invite with connection id:");
                log!("{}", response.id);
                if *qr {
                    log_info!("Scan this QR code to accept the invitation:\n");
                    print_qr_code(&response.invitation_url).unwrap();
                } else {
                    log_info!("Another agent can use this URL to accept your invitation:");
                    log!("{}", &response.invitation_url);
                }
                copy!("{}", response.invitation_url);
            })
        }
        ConnectionSubcommands::Receive { url } => {
            let invitation = invite_url_to_struct(url.to_owned())?;
            agent
                .receive_invitation(invitation)
                .await
                .map(|connection| {
                    log_debug!("{}", pretty_stringify_obj(&connection));
                    log_info!("Fetched connection id:");
                    log!("{}", connection.id);
                })
        }
        ConnectionSubcommands::List {
            id,
            alias,
            their_role,
            connection_protocol,
            invitation_key,
            my_did,
            state,
            their_did,
        } => match id {
            Some(i) => agent.get_by_id(i.to_owned()).await.map(|connection| {
                loader.stop();
                copy!("{}", pretty_stringify_obj(&connection));
                log!("{}", pretty_stringify_obj(connection))
            }),
            None => {
                let options = ConnectionGetAllOptions {
                    alias: alias.as_deref().map(|a| a.to_string()),
                    their_did: their_did.as_deref().map(|t| t.to_string()),
                    state: state.as_deref().map(|s| s.to_string()),
                    my_did: my_did.as_deref().map(|m| m.to_string()),
                    invitation_key: invitation_key.as_deref().map(|i| i.to_string()),
                    connection_protocol: connection_protocol.as_deref().map(|c| c.to_string()),
                    their_role: their_role.as_deref().map(|t| t.to_string()),
                };
                agent.get_all(options).await.map(|connections| {
                    loader.stop();
                    copy!("{}", pretty_stringify_obj(&connections));
                    log!("{}", pretty_stringify_obj(connections))
                })
            }
        },
    }
}

/// Create an invitation struct from an invitation url
pub fn invite_url_to_struct(url: String) -> Result<ConnectionReceiveInvitationOptions> {
    // Split the url
    let split_url = url
        .split("c_i=")
        .map(|u| u.to_owned())
        .collect::<Vec<String>>();

    // Get the query parameters
    let query_parameters = split_url
        .get(1)
        .ok_or(Error::InvalidAgentInvitation)?
        .split('&')
        .map(|u| u.to_owned())
        .collect::<Vec<String>>();

    let serialized_invitation = query_parameters
        .get(0)
        .ok_or(Error::InvalidAgentInvitation)?;

    // Base64 decode the invitation to a Vec<u8>
    let decoded =
        base64::decode(serialized_invitation).map_err(|_| Error::InvalidAgentInvitation)?;

    // Convert the vec to a valid string
    let decoded_str = str::from_utf8(&decoded)?;

    // Convert the string to an invitation object
    serde_json::from_str(decoded_str).map_err(|e| e.into())
}
