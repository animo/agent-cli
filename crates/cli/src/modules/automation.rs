use crate::error::{Error, Result};
use crate::help_strings::HelpStrings;
use crate::modules::connection::invite_url_to_struct;
use crate::utils::loader::{Loader, LoaderVariant};
use crate::utils::qr;
use agent::modules::connection::{ConnectionCreateInvitationOptions, ConnectionModule};
use agent::modules::credential::CredentialModule;
use agent::modules::credential_definition::CredentialDefinitionModule;
use agent::modules::schema::SchemaModule;
use automations::automations::{
    create_credential_definition::CreateCredentialDefinition,
    credential_offer::CredentialOfferAutomation,
};
use clap::{Args, Subcommand};
use colored::*;
use std::collections::HashMap;

/// Automation options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Automation)]
pub struct AutomationOptions {
    /// All the subcommands of the automation cli
    #[clap(subcommand)]
    pub commands: AutomationSubcommands,
}

/// Automation subcommands
#[derive(Subcommand, Debug)]
pub enum AutomationSubcommands {
    /// Credential offer subcommand which automatically offers a credential
    #[clap(about = HelpStrings::AutomationCredentialOffer )]
    CredentialOffer {
        /// Connection id to send the credential to
        #[clap(long, short, help = HelpStrings::AutomationCredentialOfferConnectionId)]
        connection_id: Option<String>,

        /// Maximum amount of time it should wait for an established connection
        #[clap(long, short, default_value = "60", help = HelpStrings::AutomationCredentialOfferTimeout)]
        timeout: u32,

        /// Whether it should send the credential to yourself
        #[clap(long = "self", short = 's', help = HelpStrings::AutomationCredentialOfferSelf)]
        sent_to_self: bool,

        /// Whether no qr code should be printed out
        #[clap(long, short, help = HelpStrings::AutomationCredentialOfferNoQr )]
        no_qr: bool,
    },

    /// Create a credential definition subcommand
    #[clap(about = HelpStrings::AutomationCreateCredentialDefinition )]
    CreateCredentialDefinition {
        /// Name of the schema that the credential definition will be based on
        #[clap(long, short='n', default_value="agent-cli-schema", help = HelpStrings::AutomationCreateCredentialDefinitionName)]
        name: String,

        /// Attributes of the schema the credential definition will be based on
        #[clap(long, short='a', multiple_occurrences(true), help = HelpStrings::AutomationCreateCredentialDefinitionAttributes)]
        attributes: Vec<String>,

        /// Version of the schema the credential definition will be based on
        #[clap(long = "version", short = 'v', default_value="1.0", help = HelpStrings::AutomationCreateCredentialDefinitionVersion)]
        version: String,
    },
}

/// Subcommand Automation parser
pub async fn parse_automation_args(
    options: &AutomationOptions,
    agent: impl ConnectionModule + CredentialModule + SchemaModule + CredentialDefinitionModule,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());

    match &options.commands {
        AutomationSubcommands::CredentialOffer {
            connection_id,
            timeout,
            sent_to_self,
            no_qr,
        } => match connection_id {
            Some(c) => credential_offer(c.to_owned(), agent).await?,
            None => {
                let connection = agent
                    .create_invitation(ConnectionCreateInvitationOptions {
                        auto_accept: true,
                        alias: Some(String::from("automation")),
                        ..Default::default()
                    })
                    .await?;
                if *sent_to_self {
                    let invitation_object = invite_url_to_struct(connection.invitation_url)?;
                    agent.receive_invitation(invitation_object).await?;
                } else {
                    if !no_qr {
                        log!("{} the QR code to accept the invitation", "Scan".bold(),);
                        qr::print_qr_code(&connection.invitation_url)?;
                    }
                    log!();
                    log!();
                    log!("================");
                    log!("{}", "Credential offer".bold());
                    log!("================");
                    log!();

                    log!(
                        "{} invitation with connection id {}.",
                        "Created".green(),
                        connection.id.bold()
                    );
                    log!();
                    log!("Use this URL:\n\n{}", connection.invitation_url);
                    log!();
                    log!();
                    log!(
                        "{} for the invitation to be accepted. Timeout is {} seconds...",
                        "Waiting".cyan(),
                        timeout
                    );
                    copy!("{}", connection.invitation_url);
                }
                log_debug!("Looping {} times", timeout);
                for i in 1..=*timeout {
                    let connection =
                        ConnectionModule::get_by_id(&agent, connection.id.to_owned()).await?;
                    if connection.state != "active" && connection.state != "response" {
                        log_trace!(
                            "Connection state is not active, waiting 1 second then trying again..."
                        );
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                    } else {
                        log!("Invitation {}!", "accepted".green());
                        credential_offer(connection.id, agent).await?;
                        break;
                    }
                    if i == *timeout {
                        return Err(Error::InactiveConnection.into());
                    }
                }
                log_info!("Successfully executed automation");
                log!("It might take a few seconds for the credential to arrive",);
                loader.stop();
            }
        },
        AutomationSubcommands::CreateCredentialDefinition {
            name,
            version,
            attributes,
        } => {
            let automation = CreateCredentialDefinition {
                name: name.to_owned(),
                version: version.to_owned(),
                attributes: attributes.to_owned(),
            };
            automation.execute(agent).await?;
        }
    };
    Ok(())
}

/// Building and offering the credential
async fn credential_offer(
    connection_id: String,
    agent: impl ConnectionModule + CredentialModule + SchemaModule + CredentialDefinitionModule,
) -> Result<()> {
    // Mock credential
    let mut attributes: HashMap<String, String> = HashMap::new();
    attributes.insert(String::from("Name"), String::from("Joyce Brown"));
    attributes.insert(String::from("Date Of Birth"), String::from("19890321"));
    attributes.insert(String::from("Street"), String::from("Main Road 207"));
    attributes.insert(String::from("City"), String::from("New York"));
    attributes.insert(String::from("Bank"), String::from("qBank New York"));
    attributes.insert(
        String::from("Card Number"),
        String::from("4537-6696-0666-0146"),
    );
    attributes.insert(String::from("Security Code"), String::from("063"));
    attributes.insert(String::from("Valid Until"), String::from("20251212"));
    log_debug!("Mock credential:\n{:#?}", attributes);

    let automation = CredentialOfferAutomation {
        connection_id,
        attributes,
    };

    automation.execute(agent).await
}
