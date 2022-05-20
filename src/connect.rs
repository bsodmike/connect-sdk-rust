//! Connect API

use crate::{
    client::Client,
    error::Error,
    models::{Item, Vault, VaultData},
};
use dotenv::dotenv;

pub struct Connect {
    server_url: String,
    token: String,
    client: Client,
    vault: Vault,
    item: Item,
}

impl Connect {
    pub fn new() -> Self {
        let token = std::env::var("OP_API_TOKEN").expect("1Password API token expected!");
        let host = std::env::var("OP_SERVER_URL").expect("1Password Connect server URL expected!");

        // .env to override settings in ENV
        dotenv().ok();
        let client = Client::new(&token, &host);

        Self {
            server_url: host.clone(),
            token: token.clone(),
            client,
            vault: Vault::new(Client::new(&token, &host)),
            item: Item::new(Client::new(&token, &host)),
        }
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }

    pub(crate) fn vault(&self) -> &Vault {
        &self.vault
    }

    pub(crate) fn item(&self) -> &Item {
        &self.item
    }

    pub async fn list_vaults(&self) -> Result<(Vec<VaultData>, serde_json::Value), Error> {
        let result = self.vault.get_list().await?;

        Ok(result)
    }
}
