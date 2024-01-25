use cosmwasm_std::{Addr, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::contract::{execute, instantiate, query};
use crate::msg::InstantiateMsg;

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
}

impl From<Cw1155> for Addr {
    fn from(contract: Cw1155) -> Self {
        contract.0
    }
}
