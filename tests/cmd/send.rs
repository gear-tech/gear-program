//! Integration tests for command `send`
use crate::common::{self, Result};
use gear_program::api::Api;

#[tokio::test]
async fn test_command_send_works() -> Result<()> {
    let node = common::create_messager().await?;

    // Get balance of the testing address
    let api = Api::new(Some(&node.ws()), None).await?;
    let mailbox = api.mailbox(common::alice_account_id(), 10).await?;

    assert_eq!(mailbox.len(), 1);
    Ok(())
}
