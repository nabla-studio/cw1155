use cosmwasm_std::{Addr, Storage};

use crate::{
    state::{CONFIG, REGISTERED_TOKENS},
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

pub fn increase_registered_tokens(store: &mut dyn Storage) -> Result<u64, ContractError> {
    REGISTERED_TOKENS.update(store, |tokens_number| -> Result<u64, ContractError> {
        match tokens_number.checked_add(1) {
            Some(new_value) => Ok(new_value),
            None => Err(ContractError::MaximumNumberOfTokens),
        }
    })
}
