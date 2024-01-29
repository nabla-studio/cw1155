use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::App;

use crate::{msg::ContractInfoResponse, ContractError};

use super::contract::Cw1155;

const METADATA_URI: &str = "https://metadata_uri_nabla_collection/{ID}.json";
const NAME: &str = "nabla collection";
const DESCRIPTION: &str = "This is the official nabla tokens collection";

#[test]
fn get_config() {
    let sender = Addr::unchecked("sender");
    let minter = Addr::unchecked("minter");
    let owner = Addr::unchecked("owner");

    let mut app = App::default();

    let code_id = Cw1155::store_code(&mut app);

    let contract = Cw1155::instantiate(
        &mut app,
        code_id,
        &sender,
        "CW1155 nabla collection",
        None,
        &METADATA_URI,
        Some(&minter),
        Some(&owner),
        &NAME,
        &DESCRIPTION,
    )
    .unwrap();

    // Query the contract configuration
    let info = contract.query_contract_info(&app).unwrap();

    // Verify that contract configuration is as expected
    assert_eq!(
        info,
        ContractInfoResponse {
            registered_tokens: 0,
            metadata_uri: METADATA_URI.to_string(),
            minter: Some(minter.to_string()),
            owner: Some(owner.to_string()),
            name: NAME.to_string(),
            description: DESCRIPTION.to_string(),
        }
    );
}

#[test]
fn get_config_empty_owner_and_minter() {
    let sender = Addr::unchecked("sender");

    let mut app = App::default();

    let code_id = Cw1155::store_code(&mut app);

    let contract = Cw1155::instantiate(
        &mut app,
        code_id,
        &sender,
        "CW1155 nabla collection",
        None,
        &METADATA_URI,
        None,
        None,
        &NAME,
        &DESCRIPTION,
    )
    .unwrap();

    // Query the contract configuration
    let info = contract.query_contract_info(&app).unwrap();

    // Verify that contract configuration is as expected
    assert_eq!(
        info,
        ContractInfoResponse {
            registered_tokens: 0,
            metadata_uri: METADATA_URI.to_string(),
            minter: Some(sender.to_string()),
            owner: Some(sender.to_string()),
            name: NAME.to_string(),
            description: DESCRIPTION.to_string(),
        }
    );
}

#[test]
fn get_no_registered_tokens() {
    let sender = Addr::unchecked("sender");

    let mut app = App::default();

    let code_id = Cw1155::store_code(&mut app);

    let contract = Cw1155::instantiate(
        &mut app,
        code_id,
        &sender,
        "CW1155 nabla collection",
        None,
        &METADATA_URI,
        None,
        None,
        &NAME,
        &DESCRIPTION,
    )
    .unwrap();

    // Query the contract configuration
    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

    // Verify that contract configuration is as expected
    assert_eq!(token_count, 0);
}

#[test]
fn register_max_supply_and_is_transferrable_token() {
    let sender = Addr::unchecked("sender");

    let mut app = App::default();

    let code_id = Cw1155::store_code(&mut app);

    let contract = Cw1155::instantiate(
        &mut app,
        code_id,
        &sender,
        "CW1155 nabla collection",
        None,
        &METADATA_URI,
        None,
        None,
        &NAME,
        &DESCRIPTION,
    )
    .unwrap();

    contract
        .register_token(&mut app, &sender, Uint128::from(1000u128), true)
        .unwrap();

    // Query the contract configuration
    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

    // Verify that contract configuration is as expected
    assert_eq!(token_count, 1);
}

#[test]
fn register_max_supply_and_is_not_transferrable_token() {
    let sender = Addr::unchecked("sender");

    let mut app = App::default();

    let code_id = Cw1155::store_code(&mut app);

    let contract = Cw1155::instantiate(
        &mut app,
        code_id,
        &sender,
        "CW1155 nabla collection",
        None,
        &METADATA_URI,
        None,
        None,
        &NAME,
        &DESCRIPTION,
    )
    .unwrap();

    contract
        .register_token(&mut app, &sender, Uint128::from(1u128), false)
        .unwrap();

    // Query the contract configuration
    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

    // Verify that contract configuration is as expected
    assert_eq!(token_count, 1);
}

#[test]
fn register_max_supply_zero_token() {
    let sender = Addr::unchecked("sender");

    let mut app = App::default();

    let code_id = Cw1155::store_code(&mut app);

    let contract = Cw1155::instantiate(
        &mut app,
        code_id,
        &sender,
        "CW1155 nabla collection",
        None,
        &METADATA_URI,
        None,
        None,
        &NAME,
        &DESCRIPTION,
    )
    .unwrap();

    let err = contract
        .register_token(&mut app, &sender, Uint128::from(0u128), None)
        .unwrap_err();

    assert_eq!(err, ContractError::ZeroMaxSupply {});

    // Query the contract configuration
    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

    // Verify that contract configuration is as expected
    assert_eq!(token_count, 0);
}

#[test]
fn get_one_registered_token() {
    let sender = Addr::unchecked("sender");

    let mut app = App::default();

    let code_id = Cw1155::store_code(&mut app);

    let contract = Cw1155::instantiate(
        &mut app,
        code_id,
        &sender,
        "CW1155 nabla collection",
        None,
        &METADATA_URI,
        None,
        None,
        &NAME,
        &DESCRIPTION,
    )
    .unwrap();

    contract
        .register_token(&mut app, &sender, None, None)
        .unwrap();

    // Query the contract configuration
    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

    // Verify that contract configuration is as expected
    assert_eq!(token_count, 1);
}

#[test]
fn unauthorized_register_token() {
    let sender = Addr::unchecked("sender");
    let owner = Addr::unchecked("owner");

    let mut app = App::default();

    let code_id = Cw1155::store_code(&mut app);

    let contract = Cw1155::instantiate(
        &mut app,
        code_id,
        &sender,
        "CW1155 nabla collection",
        None,
        &METADATA_URI,
        None,
        Some(&owner),
        &NAME,
        &DESCRIPTION,
    )
    .unwrap();

    let err = contract
        .register_token(&mut app, &sender, None, None)
        .unwrap_err();

    assert_eq!(err, ContractError::NotOwner {});

    // Query the contract configuration
    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

    // Verify that contract configuration is as expected
    assert_eq!(token_count, 0);
}
