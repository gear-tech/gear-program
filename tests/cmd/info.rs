//! Integration tests for command `deploy`
use crate::common::{self, logs, traits::Convert, Result, ALICE_SS58_ADDRESS};

const EXPECTED_BALANCE: &str = r#"
AccountInfo {
    nonce: 0,
    consumers: 0,
    providers: 1,
    sufficients: 0,
    data: AccountData {
        free: 1152921504606846976,
        reserved: 0,
        misc_frozen: 0,
        fee_frozen: 0,
    },
}
"#;

const EXPECTED_MAILBOX: &str = r#"
Mail {
    id: "0xd29c74cb2f634999595744197ba2fd022b652c1f0ff9b96957e88fc377f23116",
    source: "0x35146b3489795613ad4cfce3d540f173345ee96cc7a97e3676edd807fb787e28",
    destination: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
    payload: "0x",
    value: 1000000,
    reply: None,
    interval: Interval {
        start: 2,
        finish: 31,
    },
}
"#;

#[tokio::test]
async fn test_action_balance_works() -> Result<()> {
    common::login_as_alice().expect("login failed");
    let mut node = common::Node::dev()?;
    node.wait(logs::gear_node::IMPORTING_BLOCKS)?;

    let output = common::gear(&["-e", &node.ws(), "info", "//Alice", "balance"])?;
    assert_eq!(EXPECTED_BALANCE.trim(), output.stdout.convert().trim());
    Ok(())
}

#[tokio::test]
async fn test_action_mailbox_works() -> Result<()> {
    let node = common::create_messager().await?;
    let output = common::gear(&["-e", &node.ws(), "info", ALICE_SS58_ADDRESS, "mailbox"])?;

    assert_eq!(EXPECTED_MAILBOX.trim(), output.stdout.convert().trim());
    Ok(())
}
