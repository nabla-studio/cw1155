use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

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

    #[error("Maximum number of tokens has been reached")]
    MaximumNumberOfTokens,

    #[error("Cannot use zero as maximum supply for a token")]
    ZeroMaxSupply,
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
