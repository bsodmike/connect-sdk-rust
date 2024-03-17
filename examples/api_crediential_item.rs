use connect_1password::error::ConnectResult;
#[allow(unused_imports)]
use connect_1password::{
    client::Client,
    items,
    models::{
        item::{FullItem, ItemBuilder, ItemCategory, LoginItem},
        ApiCredentialItem,
    },
    vaults,
};

const SLEEP_DELAY: u64 = 4; // seconds

#[tokio::main]
async fn main() -> ConnectResult<()> {
    let client = Client::default();

    let (vaults, _) = vaults::all(&client).await?;
    assert!(!vaults.is_empty());

    let item = ApiCredentialItem::build(
        &ItemBuilder::new(&vaults[0].id, ItemCategory::ApiCredential)
            .api_key("smelly-socks", "Dell XYZ"),
    )?;

    let (new_item, _) = items::add(&client, item).await?;
    assert_eq!(new_item.title, "Dell XYZ");

    tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

    let client = Client::default();
    let (item, _) = items::get(&client, &vaults[0].id, &new_item.id).await?;
    let fields: Vec<_> = item
        .fields
        .into_iter()
        .filter(|r| r.value.is_some())
        .collect();
    assert_eq!(fields.len(), 1);

    let default_value = "".to_string();
    let api_value = fields[0].value.as_ref().unwrap_or(&default_value);
    let field_type = fields[0].r#type.as_ref().unwrap_or(&default_value);
    assert_eq!(field_type, "CONCEALED");
    assert_eq!(api_value, "smelly-socks");

    // Just as a clean up measure, we remove the item created in the this example
    tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

    items::remove(&client, &vaults[0].id, &new_item.id).await?;

    Ok(())
}
