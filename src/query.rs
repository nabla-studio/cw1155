use cosmwasm_std::{Deps, StdResult};

use crate::{
    msg::ContractInfoResponse,
    state::{CONFIG, REGISTERED_TOKENS},
};

pub fn query_config(deps: Deps) -> StdResult<ContractInfoResponse> {
    let config = CONFIG.load(deps.storage)?;
    let registered_tokens = REGISTERED_TOKENS.load(deps.storage)?;

    Ok(ContractInfoResponse {
        registered_tokens,
        metadata_uri: config.metadata_uri,
        minter: config.minter.map(|minter| minter.to_string()),
        owner: config.owner.map(|owner| owner.to_string()),
        name: config.name,
        description: config.description,
    })
}
