use crate::cli::{Cli, Commands};
use crate::error::{Error, Result};
use crate::modules::automation::parse_automation_args;
use crate::modules::basic_message::parse_basic_message_args;
use crate::modules::configuration::parse_configuration_args;
use crate::modules::connection::parse_connection_args;
use crate::modules::credential::parse_credentials_args;
use crate::modules::credential_definition::parse_credential_definition_args;
use crate::modules::feature::parse_features_args;
use crate::modules::multitenancy::parse_multitenancy_args;
use crate::modules::proof::parse_proof_args;
use crate::modules::schema::parse_schema_args;
use crate::utils::config::{get_config_from_path, get_config_path};
use afj_rest::agent::{CloudAgentAfjRest, CloudAgentAfjRestVersion};
use clap::Parser;
use cloudagent_python::agent::{CloudAgentPython, CloudAgentPythonVersion};
use logger::LogLevel;
use std::path::PathBuf;

/// Register the subcommands on the cli
pub async fn register() -> Result<()> {
    let cli = Cli::parse();
    let level = if cli.quiet {
        LogLevel::Off
    } else {
        match cli.verbose {
            1 => LogLevel::Info,
            2 => LogLevel::Debug,
            3.. => LogLevel::Trace,
            _ => LogLevel::Warn,
        }
    };
    logger::init(level, cli.copy);

    log_trace!("Parsed CLI options and initialized logger");

    // Commands where the agent is not required
    match &cli.commands {
        Commands::Configuration(options) => parse_configuration_args(options).await,
        _ => {
            let (agent_url, api_key, auth_token, agent) = transform_agent_data(
                cli.agent,
                cli.config,
                cli.environment,
                cli.agent_url,
                cli.api_key,
                cli.token,
            )?;

            log_debug!(
                "Loading agent with the following config:\n- url: {}\n- api_key: {:?}\n- agent type: {}",
                agent_url,
                api_key,
                agent
            );

            // TODO: Ideally we would get an Agent<A> here that we would pass into the commands
            match agent.as_str() {
                "aca-py" => {
                    let version = CloudAgentPythonVersion::ZeroSevenThree;
                    let agent = CloudAgentPython::new(agent_url, version, api_key, auth_token);
                    // Commands that require the agent
                    match &cli.commands {
                        Commands::Schema(options) => parse_schema_args(options, agent).await,
                        Commands::Feature(_) => parse_features_args(agent).await,
                        Commands::Message(options) => {
                            parse_basic_message_args(options, agent).await
                        }
                        Commands::CredentialDefinition(options) => {
                            parse_credential_definition_args(options, agent).await
                        }
                        Commands::Connection(options) => {
                            parse_connection_args(options, agent).await
                        }
                        Commands::Credential(options) => {
                            parse_credentials_args(&options.commands, agent).await
                        }
                        Commands::Proof(options) => {
                            parse_proof_args(&options.commands, agent).await
                        }
                        Commands::Multitenancy(options) => {
                            parse_multitenancy_args(options, agent).await
                        }
                        Commands::Automate(options) => parse_automation_args(options, agent).await,
                        _ => Err(Error::SubcommandNotRegisteredForAgent(
                            cli.commands.into(),
                            "aca-py",
                        )
                        .into()),
                    }
                }
                "afj" => {
                    let version = CloudAgentAfjRestVersion::ZeroEightZero;
                    let agent = CloudAgentAfjRest::new(agent_url, version, api_key, auth_token);
                    match &cli.commands {
                        // TODO: should accept struct that has a field that implements the module
                        Commands::Schema(options) => parse_schema_args(options, agent).await,
                        Commands::CredentialDefinition(options) => {
                            parse_credential_definition_args(options, agent).await
                        }
                        Commands::Connection(options) => {
                            parse_connection_args(options, agent).await
                        }
                        Commands::Message(options) => {
                            parse_basic_message_args(options, agent).await
                        }
                        _ => Err(Error::SubcommandNotRegisteredForAgent(
                            cli.commands.into(),
                            "afj",
                        )
                        .into()),
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

/// Initialize any agent from the cli
fn transform_agent_data(
    agent: Option<String>,
    config: Option<PathBuf>,
    environment: String,
    agent_url: Option<String>,
    api_key: Option<String>,
    auth_token: Option<String>,
) -> Result<(String, Option<String>, Option<String>, String)> {
    let config_path = match config {
        Some(c) => Some(c),
        None => {
            let config = get_config_path();
            match config {
                Ok(c) => {
                    if c.exists() {
                        Some(c)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
    };

    let (agent_url, api_key, auth_token, agent) = match config_path {
        Some(cp) => {
            let configurations = get_config_from_path(&cp)?;
            let configuration = configurations
                .configurations
                .get_key_value(&environment)
                .ok_or(Error::InvalidEnvironment(environment))?;
            let agent_url = agent_url.unwrap_or_else(|| configuration.1.endpoint.to_owned());
            let api_key = api_key.or_else(|| configuration.1.api_key.to_owned());
            let auth_token = auth_token.or_else(|| configuration.1.auth_token.to_owned());
            let agent = agent.or_else(|| configuration.1.agent.to_owned());
            (agent_url, api_key, auth_token, agent)
        }
        None => {
            let agent_url = agent_url.ok_or(Error::NoAgentURLSupplied)?;
            (agent_url, api_key, auth_token, agent)
        }
    };

    let agent = agent.or_else(|| Some(String::from("aca-py")));

    match agent {
        Some(a) => {
            if a != *"aca-py" && a != *"afj" {
                return Err(Error::InvalidAgent(a).into());
            }
            Ok((agent_url, api_key, auth_token, a))
        }
        None => unreachable!(),
    }
}
