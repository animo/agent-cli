use crate::agent::CloudAgentPython;
use crate::fill_query;
use agent::error::Result;
use agent::modules::connection::{
    Connection, ConnectionCreateInvitationOptions, ConnectionGetAllOptions, ConnectionModule,
    ConnectionReceiveInvitationOptions, Invitation,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Response from the server when all connections are requested
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionGetAllResponse {
    /// List of all the connections returned by the cloudagent
    /// these connections are already filtered on
    pub results: Vec<Connection>,
}

#[async_trait]
impl ConnectionModule for CloudAgentPython {
    async fn get_all(&self, options: ConnectionGetAllOptions) -> Result<Vec<Connection>> {
        let url = self.create_url(vec!["connections"])?;

        let query = fill_query!(
            options,
            alias,
            connection_protocol,
            invitation_key,
            my_did,
            state,
            their_did,
            their_role
        );

        let connections: ConnectionGetAllResponse = self.get(url, Some(query)).await?;

        Ok(connections.results)
    }

    async fn get_by_id(&self, id: String) -> Result<Connection> {
        let url = self.create_url(vec!["connections", &id])?;
        self.get::<Connection>(url, None).await
    }

    async fn create_invitation(
        &self,
        options: ConnectionCreateInvitationOptions,
    ) -> Result<Invitation> {
        let url = self.create_url(vec!["connections", "create-invitation"])?;
        let mut query: Vec<(&str, String)> = vec![];
        let mut body = None;
        if options.toolbox {
            query.push(("multi_use", false.to_string()));
            query.push(("auto_accept", true.to_string()));
            query.push(("alias", String::from("toolbox")));

            body = Some(json!({
                "metadata": {
                    "group": "admin"
                }
            }));
        } else {
            if options.multi_use {
                query.push(("multi_use", true.to_string()));
            }
            if options.auto_accept {
                query.push(("auto_accept", true.to_string()))
            }
            if let Some(alias) = &options.alias {
                query.push(("alias", alias.to_string()));
            }
        }
        self.post::<Invitation>(url, Some(query), body).await
    }
    async fn receive_invitation(
        &self,
        invitation: ConnectionReceiveInvitationOptions,
    ) -> Result<Connection> {
        let url = self.create_url(vec!["connections", "receive-invitation"])?;

        self.post(url, None, Some(serde_json::to_value(invitation)?))
            .await
    }
}
