use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};
use cw_utils::Expiration;

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

    /// Burn an already minted token.
    Burn {
        /// Address of the owner of the token to burn.
        from: String,
        /// ID of the token to burn.
        id: u64,
        /// Amount token elements to burn.
        amount: Uint128,
    },

    /// Transfer an already minted token.
    TransferFrom {
        /// Address of the owner of the token to transfer.
        from: String,
        /// Address of the recipient.
        to: String,
        /// ID of the token to transfer.
        id: u64,
        /// Amount token elements to transfer.
        amount: Uint128,
        /// Message for smart contract recipients.
        msg: Option<Binary>,
    },

    /// Approve a grants to the operator to operate on all tokens owned by the
    /// sender of the request.
    ApproveAll {
        /// Address of the operator.
        operator: String,
        /// Expiration of the grant.
        expiration: Option<Expiration>,
    },

    /// Revoke a previously approved grant to the operator to operate on all
    /// tokens owned by the sender of the request.
    RevokeAll {
        /// Address of the operator.
        operator: String,
    },

    /// Modify the address of who can perform minting operations.
    /// None value indicates that the minting operation are disabled.
    /// NOTE: Once disabled, the minter address cannot be changed.
    SetMinter { minter: Option<String> },

    /// Disable the minting operations for a specific token ID.
    DisableTokenMinting {
        /// ID of the token for which disable the minting operations.
        id: u64,
    },

    /// Modify the address of who can perform registering operations.
    /// None value indicates that the registering operation are disabled.
    /// NOTE: Once disabled, the owner address cannot be changed.
    SetOwner { owner: Option<String> },

    /// Modify the details (name and the description) of the collection.
    UpdateCollectionDetails { name: String, description: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Balance returns the amount in the balance for an owner and a specific
    /// token.
    #[returns(BalanceResponse)]
    Balance { owner: String, id: u64 },

    /// BatchBalance returns the amounts in the balances for an owner and a
    /// specific list of tokens.
    #[returns(BatchBalanceResponse)]
    BatchBalance { owner: String, ids: Vec<u64> },

    /// IsApprovedForAll returns if an operator is approved for managing all
    /// the tokens owned by an owner.
    #[returns(BalanceResponse)]
    IsApprovedForAll { owner: String, operator: String },

    /// Config returns the current information about the contract and the
    /// related tokens collection.
    #[returns(ConfigResponse)]
    Config {},

    /// TokenInfo returns the information about a specific token.
    #[returns(TokenInfo)]
    TokenInfo { id: u64 },
}
/// ConfigResponse holds basic contract information.
#[cw_serde]
pub struct ConfigResponse {
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

/// BatchBalanceResponse holds the amounts of the balances.
#[cw_serde]
pub struct BatchBalanceResponse {
    pub balances: Vec<BalanceResponse>,
}

/// IsApprovedForAllResponse holds the expiration indicating (if exists) the
/// expiring date for the grant.
#[cw_serde]
pub struct IsApprovedForAllResponse {
    // None implies that the grant is not present.
    pub expiration: Option<Expiration>,
}
