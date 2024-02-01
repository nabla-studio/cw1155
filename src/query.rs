use cosmwasm_std::{Deps, StdResult};

use crate::{
    helpers::fetch_balance,
    msg::{BalanceResponse, BatchBalanceResponse, ConfigResponse, IsApprovedForAllResponse},
    state::{approvals, TokenInfo, CONFIG, REGISTERED_TOKENS, TOKENS},
};

// Query contract configuration.
pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    // Load the contract configuration from the storage.
    let config = CONFIG.load(deps.storage)?;
    // Load the number of registered tokens from the storage.
    let registered_tokens = REGISTERED_TOKENS.load(deps.storage)?;

    // Return the configuration response.
    Ok(ConfigResponse {
        registered_tokens,
        metadata_uri: config.metadata_uri,
        minter: config.minter.map(|minter| minter.to_string()),
        owner: config.owner.map(|owner| owner.to_string()),
        name: config.name,
        description: config.description,
    })
}

// Query information about a specific token.
pub fn query_token_info(deps: Deps, id: u64) -> StdResult<TokenInfo> {
    // Load token information by token ID from the storage.
    TOKENS.load(deps.storage, id)
}

// Query the balance of a specific token for a given owner.
pub fn query_balance(deps: Deps, owner: String, id: u64) -> StdResult<BalanceResponse> {
    // Validate the owner address.
    let owner_addr = deps.api.addr_validate(&owner)?;

    // Fetch the balance using the helper function.
    fetch_balance(deps.storage, owner_addr, id)
}

// Query the balance for a batch of tokens for a given owner.
pub fn query_batch_balance(
    deps: Deps,
    owner: String,
    ids: Vec<u64>,
) -> StdResult<BatchBalanceResponse> {
    // Validate the owner address.
    let owner_addr = deps.api.addr_validate(&owner)?;

    // Fetch balances for all IDs, collecting any errors.
    let balances = ids
        .into_iter()
        .map(|id| -> StdResult<_> { fetch_balance(deps.storage, owner_addr.clone(), id) })
        .collect::<StdResult<_>>()?;

    // Return the batch balance response.
    Ok(BatchBalanceResponse { balances })
}

// Query if an operator is approved to manage all of an owner's tokens.
pub fn query_approved_for_all(
    deps: Deps,
    owner: String,
    operator: String,
) -> StdResult<IsApprovedForAllResponse> {
    // Validate the owner and operator addresses.
    let owner_addr = deps.api.addr_validate(&owner)?;
    let operator_addr = deps.api.addr_validate(&operator)?;

    // Check if there is an approval for the operator to manage all of the owner's tokens.
    let expiration = approvals()
        .may_load(deps.storage, (owner_addr.clone(), operator_addr.clone()))?
        .map(|approval| approval.expiration);

    // Return the approval status.
    Ok(IsApprovedForAllResponse { expiration })
}
