use cosmwasm_std::{Addr, Deps, Order, StdResult, Uint128};
use cw_storage_plus::Bound;
use cw_utils::Expiration;

use crate::{
    helpers::fetch_balance,
    msg::ConfigResponse,
    state::{approvals, Approval, TokenInfo, CONFIG, REGISTERED_TOKENS, TOKENS},
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
pub fn query_token_info(deps: Deps, id: u64) -> Result<TokenInfo, ContractError> {
    // Load token information by token ID from the storage.
    match TOKENS.may_load(deps.storage, id)? {
        Some(token_info) => Ok(token_info),
        None => return Err(ContractError::InvalidToken { id }),
    }
}

// Query the balance of a specific token for a given owner.
pub fn query_balance(deps: Deps, owner: String, id: u64) -> Result<Uint128, ContractError> {
    // Validate the owner address.
    let owner_addr = deps.api.addr_validate(&owner)?;

    // Return invalid token if the token is not registered.
    let max_token_id = REGISTERED_TOKENS.load(deps.storage)?;
    if id > max_token_id {
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
    let expiration = approvals()
        .may_load(deps.storage, (owner_addr.clone(), operator_addr.clone()))?
        .map(|approval| approval.expiration);

    // Return the approval status.
    Ok(expiration)
}

// Query te list of operators approved to manage all of an owner's tokens.
pub fn query_approvals_by_owner(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<Approval>> {
    // Validate the owner's address to ensure it's in a proper format.
    let owner_addr = deps.api.addr_validate(&owner)?;

    // Determine the start address for the query. This is used for pagination.
    let start_addr = match start_after {
        Some(addr) => {
            // If a start_after address is provided, validate it.
            let validated_addr = deps.api.addr_validate(&addr)?;
            // Use the validated address as the starting point.
            Some(validated_addr)
        }
        // If no start_after address is provided, the query will start from the beginning.
        None => None,
    };

    // Set a limit for the number of results.
    // If the limit is not specified, use DEFAULT_LIMIT. Ensure it does not exceed MAX_LIMIT.
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    // Create a bound for the query based on the owner's address and the start address.
    let start = start_addr.map(|addr| Bound::<(Addr, Addr)>::exclusive((owner_addr.clone(), addr)));

    // Execute the query:
    // - Use the owner_index to find all approvals for the owner.
    // - Start the query at the 'start' address if provided.
    // - Order the results in ascending order.
    // - Limit the number of results to the specified limit.
    // - Convert each result into an Approval, handling any potential errors.
    let res: Vec<Approval> = approvals()
        .idx
        .owner_index
        .prefix(owner_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .flat_map(|vc| Ok::<Approval, ContractError>(vc?.1))
        .collect();

    // Return the list of approvals.
    Ok(res)
}
