use cosmwasm_std::{OverflowError, StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Contract ownership has been renounced")]
    NoOwner,

    #[error("Caller is not the contract's current owner")]
    NotOwner,

    #[error("Contract ownership has been renounced")]
    NoMinter,

    #[error("Caller is not the contract's current owner")]
    NotMinter,

    #[error("Token is already registered")]
    AlreadyRegisteredToken,

    #[error("Maximum number of registrable tokens has been reached")]
    MaximumNumberOfTokens,

    #[error("Cannot use zero as maximum supply for a token")]
    ZeroMaxSupply,

    #[error("Invalid zero amount")]
    InvalidZeroAmount,

    #[error("Minting cannot exceed the maximum supply")]
    CannotExceedMaxSupply,

    #[error("Invalid token ID")]
    InvalidToken,

    #[error("User has insufficient balance for token with ID {id}: required {required}, available {available}")]
    InsufficientFunds {
        id: u64,
        required: Uint128,
        available: Uint128,
    },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
