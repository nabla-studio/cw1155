use cosmwasm_std::{DepsMut, MessageInfo, Response, Uint128};

use crate::{
    helpers::{assert_owner, increase_registered_tokens},
    state::{TokenInfo, TOKENS},
    ContractError,
};

pub fn register(
    deps: DepsMut,
    info: MessageInfo,
    max_supply: Option<Uint128>,
    is_transferrable: Option<bool>,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;

    // Increase the number of registered tokens and retrieve the new token id.
    let next_token_id = increase_registered_tokens(deps.storage)?;

    // Ensure max_supply is not zero if provided.
    let max_supply = match max_supply {
        Some(supply) if supply > Uint128::zero() => Some(supply),
        Some(_) => return Err(ContractError::ZeroMaxSupply),
        None => None,
    };

    // Create the token.
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
    //     next_token_id,
    //     |existing_token| match existing_token {
    //         Some(_) => Err(ContractError::AlreadyRegisteredToken {}),
    //         None => Ok(token),
    //     },
    // )?;

    // For this reason, we can directly use the following code
    TOKENS.save(deps.storage, next_token_id, &token)?;

    let resp = Response::default()
        .add_attribute("action", "register")
        .add_attribute("token_id", next_token_id.to_string());

    Ok(resp)
}
