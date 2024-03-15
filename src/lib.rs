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
//! # Examples
//!
//! ## Create a Login item
//!
//! To create a Login item, make sure to use the Trait [`LoginItem`](models::item::LoginItem), so as to be able to call
//! respective methods (enforced by the interface) on [`ItemBuilder`](models::item::ItemBuilder).
//!
//! ```
//! use connect_1password::{
//!     error::Error,
//!     client::{Client, HTTPClient},
//!     models::{
//!         item::{LoginItem, FullItem, ItemBuilder, ItemCategory},
//!     },
//!     vaults,
//!     items,
//! };
//!
//! const SLEEP_DELAY: u64 = 4; // seconds
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::default();
//!
//!     let (vaults, _) = vaults::all(&client).await?;
//!     assert!(!vaults.is_empty());
//!
//!     let item: FullItem = ItemBuilder::new(&vaults[0].id, ItemCategory::Login)
//!         .title("Secure server login")
//!         .username("Bob")
//!         .password("")
//!         .build()
//!         .unwrap();
//!
//!     let (new_item, _) = items::add(&client, item).await?;
//!     assert_eq!(new_item.title, "Secure server login");
//!
//!     // Just as a clean up measure, we remove the item created in the this example
//!     tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;
//!
//!     items::remove(&client, &vaults[0].id, &new_item.id)
//!         .await;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Create an API Credential item
//!
//! This is ideally used for programmatic access, and potentially the main interface required for
//! this entire API wrapper.
//!
//! In the example below, since we have not provided a specific API key value, one is generated for
//! us by the Connect API.
//!
//! ```
//! use connect_1password::{
//!     error::Error,
//!     client::{Client, HTTPClient},
//!     models::{
//!         item::{ApiCredentialItem, FullItem, ItemBuilder, ItemCategory, FieldObject},
//!     },
//!     vaults,
//!     items,
//! };
//!
//! const SLEEP_DELAY: u64 = 4; // seconds
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::default();
//!
//!     let (vaults, _) = vaults::all(&client).await?;
//!     assert!(!vaults.is_empty());
//!
//!     let item: FullItem = ItemBuilder::new(&vaults[0].id, ItemCategory::ApiCredential)
//!         .api_key("", "Dell XYZ")
//!         .build()
//!         .unwrap();
//!
//!     let client = Client::default();
//!     let (new_item, _) = items::add(&client, item).await?;
//!     assert_eq!(new_item.title, "Dell XYZ");
//!
//!     tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;
//!
//!     let (item, _) = items::get(&client, &vaults[0].id, &new_item.id).await?;
//!     let fields: Vec<_> = item.fields.into_iter().filter(|r| r.value.is_some()).collect();
//!     assert_eq!(fields.len(), 1);
//!
//!     let default_value = "".to_string();
//!     let api_value = fields[0].value.as_ref().unwrap_or(&default_value);
//!     let field_type = fields[0].r#type.as_ref().unwrap_or(&default_value);
//!     assert_eq!(field_type, "CONCEALED");
//!     assert!(!api_value.is_empty());
//!
//!     // Just as a clean up measure, we remove the item created in the this example
//!     tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;
//!
//!     items::remove(&client, &vaults[0].id, &new_item.id)
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! However, if we provide a specific key, this is the value persisted into 1Password.
//!
//! ```
//! use connect_1password::{
//!     error::Error,
//!     client::{Client, HTTPClient},
//!     models::{
//!         item::{ApiCredentialItem, FullItem, ItemBuilder, ItemCategory, FieldObject},
//!     },
//!     vaults,
//!     items,
//! };
//!
//! const SLEEP_DELAY: u64 = 4; // seconds
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::default();
//!
//!     let (vaults, _) = vaults::all(&client).await?;
//!     assert!(!vaults.is_empty());
//!
//!     let item: FullItem = ItemBuilder::new(&vaults[0].id, ItemCategory::ApiCredential)
//!         .api_key("smelly-socks", "Dell XYZ")
//!         .build()
//!         .unwrap();
//!
//!     let (new_item, _) = items::add(&client, item).await?;
//!     assert_eq!(new_item.title, "Dell XYZ");
//!
//!     tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;
//!
//!     let client = Client::default();
//!     let (item, _) = items::get(&client, &vaults[0].id, &new_item.id).await?;
//!     let fields: Vec<_> = item.fields.into_iter().filter(|r| r.value.is_some()).collect();
//!     assert_eq!(fields.len(), 1);
//!
//!     let default_value = "".to_string();
//!     let api_value = fields[0].value.as_ref().unwrap_or(&default_value);
//!     let field_type = fields[0].r#type.as_ref().unwrap_or(&default_value);
//!     assert_eq!(field_type, "CONCEALED");
//!     assert_eq!(api_value, "smelly-socks");
//!
//!     // Just as a clean up measure, we remove the item created in the this example
//!     tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;
//!
//!     items::remove(&client, &vaults[0].id, &new_item.id)
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

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
