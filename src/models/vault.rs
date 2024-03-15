use chrono::{DateTime, Utc};

use serde::Deserialize;

/// Defines a Vault object
#[derive(Debug, Deserialize, PartialEq)]
pub struct VaultData {
    /// The UUID of the vault.
    pub id: String,
    /// The name of the vault.
    pub name: String,
    /// The description of the vault.
    pub description: Option<String>,
    /// The version of the vault metadata.
    #[serde(rename = "attributeVersion")]
    pub attribute_version: u32,
    /// The version of the vault contents.
    #[serde(rename = "contentVersion")]
    pub content_version: u32,
    /// The type of vault.
    pub r#type: String,
    /// Date and time when the vault was created.
    pub created_at: Option<DateTime<Utc>>,
    /// Date and time when the vault or its contents were last changed.
    pub updated_at: Option<DateTime<Utc>>,
}
