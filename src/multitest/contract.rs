use cosmwasm_std::{Addr, Binary, StdResult, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};
use cw_utils::Expiration;

use crate::contract::{execute, instantiate, query};
use crate::msg::{
    BalanceResponse, ContractInfoResponse, ExecuteMsg, InstantiateMsg, IsApprovedForAllResponse,
    QueryMsg,
};
use crate::state::TokenInfo;
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
        is_transferrable: impl Into<Option<bool>>,
    ) -> Result<(), ContractError> {
        let max_supply = max_supply.into();
        let is_transferrable = is_transferrable.into();

        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::Register {
                max_supply,
                is_transferrable,
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
    pub fn query_contract_info(&self, app: &App) -> StdResult<ContractInfoResponse> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::ContractInfo {})
    }

    #[track_caller]
    pub fn query_token_info(&self, app: &App, id: u64) -> StdResult<TokenInfo> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::TokenInfo { id })
    }

    #[track_caller]
    pub fn query_balance(&self, app: &App, owner: String, id: u64) -> StdResult<BalanceResponse> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Balance { owner, id })
    }

    #[track_caller]
    pub fn query_is_approved_for_all(
        &self,
        app: &App,
        owner: String,
        operator: String,
    ) -> StdResult<IsApprovedForAllResponse> {
        app.wrap().query_wasm_smart(
            self.0.clone(),
            &QueryMsg::IsApprovedForAll { owner, operator },
        )
    }
}

impl From<Cw1155> for Addr {
    fn from(contract: Cw1155) -> Self {
        contract.0
    }
}
