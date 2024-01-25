use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");

#[cw_serde]
pub struct Config {
    /// Number of tokens that has been registered.
    pub registered_tokens: u128,
    /// Base URI for metadata. It is immutable after the creation.
    pub metadata_uri: String,
    /// When the minter value is changed to Empty, no token can be minted
    /// anymore.
    pub minter: Option<Addr>,
    /// When the owner value is changed to Empty, no token can be registerd
    /// anymore.
    pub owner: Option<Addr>,
    /// A human-readable title for the collection.
    pub name: String,
    /// A description for the collection.
    pub description: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
