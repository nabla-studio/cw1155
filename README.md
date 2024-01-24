# CW1155: Multiple Tokens Manager

CW1155 is an innovative smart contract tailored for the efficient management of
a wide range of token types within CosmWasm-enabled blockchains. 
Drawing inspiration from Ethereum's ERC1155 and CosmWasm's CW1155 standards, 
this contract introduces significant enhancements, broadening its functionality
and utility. Its core objective is to facilitate the management of a variety of
token types, including fungible, non-fungible, and semi-fungible tokens, 
through a singular contract instance.

### Features:
- **Token Identifier**: Employs integers for seamless token identification, 
thereby bypassing the complexities associated with string-based identification.
- **MetadataURI**: MetadataURIs are attached to the contract instance, 
supporting ID substitution by clients. Clients are mandated to replace `{id}` 
within any URI with the actual token ID, facilitating the utilization of a 
common on-chain string across numerous tokens by defining a single URI.
- **Conformity to Metadata Schema**: Adheres to the 
[ERC-1155 Metadata URI JSON Schema](https://eips.ethereum.org/EIPS/eip-1155)
for standardized metadata integration.

## Messages

**Fields marked with * are mandatory.**

*Note: All minting operations can only be performed by the designated `minter`.
Registering operations can only be performed by the designated `owner`.
Batch operations are omitted since CosmWasm allows executing multiple messages 
in the same transaction.*


### Contract Initialization - Instantiate

#### Instantiate

Create a new contract instance for managing multiple token types.

```
Instantiate {
    "uri*": "string",
    "minter": "string",
    "owner": "string"
}
```
Parameters:
- `uri`: Base URI for metadata, immutable post-creation.
- `minter`: Address authorized to minting operations on the tokens, defaulting
to the message sender.
- `owner`: Address authorized to registering operations for new tokens, 
defaulting to the message sender.

---

### Token Management - Execute

#### Register

Register a new token type with specific attributes without immediate minting.

```
Register { 
    "max_supply": "integer",
    "is_transferrable": "boolean"
}
```
Parameters:
- `max_supply`: Cap on token quantity. If unspecified, the supply is deemed 
unlimited.
- `is_transferrable`: Flag indicating if the token can be transferred 
post-minting, defaulting to true.

*Attempts to register tokens by addresses not authorized as owners will result
in transaction failure.*

#### Mint

Mint a specified quantity of a pre-registered token to a designated address.

```
Mint {
    "to*": "string",
    "id*": "integer",
    "amount*": "integer",
    "msg": "binary"
}
```
Parameters:
- `to`: Recipient address.
- `id`: Token ID.
- `amount`: Quantity to mint.
- `msg`: Message for smart contract recipients ( which must implement the 
`CW1155Receiver` ). If recipient is an EOA, `msg` should be `None`.

*Attempts to mint tokens beyond the max_supply limit or by addresses not 
authorized as minters will result in transaction failure.*

#### Burn

Burn a specified quantity of a token from a holder's address.

```
Burn {
    "from*": "string",
    "id*": "integer",
    "amount*": "integer"
}
```
Parameters:
- `from`: Holder's address.
- `id`: Token ID.
- `amount`: Quantity to burn.

Only tokens owned or authorized can be burned.

#### TransferFrom

Transfer a specified quantity of a token from one address to another.

```
TransferFrom {
    "from*": "string",
    "to*": "string",
    "id*": "integer",
    "amount*": "integer",
    "msg": "binary"
}
``` 
Parameters:
- `from`: Sender's address.
- `to`: Recipient's address.
- `id`: Token ID.
- `amount`: Quantity to transfer.
- `msg`: Message for smart contract recipients ( which must implement the 
`CW1155Receiver` ). If recipient is an EOA, `msg` should be `None`.

*Transfers beyond ownership/unauthorized or non-transferrable tokens result in 
failure.*

#### ApproveAll

Grant an operator the authority to manage all of the sender's tokens.

```
ApproveAll {
    "operator*": "string",
    "expires": "timestamp"
}
``` 
Parameters:
- `operator`: Operator's address
- `expires`: Approval expiration timestamp, defaulted to Never.

*Approval scope includes all current and future tokens of the owner, while
expires is valid.*

#### RevokeAll

Revoke an operator's authority to manage all of the sender's tokens.

```
RevokeAll {
    "operator*": "string"
}
```
Parameter:
- `operator`: Operator's address.

#### SetMinter

Modify the address of who can perform minting operations.

```
SetMinter {
    "minter*": "string"
}
```
Parameter:
- `minter`: New minter's address.

*NOTE: Only the current minter can change the minter address.*

#### SetOwner

Modify the address of who can perform registering operations.

```
SetOwner {
    "owner*": "string"
}
```
Parameter:
- `owner`: New owner's address.

*NOTE: Only the current owner can change the owner address.*

---

### Query

#### Balance

Retrieve the token balance for a specific owner and token ID.

```
Balance { 
    "owner*": "string", 
    "id*": "integer"
}
``` 
Response:
```
BalanceResponse {
    "amount": "integer"
}
```

#### BatchBalance

Query the token balances for multiple IDs associated with a single owner.

```
BatchBalance { 
    "owner*": "string", 
    "id*": "array[integer]"
}
```
Response:
```
BatchBalanceResponse {
    "amount": "array[integer]"
}
```

#### IsApprovedForAll

Check if an operator is authorized to manage all tokens of a given owner.

```
IsApprovedForAll { 
    "owner*": "string", 
    "operator*": "string"
}
``` 
Response:
```
IsApprovedForAllResponse {
    "approved": "boolean"
}
```

#### ContractInfo

Access basic contract information, including the minter's address, the owner's one and the 
metadata URI.

```
ContractInfo {}
```
Response:
```
ContractInfoResponse {
    "metadata_uri": "string"
    "minter": "string", 
    "owner": "string", 
}
```

#### TokenInfo

Fetch details about a specific token, including its transferability, maximum 
supply, burned quantity, and current supply.

```
TokenInfo {
    "id*": "integer",
}
```
Response:
```
TokenInfoResponse {
    "is_trasferrable": "boolean",
    "max_supply": "integer",
    "burned": "integer",
    "current_supply": "integer"
}
```

---

## Receiver

Contracts intending to receive CW1155 tokens MUST implement the 
`Cw1155ReceiveMsg`.
This interface is generally *not* integrated into CW1155 contracts directly.

#### Cw1155ReceiveMsg

Handle the receipt of single token transfers or mints.
```
Cw1155ReceiveMsg {
    "operator": "string",
    "from": "string",
    "token_id": "integer",
    "amount": "integer",
    "msg": "binary"
}
```