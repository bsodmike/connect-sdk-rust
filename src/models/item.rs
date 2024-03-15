use crate::error::{CustomError, ErrorTrait};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Defines an Item Object
#[derive(Debug, Deserialize, PartialEq)]
pub struct ItemData {
    /// The UUID of the item.
    pub id: String,
    /// The title of the item.
    pub title: String,
    /// An object containing an id property whose value is the UUID of the vault the item is in.
    pub vault: VaultID,
    /// The category of the item.
    pub category: Option<String>,
    /// Vector of URL objects containing URLs for the item.
    pub urls: Option<Vec<UrlObject>>,
    /// Whether the item is marked as a favourite.
    pub favorite: Option<bool>,
    /// A vector of strings of the tags assigned to the item.
    pub tags: Option<Vec<String>>,
    /// The state of the item.
    pub state: Option<String>,
    /// Date and time when the vault was created.
    pub created_at: Option<DateTime<Utc>>,
    /// Date and time when the vault or its contents were last changed.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Defines the Vault UUID via a record struct
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct VaultID {
    /// The UUID of the vault.
    pub id: String,
}

/// Defines a URL Object
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct UrlObject {
    /// The address.
    pub url: String,
    /// Whether this is the primary URL for the item.
    pub primary: bool,
}

/// This is a Field Object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FieldObject {
    /// An object containing the UUID of a section in the item.
    pub section: Option<SectionID>,
    /// Use `purpose` for the username, password, and notes fields.
    pub purpose: Option<String>,
    /// Use `type' for all other fields
    pub r#type: Option<String>,
    /// The value to save for the field. You can specify a `generate` field instead of `value` to create a password or other random information for the value.
    pub value: Option<String>,
    /// Generate a password and save in the value for the field. By default, the password is a 32-characters long, made up of letters, numbers, and symbols. To customize the password, include a `recipe` field.
    pub generate: Option<bool>,
    // FIXME the GeneratorRecipe needs to be added
    // pub recipe
    /// Some optional text
    pub label: Option<String>,
}

/// Used to specify type of a Field Object
#[derive(Debug)]
pub enum FieldType {
    /// Item value will be concealed
    Concealed,
}

impl Into<String> for FieldType {
    fn into(self) -> String {
        let value = match self {
            Self::Concealed => "CONCEALED",
        };

        value.to_string()
    }
}

/// This is a Section Object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SectionObject {
    /// The UUID of the section.
    pub id: String,
    /// Some optional text
    pub label: Option<String>,
}

impl SectionObject {
    /// Create a new instance
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: Some(label.to_string()),
        }
    }
}

/// This is a SectionID
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SectionID {
    /// The UUID of the section.
    pub id: String,
}

impl SectionID {
    /// Create new instance
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
        }
    }
}

impl Default for SectionID {
    fn default() -> Self {
        Self::new()
    }
}

/// This is a FullItem
#[derive(Debug, Deserialize, Serialize)]
pub struct FullItem {
    /// The title of the item.
    pub title: String,
    /// An object containing an id property whose value is the UUID of the vault the item is in.
    pub vault: VaultID,
    /// The category of the item.
    pub category: Option<String>,
    /// Vector of URL objects containing URLs for the item.
    pub urls: Option<Vec<UrlObject>>,
    /// Whether the item is marked as a favourite.
    pub favorite: Option<bool>,
    /// A vector of strings of the tags assigned to the item.
    pub tags: Option<Vec<String>>,
    /// A vector of Field objects of the fields to include with the item.
    pub fields: Vec<FieldObject>,
    /// A vector of Section objects of the sections to include with the item.
    pub sections: Vec<SectionObject>,
}

/// Defines a default interface
pub trait DefaultItem {
    /// Execute the builder
    fn build(&self) -> Result<FullItem, Box<dyn ErrorTrait + Send + Sync>>;
}

/// Defines an interface for a Login item
pub trait LoginItem {
    /// Specify title
    fn title(self, username: &str) -> Self;
    /// Specify username
    fn username(self, username: &str) -> Self;
    /// Specify password
    fn password(self, password: &str) -> Self;
    /// Execute the builder
    fn build(&self) -> Result<FullItem, Box<dyn std::error::Error + Send + Sync>>;
}

/// Defines an interface for a Api Credential item
pub trait ApiCredentialItem {
    /// Specify API key
    fn api_key(self, key: &str, title: &str) -> Self;
    /// Execute the builder
    fn build(&self) -> Result<FullItem, Box<dyn std::error::Error + Send + Sync>>;
}

/// This is an ItemBuilder
#[derive(Debug)]
pub struct ItemBuilder {
    /// The title of the item.
    pub title: String,
    /// An object containing an id property whose value is the UUID of the vault the item is in.
    pub vault: VaultID,
    /// The category of the item.
    pub category: Option<String>,
    /// Vector of URL objects containing URLs for the item.
    pub urls: Option<Vec<UrlObject>>,
    /// Whether the item is marked as a favourite.
    pub favorite: Option<bool>,
    /// A vector of strings of the tags assigned to the item.
    pub tags: Option<Vec<String>>,
    /// A vector of Field objects of the fields to include with the item.
    pub fields: Vec<FieldObject>,
    /// A vector of Section objects of the sections to include with the item.
    pub sections: Vec<SectionObject>,
}

/// Describes usable Item categories
#[derive(Debug)]
pub enum ItemCategory {
    /// API Credential
    ApiCredential,
    /// Login
    Login,
    /// Password
    Password,
}

impl ItemCategory {
    #[allow(dead_code)]
    fn default() -> Self {
        Self::ApiCredential
    }
}

impl Into<String> for ItemCategory {
    fn into(self) -> String {
        let value = match self {
            Self::ApiCredential => "API_CREDENTIAL",
            Self::Login => "LOGIN",
            Self::Password => "PASSWORD",
        };

        value.to_string()
    }
}

impl ItemBuilder {
    /// Create a new instance
    pub fn new(vault_id: &str, category: ItemCategory) -> Self {
        let vault = VaultID {
            id: vault_id.to_string(),
        };

        Self {
            vault,
            title: String::default(),
            category: Some(category.into()),
            favorite: Some(false),
            urls: None,
            tags: None,
            fields: vec![],
            sections: vec![],
        }
    }

    #[allow(dead_code)]
    // FIXME: This needs testing to ensure the OTP secret is applied correctly
    pub(crate) fn add_otp(mut self, secret: &str) -> Self {
        let section = SectionID::new();
        let section_obj = SectionObject::new(&section.id, "OTP");

        self.sections.push(section_obj);

        let field_object = FieldObject {
            section: Some(section),
            label: None,
            purpose: None,
            r#type: Some("OTP".to_string()),
            generate: Some(true),
            value: Some(secret.to_string()),
        };
        self.fields.push(field_object);

        self
    }
}

impl DefaultItem for ItemBuilder {
    fn build(&self) -> Result<FullItem, Box<dyn ErrorTrait + Send + Sync>> {
        Ok(FullItem {
            title: self.title.clone(),
            category: self.category.clone(),
            favorite: self.favorite,
            fields: self.fields.clone(),
            sections: self.sections.clone(),
            tags: self.tags.clone(),
            urls: self.urls.clone(),
            vault: self.vault.clone(),
        })
    }
}

impl LoginItem for ItemBuilder {
    fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    fn username(mut self, username: &str) -> Self {
        let field: FieldObject = FieldObject {
            value: Some(username.to_string()),
            purpose: Some("USERNAME".to_string()),
            generate: None,
            label: None,
            r#type: None,
            section: None,
        };

        self.fields.push(field);
        self
    }

    fn password(mut self, password: &str) -> Self {
        let field: FieldObject = FieldObject {
            value: password.is_empty().then(|| password.to_string()),
            purpose: Some("PASSWORD".to_string()),
            generate: password.is_empty().then(|| true),
            label: None,
            r#type: None,
            section: None,
        };

        self.fields.push(field);
        self
    }

    fn build(&self) -> Result<FullItem, Box<dyn std::error::Error + Send + Sync>> {
        if self.title.is_empty() {
            return Err(Box::new(CustomError::new("Title is required")));
        }

        Ok(FullItem {
            title: self.title.clone(),
            category: self.category.clone(),
            favorite: self.favorite,
            fields: self.fields.clone(),
            sections: self.sections.clone(),
            tags: self.tags.clone(),
            urls: self.urls.clone(),
            vault: self.vault.clone(),
        })
    }
}

impl ApiCredentialItem for ItemBuilder {
    fn api_key(mut self, key: &str, title: &str) -> Self {
        let section = SectionID::new();
        let section_obj = SectionObject::new(&section.id, "API Key");

        self.sections.push(section_obj);

        let field_object = FieldObject {
            section: Some(section),
            label: None,
            purpose: None,
            r#type: Some(FieldType::Concealed.into()),
            generate: Some(key.is_empty()),
            value: Some(key.to_string()),
        };
        self.fields.push(field_object);
        self.title = title.to_string();

        self
    }

    fn build(&self) -> Result<FullItem, Box<dyn std::error::Error + Send + Sync>> {
        Ok(FullItem {
            title: self.title.clone(),
            category: self.category.clone(),
            favorite: self.favorite,
            fields: self.fields.clone(),
            sections: self.sections.clone(),
            tags: self.tags.clone(),
            urls: self.urls.clone(),
            vault: self.vault.clone(),
        })
    }
}
