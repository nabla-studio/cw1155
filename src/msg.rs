use cosmwasm_schema::{cw_serde, QueryResponses};

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
    Increment {},
    Reset { count: i32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// ContractInfo returns the current information about the contract and the
    /// related tokens collection.
    #[returns(ContractInfoResponse)]
    ContractInfo {},
}
/// ContractInfoResponse holds basic contract information.
#[cw_serde]
pub struct ContractInfoResponse {
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
