#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, REGISTERED_TOKENS};
use crate::{execute, query};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw1155_nabla";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // If the minter is not passed to the instantiate function, use the sender
    // address as the minter one. Otherwise, validate the minter address.
    let minter = msg
        .minter
        .map_or(Ok(info.sender.clone()), |m| deps.api.addr_validate(&m))?;

    // If the owner is not passed to the instantiate function, use the sender
    // address as the owner one. Otherwise, validate the owner address.
    let owner = msg
        .owner
        .map_or(Ok(info.sender), |o| deps.api.addr_validate(&o))?;

    let config = Config {
        metadata_uri: msg.metadata_uri,
        minter: Some(minter),
        owner: Some(owner),
        name: msg.name,
        description: msg.description,
    };

    // At the beginning minter and owner can not be empty in the state, since
    // in this case the contract cannot be used. No token can be registered
    // and/or no token can be minted.

    REGISTERED_TOKENS.save(deps.storage, &0)?;
    CONFIG.save(deps.storage, &config)?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => execute::increment(deps),
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ContractInfo {} => to_json_binary(&query::query_config(deps)?),
    }
}
