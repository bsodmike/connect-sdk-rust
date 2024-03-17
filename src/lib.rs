#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![forbid(unsafe_code)]
#![deny(unstable_features)]
#![warn(rust_2018_idioms, future_incompatible, nonstandard_style)]

//! connect-1password is a Rust SDK for 1Password Connect.
//!
//! # High-level features
//!
//! - Based on [`tokio`], [`hyper`] and [`hyper_rustls`] by default.
//! - [`hyper`] can be replaced using the [`HTTPClient`](client::HTTPClient) interface.
//!
//! ## Details
//!
//! - To create a Login item, make sure to use the Trait [`LoginItem`](models::item::LoginItem), so as to be able to call respective methods (enforced by the interface) on [`ItemBuilder`](models::item::ItemBuilder).
//!
//!   ```
//!   # use connect_1password::{
//!   #    client::{Client, HTTPClient},
//!   #    models::{
//!   #        item::{LoginItem, FullItem, ItemBuilder, ItemCategory},
//!   #    },
//!   #    vaults,
//!   #    items,
//!   # };
//!   use connect_1password::error::ConnectResult;
//!
//!   async fn get_login_item(index: usize) ->  ConnectResult<FullItem> {
//!        let client = Client::default();
//!   
//!        let (vaults, _) = vaults::all(&client).await?;
//!        assert!(!vaults.is_empty());
//!   
//!        let item: FullItem = ItemBuilder::new(&vaults[index].id, ItemCategory::Login)
//!            .title("Secure server login")
//!            .username("Bob")
//!            .password("")
//!            .build()?;
//!   
//!        Ok(item)
//!   }
//!   #
//!   # fn main() {}
//!   ```
//!
//!
//! - This is ideally used for programmatic access, and potentially the main interface required for this entire API wrapper.
//!
//!   In the example below, since we have not provided a specific API key value, one is generated for us by the Connect API.
//!
//!   ```
//!   # use connect_1password::{
//!   #    client::{Client, HTTPClient},
//!   #    models::{
//!   #        item::{ApiCredentialItem, FullItem, ItemBuilder, ItemCategory, FieldObject},
//!   #    },
//!   #    vaults,
//!   #    items,
//!   # };
//!   use connect_1password::error::ConnectResult;
//!
//!   async fn get_api_credential(index: usize) ->  ConnectResult<FullItem> {
//!        let client = Client::default();
//!   
//!        let (vaults, _) = vaults::all(&client).await?;
//!        assert!(!vaults.is_empty());
//!   
//!        let item = ApiCredentialItem::build(
//!            &ItemBuilder::new(&vaults[0].id, ItemCategory::ApiCredential)
//!                .api_key("smelly-socks", "Dell XYZ"),
//!        )?;
//!   
//!        Ok(item)
//!   }
//!   #
//!   # fn main() {}
//!   ```
//!

//! # Examples
//!
//! Refer to `./examples`

pub mod client;
pub mod error;
pub mod items;
pub mod models;
pub mod vaults;

#[cfg(test)]
fn get_test_client() -> (client::Client, String) {
    use dotenv::dotenv;
    dotenv().ok();

    let test_vault_id =
        std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");

    (client::Client::default(), test_vault_id)
}
