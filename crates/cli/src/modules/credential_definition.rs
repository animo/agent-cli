use crate::error::Result;
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use agent::modules::credential_definition::{
    CredentialDefinitionCreateOptions, CredentialDefinitionModule,
};
use clap::{Args, Subcommand};
use serde_json::json;

use logger::pretty_stringify_obj;

/// Credential Definition options and flags
#[derive(Args)]
#[clap(about = "Retrieve or create credential definitions")]
pub struct CredentialDefinitionOptions {
    /// All the subcommands of the credential definition cli
    #[clap(subcommand)]
    pub commands: CredentialDefinitionSubcommands,
}

/// Credential Definition subcommands
#[derive(Subcommand, Debug)]
pub enum CredentialDefinitionSubcommands {
    /// Create and register a credential definition based on the schema id
    #[clap(about = HelpStrings::CredentialDefinitionCreate)]
    Create {
        /// The schema id of which to base the credential definition on
        #[clap(short, long, help = HelpStrings::CredentialDefinitionCreateSchemaId)]
        schema_id: String,

        /// The tag used for registering the credential definition
        #[clap(short, long, help = HelpStrings::CredentialDefinitionCreateTag, default_value = "default")]
        tag: String,

        /// Whether the credential definition supports revocation
        #[clap(short = 'r', long, help = HelpStrings::CredentialDefinitionCreateSupportRevocation)]
        support_revocation: bool,

        /// The revocation registry size
        #[clap(short = 'v', long, help = HelpStrings::CredentialDefinitionCreateRevocationRegistrySize)]
        revocation_registry_size: Option<i32>,
    },

    /// List all the credential definitions
    #[clap(about = HelpStrings::CredentialDefinitionList)]
    List {
        /// List a single credential definition by id
        #[clap(long, short, help = HelpStrings::CredentialDefinitionId)]
        id: Option<String>,
    },
}

/// Subcommand Credential Definition parser
pub async fn parse_credential_definition_args(
    options: &CredentialDefinitionOptions,
    agent: impl CredentialDefinitionModule,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());

    match &options.commands {
        CredentialDefinitionSubcommands::Create {
            schema_id,
            support_revocation,
            tag,
            revocation_registry_size,
        } => {
            let options = CredentialDefinitionCreateOptions {
                schema_id: schema_id.to_string(),
                support_revocation: *support_revocation,
                tag: tag.to_string(),
                revocation_registry_size: *revocation_registry_size,
            };
            agent.create(options).await.map(|cred_def| {
                loader.stop();
                copy!("{}", cred_def.credential_definition_id);
                log_info!("Created credential definition with id:");
                log!("{}", cred_def.credential_definition_id);
            })
        }
        CredentialDefinitionSubcommands::List { id } => match id {
            Some(i) => agent.get_by_id(i.to_owned()).await.map(|cred_def| {
                loader.stop();
                let loggable = json!({
                    "id": cred_def.id,
                    "schema_id": cred_def.schema_id,
                    "type": cred_def.type_field,
                    "tag": cred_def.tag,
                    "ver": cred_def.ver,
                });
                log_debug!("{}", pretty_stringify_obj(cred_def));
                copy!("{}", pretty_stringify_obj(&loggable));
                log!("{}", pretty_stringify_obj(loggable));
            }),

            None => agent.get_all().await.map(|cred_defs| {
                loader.stop();
                cred_defs
                    .credential_definition_ids
                    .iter()
                    .for_each(|x| log!("{}", x));
                log_info!("Successfully fetched credential definition IDs",);
            }),
        },
    }
}
