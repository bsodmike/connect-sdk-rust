//! Models

/// Item related models
pub mod item;
/// Vault related models
pub mod vault;

pub use item::*;
pub use vault::*;

/// This is a wrapper to assist creating instances of `ConnectAPIError`
#[derive(Debug)]
pub struct StatusWrapper {
    pub(crate) status: u16,
}

impl From<StatusWrapper> for String {
    fn from(val: StatusWrapper) -> Self {
        val.status.to_string()
    }
}
