use crate::agent::CloudAgentAfjRest;
use agent::error::Result;
use agent::modules::connection::{
    Connection, ConnectionCreateInvitationOptions, ConnectionGetAllOptions, ConnectionModule,
    ConnectionReceiveInvitationOptions, Invitation,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Create invitation response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Invitation url
    pub invitation_url: String,
    /// Invitation
    pub invitation: Value,
    /// Connection record
    pub connection: Connection,
}

#[async_trait]
impl ConnectionModule for CloudAgentAfjRest {
    async fn get_all(&self, _options: ConnectionGetAllOptions) -> Result<Vec<Connection>> {
        log_debug!("Supplied options are not being used here");
        let url = self.create_url(vec!["connections"])?;

        self.get(url, None).await
    }

    async fn get_by_id(&self, id: String) -> Result<Connection> {
        let url = self.create_url(vec!["connections", &id])?;

        self.get(url, None).await
    }

    async fn create_invitation(
        &self,
        options: ConnectionCreateInvitationOptions,
    ) -> Result<Invitation> {
        let url = self.create_url(vec!["connections", "create-invitation"])?;

        let body = json!({
            "autoAcceptConnections": options.auto_accept,
            "alias": options.alias,
        });

        let invitation: Response = self.post(url, None, Some(body)).await?;

        Ok(Invitation {
            invitation_url: invitation.invitation_url,
            invitation: invitation.invitation,
            id: invitation.connection.id,
        })
    }

    async fn receive_invitation(
        &self,
        invitation: ConnectionReceiveInvitationOptions,
    ) -> Result<Connection> {
        let url = self.create_url(vec!["connections", "receive-invitation"])?;

        let body = json!({
            "invitation": {
                "label": invitation.label,
                "did": invitation.did,
                "recipientKeys": invitation.recipient_keys,
                "routingKeys": invitation.routing_keys,
                "serviceEndpoint": invitation.service_endpoint,
            },
        });

        self.post(url, None, Some(body)).await
    }
}
