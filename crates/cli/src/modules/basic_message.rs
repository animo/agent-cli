use crate::error::Result;
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use agent::modules::basic_message::{BasicMessageModule, SendBasicMessageOptions};
use clap::Args;

/// Basic Message options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Message)]
pub struct BasicMessageOptions {
    /// The connection id to which to send the connectoon to
    #[clap(short, long, help=HelpStrings::MessageId)]
    connection_id: String,

    /// The message that should be send to the connection id
    #[clap(short, long, help=HelpStrings::MessageMessage)]
    message: String,
}

/// Subcommand Basic Message parser
pub async fn parse_basic_message_args(
    options: &BasicMessageOptions,
    agent: impl BasicMessageModule,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());
    let send_options = SendBasicMessageOptions {
        connection_id: options.connection_id.to_owned(),
        message: options.message.to_owned(),
    };
    agent.send_message(send_options).await.map(|_| {
        loader.stop();
        log!("Successfully sent message")
    })
}
