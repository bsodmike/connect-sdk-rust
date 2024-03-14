//! Vaults

use crate::error::{ConnectAPIError, Error};
use crate::{
    client::HTTPClient,
    models::{StatusWrapper, VaultData},
};

/// Get all known vaults
pub async fn all(client: &impl HTTPClient) -> Result<(Vec<VaultData>, serde_json::Value), Error> {
    let params = vec![("", "")];

    let result = match client
        .send_request::<Vec<VaultData>>("GET", "v1/vaults", &params, None)
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

/// Get vault details
pub async fn get(
    client: &impl HTTPClient,
    id: &str,
) -> Result<(VaultData, serde_json::Value), Error> {
    let params = vec![("", "")];
    let path = format!("v1/vaults/{}", id);

    let result = match client
        .send_request::<VaultData>("GET", &path, &params, None)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            let op_error = crate::error::process_connect_error_response(err.to_string())?;

            let mut message = "Invalid bearer token";
            if err.to_string().contains(message) {
                let status = StatusWrapper {
                    status: op_error.status_code.unwrap_or_default(),
                };

                return Err(Error::new_connect_error(ConnectAPIError::new(
                    status.into(),
                    message,
                )));
            }

            message = "Invalid Vault UUID";
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

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use crate::{get_test_client, vaults};
    use tokio::test;

    #[test]
    async fn all() {
        let client = get_test_client();

        let (vaults, _) = vaults::all(&client).await.unwrap();
        dbg!(&vaults);

        assert_eq!(vaults[0].name, "Automated".to_string());
    }

    #[test]
    async fn get() {
        let client = get_test_client();
        let test_vault_id =
            std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");

        let (vault, _) = vaults::get(&client, &test_vault_id).await.unwrap();
        dbg!(&vault);

        assert_eq!(vault.name, "Automated".to_string());
    }

    #[should_panic]
    #[test]
    async fn get_vault_details_not_specified() {
        let client = get_test_client();

        let (vault, _) = vaults::get(&client, "").await.unwrap();

        assert_eq!(vault.name, "Automated".to_string());
    }

    #[should_panic]
    #[test]
    async fn get_vault_details_invalid_vault() {
        let client = get_test_client();

        let (_vault, _) = vaults::get(&client, "foo").await.unwrap();
    }
}
