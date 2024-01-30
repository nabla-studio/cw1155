use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};

use crate::state::TokenInfo;

#[cw_serde]
pub struct InstantiateMsg {
    /// Base URI for metadata. It supports ID substitution by clients. Clients
    /// replace `{id}` within any URI with the actual token ID.
    /// It is immutable post-creation.
    pub metadata_uri: String,
    /// The minter is the only account that can perform minting operations.
    /// If no value is passed, the default one is the message sender.
    /// Only the current minter can change the minter address.
    pub minter: Option<String>,

    /// The owner is the only account that can register new tokens.
    /// If no value is passed, the default one is the message sender.
    /// Only the current owner can change the owner address.
    pub owner: Option<String>,

    /// The name of the collection.
    pub name: String,
    /// The description of the collection.
    pub description: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Register a new token.
    Register {
        /// Maximum number of elements of tokens that can be minted.
        max_supply: Option<Uint128>,
        /// Flag indicating whether token is transferrable after minting or not.
        is_transferrable: Option<bool>,
    },

    /// Mint an already registered token.
    Mint {
        /// Address of the recipient.
        to: String,
        /// ID of the token to mint.
        id: u64,
        /// Amount token elements to mint.
        amount: Uint128,
        /// Message for smart contract recipients.
        msg: Option<Binary>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// ContractInfo returns the current information about the contract and the
    /// related tokens collection.
    #[returns(ContractInfoResponse)]
    ContractInfo {},

    /// TokenInfo returns the information about a specific token.
    #[returns(TokenInfo)]
    TokenInfo { id: u64 },

    /// Balance returns the amount in the balance for an owner and a specific
    /// token.
    #[returns(BalanceResponse)]
    Balance { owner: String, id: u64 },
}
/// ContractInfoResponse holds basic contract information.
#[cw_serde]
pub struct ContractInfoResponse {
    /// Number of registered tokens.
    pub registered_tokens: u64,

    /// Base URI for contract metadata.
    pub metadata_uri: String,

    /// Authorized minter address. None implies minting is disabled.
    pub minter: Option<String>,

    /// Authorized owner address. None implies changes are locked.
    pub owner: Option<String>,

    /// Collection title.
    pub name: String,

    /// Collection description.
    pub description: String,
}

/// BalanceResponse holds the amount of a balance.
#[cw_serde]
pub struct BalanceResponse {
    pub amount: Uint128,
}
