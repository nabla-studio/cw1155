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
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
