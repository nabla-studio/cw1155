use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::App;
use cw_utils::Expiration;

use crate::{
    msg::{ContractInfoResponse, IsApprovedForAllResponse},
    state::TokenInfo,
    ContractError,
};

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
        .register(&mut app, &sender, Uint128::from(1000u128), true)
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
        .register(&mut app, &sender, Uint128::from(1u128), false)
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
        .register(&mut app, &sender, Uint128::from(0u128), None)
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

    contract.register(&mut app, &sender, None, None).unwrap();

    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

    assert_eq!(token_count, 1);
}

#[test]
fn unauthorized_register() {
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
        .register(&mut app, &sender, None, None)
        .unwrap_err();

    assert_eq!(err, ContractError::NotOwner {});

    let token_count = contract
        .query_contract_info(&app)
        .unwrap()
        .registered_tokens;

    assert_eq!(token_count, 0);
}

#[test]
fn mint() {
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

    contract.register(&mut app, &sender, None, None).unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    contract
        .mint(
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

    contract.register(&mut app, &sender, None, None).unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    let err = contract
        .mint(
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
        .register(&mut app, &sender, Uint128::from(20u128), None)
        .unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    let err = contract
        .mint(
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
        .register(&mut app, &sender, Uint128::from(20u128), None)
        .unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    contract
        .mint(
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
        .mint(
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

    let err = contract
        .mint(
            &mut app,
            &sender,
            recipient.as_str(),
            1,
            Uint128::from(10u128),
            None,
        )
        .unwrap_err();

    assert_eq!(err, ContractError::CannotExceedMaxSupply);
}

#[test]
fn mint_multiple_recipients() {
    let sender = Addr::unchecked("sender");
    let recipient1 = Addr::unchecked("recipient1");
    let recipient2 = Addr::unchecked("recipient2");

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
        .register(&mut app, &sender, Uint128::from(20u128), None)
        .unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    contract
        .mint(
            &mut app,
            &sender,
            recipient1.as_str(),
            1,
            Uint128::from(8u128),
            None,
        )
        .unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::from(8u128));

    contract
        .mint(
            &mut app,
            &sender,
            recipient2.as_str(),
            1,
            Uint128::from(12u128),
            None,
        )
        .unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::from(20u128));

    let recipient1_balance = contract
        .query_balance(&app, recipient1.into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(recipient1_balance, Uint128::from(8u128));

    let recipient2_balance = contract
        .query_balance(&app, recipient2.into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(recipient2_balance, Uint128::from(12u128));
}

#[test]
fn alice_approve_bob() {
    let sender = Addr::unchecked("sender");
    let alice = Addr::unchecked("alice");
    let bob = Addr::unchecked("bob");

    let mut app = App::default();

    let start_time = 12_345;

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
        .approve_all(
            &mut app,
            &alice,
            bob.as_str(),
            Some(Expiration::AtHeight(start_time + 1)),
        )
        .unwrap();

    let bob_alice_approvals = contract
        .query_is_approved_for_all(&app, alice.into_string(), bob.clone().into_string())
        .unwrap();

    assert_eq!(
        bob_alice_approvals,
        IsApprovedForAllResponse {
            expiration: Some(Expiration::AtHeight(start_time + 1))
        }
    );

    let sender_alice_approvals = contract
        .query_is_approved_for_all(&app, sender.into_string(), bob.into_string())
        .unwrap();

    assert_eq!(
        sender_alice_approvals,
        IsApprovedForAllResponse { expiration: None }
    );
}

#[test]
fn expired_approve() {
    let sender = Addr::unchecked("sender");
    let alice = Addr::unchecked("alice");
    let bob = Addr::unchecked("bob");

    let mut app = App::default();

    let start_time = 12_345;

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
        .approve_all(
            &mut app,
            &alice,
            bob.as_str(),
            Some(Expiration::AtHeight(start_time - 1)),
        )
        .unwrap_err();

    assert_eq!(err, ContractError::Expired);

    let bob_alice_approvals = contract
        .query_is_approved_for_all(&app, alice.into_string(), bob.clone().into_string())
        .unwrap();

    assert_eq!(
        bob_alice_approvals,
        IsApprovedForAllResponse { expiration: None }
    );
}

#[test]
fn alice_multiple_approve() {
    let sender = Addr::unchecked("sender");
    let alice = Addr::unchecked("alice");
    let bob = Addr::unchecked("bob");

    let mut app = App::default();

    let start_time = 12_345;

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
        .approve_all(
            &mut app,
            &alice,
            bob.as_str(),
            Some(Expiration::AtHeight(start_time + 1)),
        )
        .unwrap();

    contract
        .approve_all(
            &mut app,
            &alice,
            sender.as_str(),
            Some(Expiration::Never {}),
        )
        .unwrap();

    let bob_alice_approvals = contract
        .query_is_approved_for_all(&app, alice.clone().into_string(), bob.clone().into_string())
        .unwrap();

    assert_eq!(
        bob_alice_approvals,
        IsApprovedForAllResponse {
            expiration: Some(Expiration::AtHeight(start_time + 1))
        }
    );

    let sender_alice_approvals = contract
        .query_is_approved_for_all(&app, alice.into_string(), sender.into_string())
        .unwrap();

    assert_eq!(
        sender_alice_approvals,
        IsApprovedForAllResponse {
            expiration: Some(Expiration::Never {})
        }
    );
}

#[test]
fn alice_revoke() {
    let sender = Addr::unchecked("sender");
    let alice = Addr::unchecked("alice");
    let bob = Addr::unchecked("bob");

    let mut app = App::default();

    let start_time = 12_345;

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
        .approve_all(
            &mut app,
            &alice,
            bob.as_str(),
            Some(Expiration::AtHeight(start_time + 1)),
        )
        .unwrap();

    contract
        .approve_all(
            &mut app,
            &alice,
            sender.as_str(),
            Some(Expiration::Never {}),
        )
        .unwrap();

    let bob_alice_approvals = contract
        .query_is_approved_for_all(&app, alice.clone().into_string(), bob.clone().into_string())
        .unwrap();

    assert_eq!(
        bob_alice_approvals,
        IsApprovedForAllResponse {
            expiration: Some(Expiration::AtHeight(start_time + 1))
        }
    );

    let sender_alice_approvals = contract
        .query_is_approved_for_all(
            &app,
            alice.clone().into_string(),
            sender.clone().into_string(),
        )
        .unwrap();

    assert_eq!(
        sender_alice_approvals,
        IsApprovedForAllResponse {
            expiration: Some(Expiration::Never {})
        }
    );

    contract
        .revoke_all(&mut app, &alice, sender.as_str())
        .unwrap();

    let bob_alice_approvals = contract
        .query_is_approved_for_all(&app, alice.clone().into_string(), bob.clone().into_string())
        .unwrap();

    assert_eq!(
        bob_alice_approvals,
        IsApprovedForAllResponse {
            expiration: Some(Expiration::AtHeight(start_time + 1))
        }
    );

    let sender_alice_approvals = contract
        .query_is_approved_for_all(&app, alice.into_string(), sender.into_string())
        .unwrap();

    assert_eq!(
        sender_alice_approvals,
        IsApprovedForAllResponse { expiration: None }
    );
}

#[test]
fn burn() {
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

    contract.register(&mut app, &sender, None, None).unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    contract
        .mint(
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
        .query_balance(&app, recipient.clone().into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(user_balance, Uint128::from(10u128));

    contract
        .burn(
            &mut app,
            &recipient,
            recipient.as_str(),
            1,
            Uint128::from(3u128),
        )
        .unwrap();

    let TokenInfo {
        current_supply,
        burned,
        ..
    } = contract.query_token_info(&app, 1).unwrap();
    assert_eq!(current_supply, Uint128::from(7u128));
    assert_eq!(burned, Uint128::from(3u128));

    let user_balance = contract
        .query_balance(&app, recipient.clone().into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(user_balance, Uint128::from(7u128));
}

#[test]
fn unauthorized_burn() {
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

    contract.register(&mut app, &sender, None, None).unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    contract
        .mint(
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
        .query_balance(&app, recipient.clone().into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(user_balance, Uint128::from(10u128));

    let err = contract
        .burn(
            &mut app,
            &sender,
            recipient.as_str(),
            1,
            Uint128::from(3u128),
        )
        .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized);

    let TokenInfo {
        current_supply,
        burned,
        ..
    } = contract.query_token_info(&app, 1).unwrap();
    assert_eq!(current_supply, Uint128::from(10u128));
    assert_eq!(burned, Uint128::zero());

    let user_balance = contract
        .query_balance(&app, recipient.clone().into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(user_balance, Uint128::from(10u128));
}

#[test]
fn insufficient_burn() {
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

    contract.register(&mut app, &sender, None, None).unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    let err = contract
        .burn(&mut app, &sender, sender.as_str(), 1, Uint128::from(3u128))
        .unwrap_err();

    assert_eq!(
        err,
        ContractError::InsufficientFunds {
            id: 1,
            required: Uint128::from(3u128),
            available: Uint128::zero()
        }
    );
}

#[test]
fn approved_burn() {
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

    contract.register(&mut app, &sender, None, None).unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    contract
        .mint(
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
        .query_balance(&app, recipient.clone().into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(user_balance, Uint128::from(10u128));

    contract
        .approve_all(
            &mut app,
            &recipient,
            sender.as_str(),
            Some(Expiration::Never {}),
        )
        .unwrap();

    contract
        .burn(
            &mut app,
            &sender,
            recipient.as_str(),
            1,
            Uint128::from(3u128),
        )
        .unwrap();

    let TokenInfo {
        current_supply,
        burned,
        ..
    } = contract.query_token_info(&app, 1).unwrap();
    assert_eq!(current_supply, Uint128::from(7u128));
    assert_eq!(burned, Uint128::from(3u128));

    let user_balance = contract
        .query_balance(&app, recipient.clone().into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(user_balance, Uint128::from(7u128));
}

#[test]
fn too_much_to_burn() {
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

    contract.register(&mut app, &sender, None, None).unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::zero());

    contract
        .mint(
            &mut app,
            &sender,
            recipient.as_str(),
            1,
            Uint128::from(10u128),
            None,
        )
        .unwrap();

    contract
        .mint(
            &mut app,
            &sender,
            sender.as_str(),
            1,
            Uint128::from(40u128),
            None,
        )
        .unwrap();

    let current_supply = contract.query_token_info(&app, 1).unwrap().current_supply;
    assert_eq!(current_supply, Uint128::from(50u128));

    let user_balance = contract
        .query_balance(&app, recipient.clone().into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(user_balance, Uint128::from(10u128));

    let err = contract
        .burn(
            &mut app,
            &recipient,
            recipient.as_str(),
            1,
            Uint128::from(30u128),
        )
        .unwrap_err();

    assert_eq!(
        err,
        ContractError::InsufficientFunds {
            id: 1,
            required: Uint128::from(30u128),
            available: Uint128::from(10u128)
        }
    );

    let TokenInfo {
        current_supply,
        burned,
        ..
    } = contract.query_token_info(&app, 1).unwrap();
    assert_eq!(current_supply, Uint128::from(50u128));
    assert_eq!(burned, Uint128::zero());

    let user_balance = contract
        .query_balance(&app, recipient.clone().into_string(), 1)
        .unwrap()
        .amount;

    assert_eq!(user_balance, Uint128::from(10u128));
}
