use crate::agent::CloudAgentAfjRest;
use agent::error::{Error, Result};
use agent::modules::credential_definition::{
    CredentialDefinition, CredentialDefinitionCreateOptions, CredentialDefinitionCreateResponse,
    CredentialDefinitionGetAllResponse, CredentialDefinitionModule,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl CredentialDefinitionModule for CloudAgentAfjRest {
    async fn create(
        &self,
        options: CredentialDefinitionCreateOptions,
    ) -> Result<CredentialDefinitionCreateResponse> {
        let url = self.create_url(vec!["credential-definitions"])?;

        let body = json!({
            "tag": options.tag,
            "supportRevocation": options.support_revocation,
            "schemaId": options.schema_id
        });

        self.post(url, None, Some(body)).await
    }

    async fn get_by_id(&self, id: String) -> Result<CredentialDefinition> {
        let url = self.create_url(vec!["credential-definitions", &id])?;
        self.get(url, None).await
    }

    async fn get_all(&self) -> Result<CredentialDefinitionGetAllResponse> {
        Err(Error::CommandNotAvailable(format!("{}", self)).into())
    }
}
