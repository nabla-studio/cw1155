use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Binary, CosmosMsg, StdResult, Uint128, WasmMsg};

// `Cw1155ReceiveMsg` is used to deserialize incoming messages under the `Receive` variant in an `ExecuteMsg`.
// It represents a message received for a CW1155 token transfer.
#[cw_serde]
pub struct Cw1155ReceiveMsg {
    /// The account that executed the send message.
    pub operator: String,
    /// The account from which the token was transferred.
    pub from: Option<String>,
    /// The ID of the transferred token.
    pub id: u64,
    /// The amount of tokens transferred.
    pub amount: Uint128,
    /// The binary message associated with the token transfer.
    pub msg: Binary,
}

impl Cw1155ReceiveMsg {
    // Serializes the message into a binary format that is compatible.
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverExecuteMsg::Receive(self);
        to_json_binary(&msg)
    }

    // Creates a `CosmosMsg` to send this structure to the specified contract.
    pub fn into_cosmos_msg<T: Into<String>>(self, contract_addr: T) -> StdResult<CosmosMsg> {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: vec![],
        };
        Ok(execute.into())
    }
}

// Helper for properly serializing the above-defined message.
#[cw_serde]
enum ReceiverExecuteMsg {
    Receive(Cw1155ReceiveMsg),
}
