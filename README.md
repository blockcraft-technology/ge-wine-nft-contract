
## GE-WINE NFT Contract
This CosmWasm-based CW1155-like contract allows for the minting, transferring, and querying of NFTs and semi-fungible tokens (similar to ERC1155 on Ethereum). Designed for tokenizing wine assets, this contract supports multiple token types, each with configurable amounts.

### Features
____
- Mint Tokens: Only the contract owner can mint tokens with specified token_id, owner, and amount.
- Transfer Tokens: Users can transfer specific token IDs and amounts to other addresses.
- Query Balances: Retrieve the balance of a given token ID for a specific address.