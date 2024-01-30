// Import necessary dependencies.
use cosmwasm_std::{Addr, Binary, DepsMut, MessageInfo, Response, Uint128};

use crate::{
    helpers::{
        assert_minter, assert_owner, increase_registered_tokens, increment_current_supply,
        update_balance, BalanceAction,
    },
    receiver::Cw1155ReceiveMsg,
    state::{TokenInfo, TOKENS},
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
    // Validates the receiver's address.
    let to_addr = deps.api.addr_validate(&to)?;

    // Ensures that the message sender is a minter.
    assert_minter(deps.storage, &info.sender)?;

    // Increment the current supply of the token.
    increment_current_supply(deps.storage, id, &amount)?;

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
