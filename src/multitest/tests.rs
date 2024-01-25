use cosmwasm_std::Addr;
use cw_multi_test::App;

use crate::msg::ContractInfoResponse;

use super::contract::Cw1155;

#[test]
fn get_config() {
    let metadata_uri = String::from("https://metadata_uri_nabla_collection/{ID}.json");
    let name = String::from("nabla collection");
    let description = String::from("This is the official nabla tokens collection");
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
        &metadata_uri,
        Some(&minter),
        Some(&owner),
        &name,
        &description,
    )
    .unwrap();

    // Query the contract configuration
    let info = contract.query_contract_info(&app).unwrap();

    // Verify that contract configuration is as expected
    assert_eq!(
        info,
        ContractInfoResponse {
            registered_tokens: 0,
            metadata_uri,
            minter: Some(minter.to_string()),
            owner: Some(owner.to_string()),
            name,
            description
        }
    );
}

#[test]
fn get_config_empty_owner_and_minter() {
    let metadata_uri = String::from("https://metadata_uri_nabla_collection/{ID}.json");
    let name = String::from("nabla collection");
    let description = String::from("This is the official nabla tokens collection");
    let sender = Addr::unchecked("sender");

    let mut app = App::default();

    let code_id = Cw1155::store_code(&mut app);

    let contract = Cw1155::instantiate(
        &mut app,
        code_id,
        &sender,
        "CW1155 nabla collection",
        None,
        &metadata_uri,
        None,
        None,
        &name,
        &description,
    )
    .unwrap();

    // Query the contract configuration
    let info = contract.query_contract_info(&app).unwrap();

    // Verify that contract configuration is as expected
    assert_eq!(
        info,
        ContractInfoResponse {
            registered_tokens: 0,
            metadata_uri,
            minter: Some(sender.to_string()),
            owner: Some(sender.to_string()),
            name,
            description
        }
    );
}
