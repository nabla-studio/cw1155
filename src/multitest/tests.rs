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

    let info = contract.query_contract_info(&app).unwrap();

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

    let info = contract.query_contract_info(&app).unwrap();

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

    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

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

    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

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

    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

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

    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

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

    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

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

    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

    assert_eq!(token_count, 0);
}

#[test]
fn mint_token() {
    let sender = Addr::unchecked("sender");
    let recipient = Addr::unchecked("recipient");

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

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    contract
        .mint_token(
            &mut app,
            &sender,
            recipient.as_str(),
            1,
            Uint128::from(10u128),
            None,
        )
        .unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::from(10u128));

    let user_balance = contract
        .query_balance(&app, recipient.into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(user_balance, Uint128::from(10u128));
}

#[test]
fn unauthorized_mint() {
    let sender = Addr::unchecked("sender");
    let recipient = Addr::unchecked("recipient");

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

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    let err = contract
        .mint_token(
            &mut app,
            &recipient,
            recipient.as_str(),
            1,
            Uint128::from(10u128),
            None,
        )
        .unwrap_err();

    assert_eq!(err, ContractError::NotMinter);

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    let user_balance = contract
        .query_balance(&app, recipient.into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(user_balance, Uint128::zero());
}

#[test]
fn try_overcome_max_supply() {
    let sender = Addr::unchecked("sender");
    let recipient = Addr::unchecked("recipient");

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
        .register_token(&mut app, &sender, Uint128::from(20u128), None)
        .unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    let err = contract
        .mint_token(
            &mut app,
            &sender,
            recipient.as_str(),
            1,
            Uint128::from(100u128),
            None,
        )
        .unwrap_err();

    assert_eq!(err, ContractError::CannotExceedMaxSupply);

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());
}

#[test]
fn multiple_mint() {
    let sender = Addr::unchecked("sender");
    let recipient = Addr::unchecked("recipient");

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
        .register_token(&mut app, &sender, Uint128::from(20u128), None)
        .unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    contract
        .mint_token(
            &mut app,
            &sender,
            recipient.as_str(),
            1,
            Uint128::from(10u128),
            None,
        )
        .unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::from(10u128));

    contract
        .mint_token(
            &mut app,
            &sender,
            recipient.as_str(),
            1,
            Uint128::from(10u128),
            None,
        )
        .unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::from(20u128));
}
