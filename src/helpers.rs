use cosmwasm_std::{Addr, Env, StdResult, Storage, Uint128};

use crate::{
    msg::BalanceResponse,
    state::{approvals, balances, Balance, TokenInfo, CONFIG, REGISTERED_TOKENS, TOKENS},
    ContractError,
};

/// Assert that an account is the contract's current owner.
pub fn assert_owner(store: &dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let config = CONFIG.load(store)?;

    // the contract must have an owner
    let Some(current_owner) = &config.owner else {
        return Err(ContractError::NoOwner);
    };

    // the sender must be the current owner
    if sender != current_owner {
        return Err(ContractError::NotOwner);
    }

    Ok(())
}

/// Assert that an account is the contract's current minter.
pub fn assert_minter(store: &dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let config = CONFIG.load(store)?;

    // the contract must have a minter
    let Some(current_minter) = &config.minter else {
        return Err(ContractError::NoMinter);
    };

    // the sender must be the current minter
    if sender != current_minter {
        return Err(ContractError::NotMinter);
    }

    Ok(())
}

/// Assert that the operator can manage the tokens.
pub fn assert_can_manage(
    store: &dyn Storage,
    env: &Env,
    owner: Addr,
    operator: Addr,
) -> Result<(), ContractError> {
    // Owner can manage for itself
    if owner == operator {
        return Ok(());
    }

    // Operator can manage only for owners that have already granted him.
    match approvals().may_load(store, (owner, operator))? {
        Some(approval) if !approval.expiration.is_expired(&env.block) => Ok(()),
        _ => Err(ContractError::Unauthorized),
    }
}

/// Possible actions that can be performed on a balance.
pub enum BalanceAction {
    Increase,
    Decrease,
}

// Update the balance (Increase or Decrease) of an account for a token.
pub fn update_balance(
    store: &mut dyn Storage,
    addr: &Addr,
    id: u64,
    amount: Uint128,
    action: BalanceAction,
) -> Result<Balance, ContractError> {
    balances().update(
        store,
        (addr.clone(), id),
        |balance: Option<Balance>| -> Result<_, ContractError> {
            // If the account has no balance for this token, create it.
            let mut new_balance: Balance = balance.unwrap_or_else(|| Balance {
                owner: addr.clone(),
                amount: Uint128::new(0),
                id: id,
            });

            new_balance.amount = match action {
                // Here we do not need to check if the new balance could cause
                // overflow, since this is checked before calling this
                // function.
                BalanceAction::Increase => new_balance.amount + amount,
                BalanceAction::Decrease => {
                    // If the account has no sufficient balance, return an
                    // error.
                    new_balance.amount.checked_sub(amount).map_err(|_| {
                        ContractError::InsufficientFunds {
                            id: id,
                            required: amount,
                            available: new_balance.amount,
                        }
                    })?
                }
            };

            // Save the updated balance.
            Ok(new_balance)
        },
    )
}

/// Increase the number of registered tokens.
pub fn increase_registered_tokens(store: &mut dyn Storage) -> Result<u64, ContractError> {
    REGISTERED_TOKENS.update(store, |tokens_number| -> Result<u64, ContractError> {
        match tokens_number.checked_add(1) {
            Some(new_value) => Ok(new_value),
            None => Err(ContractError::MaximumNumberOfTokens),
        }
    })
}

/// Increase the current supply of a token remaining in the safe range of
/// Uin128 and below the maximum supply (if provided).
pub fn increase_current_supply(
    store: &mut dyn Storage,
    id: u64,
    amount: &Uint128,
) -> Result<TokenInfo, ContractError> {
    // Validates that the amount is not zero.
    if amount.is_zero() {
        return Err(ContractError::InvalidZeroAmount);
    }

    TOKENS.update(
        store,
        id,
        |token_info| -> Result<TokenInfo, ContractError> {
            // Return an error if the token does not yet exist.
            let mut token_info: TokenInfo = token_info.ok_or(ContractError::InvalidToken)?;

            // Increase the current supply of the token.
            token_info.current_supply = token_info.current_supply.checked_add(*amount)?;

            // Calculate the total supply (current_supply + burned).
            let total_supply = token_info.current_supply.checked_add(token_info.burned)?;

            // If a max_supply is set, ensure the total supply does not exceed it.
            if let Some(max_supply) = token_info.max_supply {
                if total_supply > max_supply {
                    return Err(ContractError::CannotExceedMaxSupply);
                }
            }

            // Save the updated token information.
            Ok(token_info)
        },
    )
}

/// Decrease the current supply of a token. It can only happen by burning
/// actions, so the burned supply needs to increase. All of this in the
/// safe range of Uin128.
pub fn decrease_current_supply(
    store: &mut dyn Storage,
    id: u64,
    amount: &Uint128,
) -> Result<TokenInfo, ContractError> {
    // Validates that the amount is not zero.
    if amount.is_zero() {
        return Err(ContractError::InvalidZeroAmount);
    }

    TOKENS.update(
        store,
        id,
        |token_info| -> Result<TokenInfo, ContractError> {
            // Return an error if the token does not yet exist.
            let mut token_info: TokenInfo = token_info.ok_or(ContractError::InvalidToken)?;

            // Decrease the current supply of the token.
            token_info.current_supply =
                token_info
                    .current_supply
                    .checked_sub(*amount)
                    .map_err(|_| ContractError::InsufficientFunds {
                        id: id,
                        required: *amount,
                        available: token_info.current_supply,
                    })?;

            // Increase the burned supply of the token.
            token_info.burned = token_info.burned.checked_add(*amount)?;

            // Save the updated token information.
            Ok(token_info)
        },
    )
}

/// Update the max supply of a token to the actual current_supply. It can only
/// happen to disable the minting operations for the token.
pub fn update_max_supply(store: &mut dyn Storage, id: u64) -> Result<TokenInfo, ContractError> {
    TOKENS.update(
        store,
        id,
        |token_info| -> Result<TokenInfo, ContractError> {
            // Return an error if the token does not yet exist.
            let mut token_info: TokenInfo = token_info.ok_or(ContractError::InvalidToken)?;

            // Increase the current supply of the token.
            token_info.max_supply = Some(token_info.current_supply);

            // Save the updated token information.
            Ok(token_info)
        },
    )
}

/// helper function to get the balance of an account for a token.
pub fn fetch_balance(store: &dyn Storage, owner_addr: Addr, id: u64) -> StdResult<BalanceResponse> {
    // Load the balance of the account for the token.
    let balance = balances()
        .may_load(store, (owner_addr.clone(), id))?
        .unwrap_or(Balance {
            owner: owner_addr,
            id,
            amount: Uint128::new(0),
        });
    // Return the balance
    Ok(BalanceResponse {
        amount: balance.amount,
    })
}
