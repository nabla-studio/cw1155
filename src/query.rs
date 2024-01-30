use cosmwasm_std::{Deps, StdResult, Uint128};

use crate::{
    msg::{BalanceResponse, ContractInfoResponse, IsApprovedForAllResponse},
    state::{approvals, balances, Balance, TokenInfo, CONFIG, REGISTERED_TOKENS, TOKENS},
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

pub fn query_token_info(deps: Deps, id: u64) -> StdResult<TokenInfo> {
    TOKENS.load(deps.storage, id)
}

pub fn query_balance(deps: Deps, owner: String, id: u64) -> StdResult<BalanceResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let balance = balances()
        .may_load(deps.storage, (owner_addr.clone(), id))?
        .unwrap_or(Balance {
            owner: owner_addr,
            id,
            amount: Uint128::new(0),
        });
    Ok(BalanceResponse {
        amount: balance.amount,
    })
}

pub fn query_approved_for_all(
    deps: Deps,
    owner: String,
    operator: String,
) -> StdResult<IsApprovedForAllResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let operator_addr = deps.api.addr_validate(&operator)?;

    let expiration = approvals()
        .may_load(deps.storage, (owner_addr.clone(), operator_addr.clone()))?
        .map(|approval| approval.expiration);

    Ok(IsApprovedForAllResponse { expiration })
}
