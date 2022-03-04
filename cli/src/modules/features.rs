use agent_controller::modules::features::FeaturesModule;
use clap::Args;

use crate::error::Result;
// use crate::utils::logger::Log;

#[derive(Args)]
pub struct FeaturesOptions {}

pub async fn parse_features_args(agent: impl FeaturesModule) -> Result<()> {
    agent.discover_features().await.map(|features| {
        // if logger.debug {
        //     logger.log_pretty(features)
        // } else {
        features
            .results
            .keys()
            .collect::<Vec<_>>()
            .iter()
            .for_each(|x| println!("{}", x));
        // }
    })
}
