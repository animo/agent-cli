use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Query filters applied to retrieving all the connections
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConnectionGetAllOptions {
    /// Optional `alias` to filter on
    pub alias: Option<String>,

    // TODO: enum
    /// Optional `connection protocol` to filter on
    pub connection_protocol: Option<String>,

    /// Optional `invitation key` to filter on
    pub invitation_key: Option<String>,

    /// Optional `my did` to filter on
    pub my_did: Option<String>,

    // TODO: enum
    /// Optional `state` to filter on
    pub state: Option<String>,

    /// Optional `their did` to filter on
    pub their_did: Option<String>,

    // TODO: enum
    /// Optional `their role` to filter on
    pub their_role: Option<String>,
}

/// Create invitation response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invitation {
    /// Invitation url
    #[serde(alias = "invitationUrl")]
    pub invitation_url: String,
    /// Invitation structure
    pub invitation: Value,

    /// Connection id of the invitation
    #[serde(alias = "connection_id")]
    pub id: String,
}

/// A single connection structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    /// The connection id used for further functionality
    #[serde(alias = "connection_id")]
    pub id: String,

    /// When the connection is created
    #[serde(alias = "createdAt")]
    pub created_at: String,

    /// Your did used in the connection process
    #[serde(alias = "my_did")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,

    /// The current connection state
    pub state: String,

    /// Their role in the connection process
    #[serde(alias = "their_role")]
    pub role: String,

    /// Auto accept state
    #[serde(alias = "accept", alias = "autoAcceptConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept: Option<Value>,

    /// Their label as set when the connection is initialized
    #[serde(alias = "theirLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub their_label: Option<String>,

    /// Optional their did used in the connection proces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub their_did: Option<String>,

    /// (AFJ) verkey used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verkey: Option<String>,
}

/// Options supplied by the frontend for creating an invitation
#[derive(Debug, Default)]
pub struct ConnectionCreateInvitationOptions {
    /// Whether we want to auto accept the connection process
    pub auto_accept: bool,

    /// Whether a QR should be outputted to the user
    pub qr: bool,

    /// Whether it should create a special invitation for the toolbox
    pub toolbox: bool,

    /// Whether the invitation is reuseable
    pub multi_use: bool,

    /// Optional custom alias for the connection
    pub alias: Option<String>,
}

/// Options for receiving an invitation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// Every field is optional here as there are some collisions between, for example, did and
// routing_key. We rely on the cloudagent for error handling these collisions
// Sadly we cannot skip serializing on the whole struct, we must specify it for each element
pub struct ConnectionReceiveInvitationOptions {
    /// Id of the connection
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Did used for the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,

    /// Url to an image that can be used as an avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,

    /// Label used for the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Keys used by the recipient
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_keys: Option<Vec<String>>,

    /// Routing keys used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_keys: Option<Vec<String>>,

    /// Service endpoint to call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
}

/// Generic cloudagent connection module
#[async_trait]
pub trait ConnectionModule {
    /// Gets all the connections
    async fn get_all(&self, options: ConnectionGetAllOptions) -> Result<Vec<Connection>>;

    /// Get a connection by id
    async fn get_by_id(&self, id: String) -> Result<Connection>;

    /// Create an invitation
    async fn create_invitation(
        &self,
        options: ConnectionCreateInvitationOptions,
    ) -> Result<Invitation>;

    /// Receive and accept a connection
    async fn receive_invitation(
        &self,
        invitation: ConnectionReceiveInvitationOptions,
    ) -> Result<Connection>;
}
