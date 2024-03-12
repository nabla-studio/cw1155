use cosmwasm_std::{Deps, Order, StdResult, Uint128};
use cw_storage_plus::Bound;
use cw_utils::Expiration;

use crate::{
    helpers::fetch_balance,
    msg::{ConfigResponse, TokenInfoResponse},
    state::{APPROVALS, CONFIG, REGISTERED_TOKENS, TOKENS},
    ContractError,
};

pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 50;

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
pub fn query_token_info(deps: Deps, id: u64) -> Result<TokenInfoResponse, ContractError> {
    // Load token information by token ID from the storage.
    match TOKENS.may_load(deps.storage, id)? {
        Some(token_info) => Ok(TokenInfoResponse {
            id,
            info: token_info,
        }),
        None => Err(ContractError::InvalidToken { id }),
    }
}

// Query information about a list of token iterated from start_after.
pub fn query_tokens_info(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<Vec<TokenInfoResponse>> {
    // Create a bound for the query based on the tokens start id.
    let start = start_after.map(Bound::<u64>::exclusive);

    // Set a limit for the number of results.
    // If the limit is not specified, use DEFAULT_LIMIT. Ensure it does not exceed MAX_LIMIT.
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    // Load token information by token ID for the tokens list from the storage.
    let res = TOKENS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .filter_map(|result| result.ok())
        .map(|(id, info)| TokenInfoResponse { id, info })
        .collect::<Vec<_>>();

    Ok(res)
}

// Query the balance of a specific token for a given owner.
pub fn query_balance(deps: Deps, owner: String, id: u64) -> Result<Uint128, ContractError> {
    // Validate the owner address.
    let owner_addr = deps.api.addr_validate(&owner)?;

    // Return invalid token if the token is not registered.
    if !TOKENS.has(deps.storage, id) {
        return Err(ContractError::InvalidToken { id });
    }

    // Fetch the balance using the helper function.
    fetch_balance(deps.storage, owner_addr, id)
}

// Query the balance for a batch of tokens for a given owner.
pub fn query_batch_balance(
    deps: Deps,
    owner: String,
    ids: Vec<u64>,
) -> Result<Vec<Uint128>, ContractError> {
    // Validate the owner address.
    let owner_addr = deps.api.addr_validate(&owner)?;

    // Return invalid token if the token is not registered.
    let max_token_id = REGISTERED_TOKENS.load(deps.storage)?;

    // Fetch balances for all IDs, collecting any errors.
    let balances = ids
        .into_iter()
        .map(|id| -> Result<_, ContractError> {
            if id > max_token_id {
                return Err(ContractError::InvalidToken { id });
            }
            fetch_balance(deps.storage, owner_addr.clone(), id)
        })
        .collect::<Result<_, ContractError>>()?;

    // Return the batch balance response.
    Ok(balances)
}

// Query if an operator is approved to manage all of an owner's tokens.
pub fn query_approved_for_all(
    deps: Deps,
    owner: String,
    operator: String,
) -> StdResult<Option<Expiration>> {
    // Validate the owner and operator addresses.
    let owner_addr = deps.api.addr_validate(&owner)?;
    let operator_addr = deps.api.addr_validate(&operator)?;

    // Check if there is an approval for the operator to manage all of the owner's tokens.
    let expiration =
        APPROVALS.may_load(deps.storage, (owner_addr.clone(), operator_addr.clone()))?;

    // Return the approval status.
    Ok(expiration)
}
