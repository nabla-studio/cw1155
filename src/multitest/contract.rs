use cosmwasm_std::{Addr, Binary, StdResult, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};
use cw_utils::Expiration;

use crate::contract::{execute, instantiate, query};
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TokenInfoResponse};
use crate::state::{Approval, Balance};
use crate::ContractError;

pub struct Cw1155(Addr);

impl Cw1155 {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate<'a>(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        admin: impl Into<Option<&'a Addr>>,
        metadata_uri: &str,
        minter: impl Into<Option<&'a Addr>>,
        owner: impl Into<Option<&'a Addr>>,
        name: &str,
        description: &str,
    ) -> StdResult<Self> {
        let admin = admin.into();
        let minter = minter.into();
        let owner = owner.into();

        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
                metadata_uri: metadata_uri.to_string(),
                minter: minter.map(Addr::to_string),
                owner: owner.map(Addr::to_string),
                name: name.to_string(),
                description: description.to_string(),
            },
            &[],
            label,
            admin.map(Addr::to_string),
        )
        .map(Cw1155)
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn register(
        &self,
        app: &mut App,
        sender: &Addr,
        max_supply: impl Into<Option<Uint128>>,
        is_transferable: impl Into<Option<bool>>,
    ) -> Result<(), ContractError> {
        let max_supply = max_supply.into();
        let is_transferable = is_transferable.into();

        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::Register {
                max_supply,
                is_transferable,
            },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn mint(
        &self,
        app: &mut App,
        sender: &Addr,
        to: &str,
        id: u64,
        amount: Uint128,
        msg: impl Into<Option<Binary>>,
    ) -> Result<(), ContractError> {
        let msg = msg.into();

        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::Mint {
                to: to.to_string(),
                id,
                amount,
                msg,
            },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn burn(
        &self,
        app: &mut App,
        sender: &Addr,
        from: &str,
        id: u64,
        amount: Uint128,
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::Burn {
                from: from.to_string(),
                id,
                amount,
            },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn transfer_from(
        &self,
        app: &mut App,
        sender: &Addr,
        from: &str,
        to: &str,
        id: u64,
        amount: Uint128,
        msg: impl Into<Option<Binary>>,
    ) -> Result<(), ContractError> {
        let msg = msg.into();

        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::TransferFrom {
                from: from.to_string(),
                to: to.to_string(),
                id,
                amount,
                msg,
            },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn approve_all(
        &self,
        app: &mut App,
        sender: &Addr,
        operator: &str,
        expiration: impl Into<Option<Expiration>>,
    ) -> Result<(), ContractError> {
        let expiration = expiration.into();

        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::ApproveAll {
                operator: operator.to_string(),
                expiration,
            },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn revoke_all(
        &self,
        app: &mut App,
        sender: &Addr,
        operator: &str,
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::RevokeAll {
                operator: operator.to_string(),
            },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn set_minter(
        &self,
        app: &mut App,
        sender: &Addr,
        minter: Option<String>,
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::SetMinter { minter },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn disable_token_minting(
        &self,
        app: &mut App,
        sender: &Addr,
        id: u64,
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::DisableTokenMinting { id },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn set_owner(
        &self,
        app: &mut App,
        sender: &Addr,
        owner: Option<String>,
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::SetOwner { owner },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn update_collection_details(
        &self,
        app: &mut App,
        sender: &Addr,
        name: String,
        description: String,
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::UpdateCollectionDetails { name, description },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn query_contract_info(&self, app: &App) -> StdResult<ConfigResponse> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Config {})
    }

    #[track_caller]
    pub fn query_token_info(&self, app: &App, id: u64) -> StdResult<TokenInfoResponse> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::TokenInfo { id })
    }

    #[track_caller]
    pub fn query_tokens_info(
        &self,
        app: &App,
        start_after: Option<u64>,
        limit: Option<u32>,
    ) -> StdResult<Vec<TokenInfoResponse>> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::TokensInfo { start_after, limit })
    }

    #[track_caller]
    pub fn query_balance(&self, app: &App, owner: String, id: u64) -> StdResult<Uint128> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Balance { owner, id })
    }

    #[track_caller]
    pub fn query_batch_balance(
        &self,
        app: &App,
        owner: String,
        ids: Vec<u64>,
    ) -> StdResult<Vec<Uint128>> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::BatchBalance { owner, ids })
    }

    #[track_caller]
    pub fn query_is_approved_for_all(
        &self,
        app: &App,
        owner: String,
        operator: String,
    ) -> StdResult<Option<Expiration>> {
        app.wrap().query_wasm_smart(
            self.0.clone(),
            &QueryMsg::IsApprovedForAll { owner, operator },
        )
    }

    #[track_caller]
    pub fn query_approvals_by_owner(
        &self,
        app: &App,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<Vec<Approval>> {
        app.wrap().query_wasm_smart(
            self.0.clone(),
            &QueryMsg::ApprovalsByOwner {
                owner,
                start_after,
                limit,
            },
        )
    }

    #[track_caller]
    pub fn query_approvals_by_operator(
        &self,
        app: &App,
        operator: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<Vec<Approval>> {
        app.wrap().query_wasm_smart(
            self.0.clone(),
            &QueryMsg::ApprovalsByOperator {
                operator,
                start_after,
                limit,
            },
        )
    }

    #[track_caller]
    pub fn query_balances_by_owner(
        &self,
        app: &App,
        owner: String,
        start_after: Option<u64>,
        limit: Option<u32>,
    ) -> StdResult<Vec<Balance>> {
        app.wrap().query_wasm_smart(
            self.0.clone(),
            &QueryMsg::BalancesByOwner {
                owner,
                start_after,
                limit,
            },
        )
    }

    #[track_caller]
    pub fn query_balances_by_id(
        &self,
        app: &App,
        id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<Vec<Balance>> {
        app.wrap().query_wasm_smart(
            self.0.clone(),
            &QueryMsg::BalancesById {
                id,
                start_after,
                limit,
            },
        )
    }
}

impl From<Cw1155> for Addr {
    fn from(contract: Cw1155) -> Self {
        contract.0
    }
}
