use crate::agent::CloudAgentAfjRest;
use agent::error::Result;
use agent::modules::basic_message::{
    BasicMessageModule, SendBasicMessageOptions, SendBasicMessageResponse,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl BasicMessageModule for CloudAgentAfjRest {
    async fn send_message(
        &self,
        options: SendBasicMessageOptions,
    ) -> Result<SendBasicMessageResponse> {
        let url = self.create_url(vec!["basic-messages", &options.connection_id])?;

        let body = json!({
          "content": options.message,
        });

        self.post(url, None, Some(body)).await
    }
}
