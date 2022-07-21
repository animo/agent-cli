use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Options provided when registering a credential definition on the ledger
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CredentialDefinitionCreateOptions {
    /// Schema id that the credential definition will be linked to
    pub schema_id: String,

    /// Optional tag used for the credential
    /// If none is supplied `default` will be used
    pub tag: String,

    /// Whether the credential definition supports revocation
    pub support_revocation: bool,

    /// The size of the revocation registry, how many credentials
    /// fit in the revocation registry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_registry_size: Option<i32>,
}

/// Response from the cloudagent when creating a credential definition
#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialDefinitionCreateResponse {
    /// Id of the credential definition
    #[serde(alias = "id")]
    pub credential_definition_id: String,
}

/// Response from the cloudagent when requesting a credential definition
/// by id
#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDefinitionGetByIdResponse {
    /// The credential definition requested
    pub credential_definition: CredentialDefinition,
}

/// A credential definition structure
#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDefinition {
    /// Version of the credential definition
    pub ver: String,

    /// Id of the credential definition
    pub id: String,

    /// Schema id on which the credential definintion is based
    #[serde(alias = "schemaId")]
    pub schema_id: String,

    /// The type of the credential definition
    #[serde(rename = "type")]
    pub type_field: String,

    /// Tag used by the credential definition
    /// default is `default`
    pub tag: String,

    /// TODO
    pub value: Value,
}

/// Response from the cloudagent when requesting all the credential definitions
#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDefinitionGetAllResponse {
    /// List of credential definitions registered by the cloudagent
    pub credential_definition_ids: Vec<String>,
}

/// Generic cloudagent credential definition module
#[async_trait]
pub trait CredentialDefinitionModule {
    /// Register a credential definition on the ledger
    async fn create(
        &self,
        options: CredentialDefinitionCreateOptions,
    ) -> Result<CredentialDefinitionCreateResponse>;

    /// Get the registered credential definition by id
    async fn get_by_id(&self, id: String) -> Result<CredentialDefinition>;

    /// Get all the registered credential definitions
    async fn get_all(&self) -> Result<CredentialDefinitionGetAllResponse>;
}
