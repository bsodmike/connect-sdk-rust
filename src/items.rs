//! Vault items

use crate::error::{ConnectAPIError, Error};
use crate::{
    client::HTTPClient,
    models::{
        item::{FullItem, ItemData},
        StatusWrapper,
    },
};
use serde::Deserialize;

/// Get all items
pub async fn all(
    client: &impl HTTPClient,
    id: &str,
) -> Result<(Vec<ItemData>, serde_json::Value), crate::error::Error> {
    let params = vec![("", "")];
    let path = format!("v1/vaults/{}/items", id);

    let result = match client
        .send_request::<Vec<ItemData>>("GET", &path, &params, None)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            let op_error = crate::error::process_connect_error_response(err.to_string())?;

            let message = "Invalid bearer token";
            if err.to_string().contains(message) {
                let status = StatusWrapper {
                    status: op_error.status_code.unwrap_or_default(),
                };

                return Err(Error::new_connect_error(ConnectAPIError::new(
                    status.into(),
                    message,
                )));
            }

            return Err(Error::new_internal_error().with(err));
        }
    };

    Ok(result)
}

/// Get item details
pub async fn get(
    client: &impl HTTPClient,
    vault_id: &str,
    item_id: &str,
) -> Result<(FullItem, serde_json::Value), crate::error::Error> {
    let params = vec![("", "")];
    let path = format!("v1/vaults/{}/items/{}", vault_id, item_id);

    let body = None;
    let result = match client
        .send_request::<FullItem>("GET", &path, &params, body)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            let op_error = crate::error::process_connect_error_response(err.to_string())?;

            let message = "Invalid bearer token";
            if err.to_string().contains(message) {
                let status = StatusWrapper {
                    status: op_error.status_code.unwrap_or_default(),
                };

                return Err(Error::new_connect_error(ConnectAPIError::new(
                    status.into(),
                    message,
                )));
            }

            return Err(Error::new_internal_error().with(err));
        }
    };

    Ok(result)
}

/// Add an item
pub async fn add(
    client: &impl HTTPClient,
    item: FullItem,
) -> Result<(ItemData, serde_json::Value), crate::error::Error> {
    let id = &item.vault.id;

    let params = vec![("", "")];
    let path = format!("v1/vaults/{}/items", id);

    let body = Some(serde_json::to_string(&item)?);
    let result = match client
        .send_request::<ItemData>("POST", &path, &params, body)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            let op_error = crate::error::process_connect_error_response(err.to_string())?;

            let message = "Invalid bearer token";
            if err.to_string().contains(message) {
                let status = StatusWrapper {
                    status: op_error.status_code.unwrap_or_default(),
                };

                return Err(Error::new_connect_error(ConnectAPIError::new(
                    status.into(),
                    message,
                )));
            }

            return Err(Error::new_internal_error().with(err));
        }
    };

    Ok(result)
}

#[derive(Debug, Deserialize, PartialEq)]
struct DeleteReturnType {}

/// Delete an item
pub async fn remove(
    client: &impl HTTPClient,
    id: &str,
    item_id: &str,
) -> Result<(), crate::error::Error> {
    let params = vec![("", "")];
    let path = format!("v1/vaults/{}/items/{}", id, item_id);
    dbg!(&path);

    let body = None;
    let _result = match client
        .send_request::<DeleteReturnType>("DELETE", &path, &params, body)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            let op_error = crate::error::process_connect_error_response(err.to_string())?;

            let message = "Invalid bearer token";
            if err.to_string().contains(message) {
                let status = StatusWrapper {
                    status: op_error.status_code.unwrap_or_default(),
                };

                return Err(Error::new_connect_error(ConnectAPIError::new(
                    status.into(),
                    message,
                )));
            }

            return Err(Error::new_internal_error().with(err));
        }
    };

    Ok(())
}

#[allow(dead_code)]
const SLEEP_DELAY: u64 = 4; // seconds

#[cfg(test)]
mod tests {
    use super::*;

    mod default {
        use super::SLEEP_DELAY;
        use crate::get_test_client;
        use tokio::test;

        use crate::{
            items,
            models::item::{DefaultItem, FullItem, ItemBuilder, ItemCategory},
        };

        #[test]
        async fn add_item() {
            let (client, test_vault_id) = get_test_client();

            let item: FullItem = ItemBuilder::new(&test_vault_id, ItemCategory::ApiCredential)
                .build()
                .unwrap();
            let (new_item, _) = items::add(&client, item).await.unwrap();
            dbg!(&new_item);

            assert_ne!(new_item.id, "foo");

            tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

            items::remove(&client, &test_vault_id, &new_item.id)
                .await
                .unwrap();
        }

        #[test]
        async fn all() {
            let (client, test_vault_id) = get_test_client();

            let (items, _) = items::all(&client, &test_vault_id).await.unwrap();

            if items.len() == 0 {
                assert!(items.is_empty());
            }
        }
    }

    mod login_item {
        use super::SLEEP_DELAY;
        use crate::get_test_client;
        use tokio::test;

        use crate::{
            items,
            models::item::{FullItem, ItemBuilder, ItemCategory, LoginItem},
        };

        #[test]
        async fn add_login_item() {
            let (client, test_vault_id) = get_test_client();

            let item: FullItem = ItemBuilder::new(&test_vault_id, ItemCategory::Login)
                .title("Test login item")
                .username("Bob")
                .password("")
                .build()
                .unwrap();
            let (new_item, _) = items::add(&client, item).await.unwrap();
            dbg!(&new_item);

            assert_ne!(new_item.id, "foo");

            tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

            items::remove(&client, &test_vault_id, &new_item.id)
                .await
                .unwrap();
        }

        #[test]
        async fn add_login_item_with_otp() {
            let (client, test_vault_id) = get_test_client();

            let item: FullItem = ItemBuilder::new(&test_vault_id, ItemCategory::Login)
                .title("Test login item")
                .username("Bob")
                .password("")
                .add_otp("replaceme")
                .build()
                .unwrap();
            let (new_item, _) = items::add(&client, item).await.unwrap();
            dbg!(&new_item);

            assert_ne!(new_item.id, "foo");

            tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

            items::remove(&client, &test_vault_id, &new_item.id)
                .await
                .unwrap();
        }

        #[should_panic]
        #[test]
        async fn add_login_item_requires_title() {
            let (client, test_vault_id) = get_test_client();

            let item: FullItem = ItemBuilder::new(&test_vault_id, ItemCategory::Login)
                .username("Bob")
                .password("")
                .build()
                .unwrap();
            let (_new_item, _) = items::add(&client, item).await.unwrap();
        }

        #[test]
        async fn remove_login_item() {
            let (client, test_vault_id) = get_test_client();

            let item: FullItem = ItemBuilder::new(&test_vault_id, ItemCategory::Login)
                .title("Test login item, will be removed")
                .username("Bob")
                .password("")
                .build()
                .unwrap();
            let (new_item, _) = items::add(&client, item).await.unwrap();
            dbg!(&new_item);

            tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY + 2, 0)).await;

            items::remove(&client, &test_vault_id, &new_item.id)
                .await
                .unwrap();

            tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

            let (items, _) = items::all(&client, &test_vault_id).await.unwrap();
            assert!(items.is_empty());
        }
    }

    mod api_credential_item {
        use super::SLEEP_DELAY;
        use crate::get_test_client;
        use tokio::test;

        use crate::{
            items,
            models::item::{ApiCredentialItem, FullItem, ItemBuilder, ItemCategory},
        };

        #[test]
        async fn get_item() {
            let (client, test_vault_id) = get_test_client();

            let item: FullItem = ItemBuilder::new(&test_vault_id, ItemCategory::ApiCredential)
                .api_key("lawyer-rottenborn", "Dell XYZ")
                .build()
                .unwrap();
            let (new_item, _) = items::add(&client, item).await.unwrap();
            assert_eq!(new_item.title, "Dell XYZ");

            tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

            let (item, _) = items::get(&client, &test_vault_id, &new_item.id)
                .await
                .unwrap();
            let fields: Vec<_> = item
                .fields
                .into_iter()
                .filter(|r| r.value.is_some())
                .collect();
            assert_eq!(fields.len(), 1);
            dbg!(&fields);

            let default_value = "".to_string();
            let api_value = fields[0].value.as_ref().unwrap_or(&default_value);
            let field_type = fields[0].r#type.as_ref().unwrap_or(&default_value);
            assert_eq!(field_type, "CONCEALED");
            assert_eq!(api_value, "lawyer-rottenborn");

            // Just as a clean up measure, we remove the item created in the this example
            tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

            items::remove(&client, &test_vault_id, &new_item.id)
                .await
                .unwrap();
        }

        #[test]
        async fn add_api_credential_item() {
            let (client, test_vault_id) = get_test_client();

            let item: FullItem = ItemBuilder::new(&test_vault_id, ItemCategory::ApiCredential)
                .api_key("", "Dell XYZ")
                .build()
                .unwrap();
            let (new_item, _) = items::add(&client, item).await.unwrap();
            dbg!(&new_item);

            assert_ne!(new_item.id, "foo");

            tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

            items::remove(&client, &test_vault_id, &new_item.id)
                .await
                .unwrap();
        }
    }
}
