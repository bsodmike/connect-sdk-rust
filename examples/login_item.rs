use connect_1password::error::ConnectResult;
#[allow(unused_imports)]
use connect_1password::{
    client::Client,
    items,
    models::item::{FullItem, ItemBuilder, ItemCategory, LoginItem},
    vaults,
};

const SLEEP_DELAY: u64 = 4; // seconds

#[tokio::main]
async fn main() -> ConnectResult<()> {
    let client = Client::default();

    let (vaults, _) = vaults::all(&client).await?;
    assert!(!vaults.is_empty());

    let item: FullItem = ItemBuilder::new(&vaults[0].id, ItemCategory::Login)
        .title("Secure server login")
        .username("Bob")
        .password("")
        .build()?;

    let (new_item, _) = items::add(&client, item).await?;
    assert_eq!(new_item.title, "Secure server login");

    // Just as a clean up measure, we remove the item created in the this example
    tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

    items::remove(&client, &vaults[0].id, &new_item.id).await?;

    Ok(())
}
