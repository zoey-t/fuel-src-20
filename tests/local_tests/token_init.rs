use crate::utils::local_test_utils::{
    abi_calls::{decimals, name, owner, symbol},
    setup_utils::setup_token,
};

use fuels::prelude::*;

#[tokio::test]
async fn should_have_correct_config_and_owner() {
    let token_name = "My Token";
    let token_symbol = "MTK";
    let token_decimals = 18;

    let (token_instance, wallets) = setup_token(token_name, token_symbol, token_decimals).await;
    // name
    let name = name(&token_instance).await.unwrap().value;

    let mut expected_name = token_name.to_string();
    expected_name.push_str(" ".repeat(32 - token_name.len()).as_str());

    assert_eq!(name.to_string(), expected_name);

    // symbol
    let symbol = symbol(&token_instance).await.unwrap().value;
    let mut expeted_symbol = token_symbol.to_string();
    expeted_symbol.push_str(" ".repeat(8 - token_symbol.len()).as_str());

    assert_eq!(symbol.to_string(), expeted_symbol);

    // decimal
    let decimals = decimals(&token_instance).await.unwrap().value;
    assert_eq!(decimals, token_decimals);

    // owner
    let owner = owner(&token_instance).await.unwrap().value;
    assert_eq!(
        owner,
        Identity::Address(Address::from(wallets.wallet_owner.address()))
    );
}
