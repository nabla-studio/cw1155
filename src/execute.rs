// Import necessary dependencies.
use cosmwasm_std::{Addr, Binary, DepsMut, Env, MessageInfo, Response, Uint128};
use cw_utils::Expiration;

use crate::{
    helpers::{
        assert_can_manage, assert_minter, assert_owner, decrease_current_supply,
        increase_current_supply, increase_registered_tokens, update_balance, update_max_supply,
        BalanceAction,
    },
    receiver::Cw1155ReceiveMsg,
    state::{approvals, Approval, TokenInfo, CONFIG, TOKENS},
    ContractError,
};

// Registers a new token type with optional max_supply and transferability.
pub fn register(
    deps: DepsMut,
    info: MessageInfo,
    max_supply: Option<Uint128>,
    is_transferrable: Option<bool>,
) -> Result<Response, ContractError> {
    // Ensures that the message sender is the owner of the contract.
    assert_owner(deps.storage, &info.sender)?;

    // Increases the number of registered tokens and retrieves the new token ID.
    let next_id = increase_registered_tokens(deps.storage)?;

    // Ensures max_supply is not zero if provided.
    let max_supply = match max_supply {
        Some(supply) if supply > Uint128::zero() => Some(supply),
        Some(_) => return Err(ContractError::ZeroMaxSupply),
        None => None,
    };

    // Creates a new token with the provided details.
    let token = TokenInfo {
        is_transferrable: is_transferrable.unwrap_or(true),
        max_supply,
        burned: Uint128::zero(),
        current_supply: Uint128::zero(),
    };

    // The following condition cannot happen, since the token id is
    // automatically incremented by the contract
    // TOKENS.update(
    //     deps.storage,
    //     next_id,
    //     |existing_token| match existing_token {
    //         Some(_) => Err(ContractError::AlreadyRegisteredToken {}),
    //         None => Ok(token),
    //     },
    // )?;

    // Saves the newly created token to the TOKENS storage.
    TOKENS.save(deps.storage, next_id, &token)?;

    // Prepares the response with the action and id attributes.
    let resp = Response::default()
        .add_attribute("action", "register")
        .add_attribute("id", next_id.to_string());

    Ok(resp)
}

// Mints new tokens of a specific type and sends them to a specified address.
pub fn mint(
    deps: DepsMut,
    info: MessageInfo,
    to: String,
    id: u64,
    amount: Uint128,
    msg: Option<Binary>,
) -> Result<Response, ContractError> {
    // Ensures that the message sender is a minter.
    assert_minter(deps.storage, &info.sender)?;

    // Validates the receiver's address.
    let to_addr = deps.api.addr_validate(&to)?;

    // Increase the current supply of the token.
    increase_current_supply(deps.storage, id, &amount)?;

    // Executes the transfer of the minted tokens to the specified address.
    exec_transfer(deps, None, Some(to_addr), id, amount)?;

    // Initialize a response with basic minting attributes.
    let mut resp = Response::default()
        .add_attribute("action", "mint")
        .add_attribute("to", &to)
        .add_attribute("id", id.to_string())
        .add_attribute("amount", amount);

    // If a message is provided, create a Cw1155ReceiveMsg and add it to the response.
    if let Some(msg) = msg {
        resp = resp.add_message(
            Cw1155ReceiveMsg {
                operator: info.sender.to_string(),
                from: None,
                id,
                amount,
                msg,
            }
            .into_cosmos_msg(to)?,
        );
    }

    Ok(resp)
}

//  Burns a specified quantity of a token from a holder's address.
pub fn burn(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    from: String,
    id: u64,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Validates the sender's address.
    let from_addr = deps.api.addr_validate(&from)?;

    // Ensures that the message sender can manage the tokens.
    assert_can_manage(deps.storage, &env, from_addr.clone(), info.sender)?;

    // Decrease the current supply of the token.
    decrease_current_supply(deps.storage, id, &amount)?;

    // Executes the transfer of the minted tokens to the specified address.
    exec_transfer(deps, Some(from_addr), None, id, amount)?;

    // Initialize a response with basic minting attributes.
    let resp = Response::default()
        .add_attribute("action", "burn")
        .add_attribute("from", &from)
        .add_attribute("id", id.to_string())
        .add_attribute("amount", amount);

    Ok(resp)
}

// Internal function to handle the transfer of tokens.
// From None: Mint
// To None: Burn
// Ensure permissions are checked before calling this.
fn exec_transfer(
    deps: DepsMut,
    from: Option<Addr>,
    to: Option<Addr>,
    id: u64,
    amount: Uint128,
) -> Result<(), ContractError> {
    // Validates that the amount is not zero.
    if amount.is_zero() {
        return Err(ContractError::InvalidZeroAmount);
    }

    // Handles decrementing the balance from the sender if specified.
    if let Some(from_addr) = &from {
        update_balance(deps.storage, from_addr, id, amount, BalanceAction::Decrease)?;
    }

    // Handles incrementing the balance to the receiver if specified.
    if let Some(to_addr) = &to {
        update_balance(deps.storage, to_addr, id, amount, BalanceAction::Increase)?;
    }

    // If everything went well, returns an empty response.
    Ok(())
}

/// Grants the operator to operate on all tokens owned by the info.sender.
pub fn approve_all(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    operator: String,
    expiration: Option<Expiration>,
) -> Result<Response, ContractError> {
    // Retrieves the expiration.
    let expiration = expiration.unwrap_or_default();

    // Validate the operator's address.
    let operator_addr = deps.api.addr_validate(&operator)?;

    // Check if expiration already expired.
    if expiration.is_expired(&env.block) {
        return Err(ContractError::Expired);
    }

    // Save the new approval.
    approvals().save(
        deps.storage,
        (info.sender.clone(), operator_addr.clone()),
        &Approval {
            owner: info.sender.clone(),
            operator: operator_addr,
            expiration,
        },
    )?;

    // Prepare the response.
    let resp = Response::new()
        .add_attribute("action", "approve_all")
        .add_attribute("owner", &info.sender)
        .add_attribute("operator", &operator)
        .add_attribute("expiration", expiration.to_string());

    Ok(resp)
}

/// Revoke a grants to the operator to operate on all tokens owned by the
/// info.sender.
pub fn revoke_all(
    deps: DepsMut,
    info: MessageInfo,
    operator: String,
) -> Result<Response, ContractError> {
    // Validate the operator's address.
    let operator_addr = deps.api.addr_validate(&operator)?;

    // Remove a previous grant.
    approvals().remove(deps.storage, (info.sender.clone(), operator_addr))?;

    // Prepare the response.
    let resp = Response::default()
        .add_attribute("action", "revoke_all")
        .add_attribute("owner", &info.sender)
        .add_attribute("operator", &operator);

    Ok(resp)
}

/// Modify the address of who can perform minting operations.
pub fn set_minter(
    deps: DepsMut,
    info: MessageInfo,
    minter: Option<String>,
) -> Result<Response, ContractError> {
    // Retrieve the configuration information.
    let mut config = CONFIG.load(deps.storage)?;

    // Ensures that the message sender is a minter.
    assert_minter(deps.storage, &info.sender)?;

    // Check the new minter's address and use it if valid.
    config.minter = match minter.clone() {
        Some(minter) => Some(deps.api.addr_validate(&minter)?),
        None => None,
    };

    // Saves the newly updated token configuration.
    CONFIG.save(deps.storage, &config)?;

    // Prepare the response.
    let resp = Response::default()
        .add_attribute("action", "update_minter")
        .add_attribute("new_minter", minter.unwrap_or_else(|| "None".to_string()));

    Ok(resp)
}

/// Disable the ability to mint new tokens for a specific id.
pub fn disable_token_minting(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    // Ensures that the message sender is the minter.
    assert_minter(deps.storage, &info.sender)?;

    // Update the max supply of the token to the current one.
    update_max_supply(deps.storage, id)?;

    // Prepare the response.
    let resp = Response::default()
        .add_attribute("action", "disable_token_minting")
        .add_attribute("id", id.to_string());

    Ok(resp)
}
