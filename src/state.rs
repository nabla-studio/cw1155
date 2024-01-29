use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
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

pub const REGISTERED_TOKENS: Item<u64> = Item::new("registered_tokens");

#[cw_serde]
pub struct TokenInfo {
    /// Boolean flag indicating whether token is transferrable after minting or
    /// not.
    pub is_transferrable: bool,
    /// Maximum number of elements of tokens that can be minted.
    pub max_supply: Option<Uint128>,
    /// Number of elements of tokens that have been burned.
    pub burned: Uint128,
    /// Number of elements of tokens that have been minted minus number of
    /// elements of tokens that have been burned.
    pub current_supply: Uint128,
}

/// There is no Vec structure available in cw-storage-plus. We had a look
/// around and it seems that also when the key is an integer, a Map is used.
/// https://github.com/DA0-DA0/dao-contracts/blob/bc3a44983c1bbad48d12436353a95180489143e8/contracts/proposal/dao-proposal-single/src/state.rs#L71
/// TODO: Understand why no Vec is available in cw-storage-plus and if
/// using a Map is the right way to go.
pub const TOKENS: Map<u64, TokenInfo> = Map::new("tokens");
