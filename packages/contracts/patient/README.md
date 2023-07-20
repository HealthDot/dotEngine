# Patient NFT Contract

## Description
This repository contains the implementation of a non-fungible token (NFT) contract using ink!, a Rust-based eDSL for writing Wasm smart contracts. It's specially designed to store patient data in the form of tokens on a blockchain. Each token holds a unique identifier (`TokenId`) linked to the patient's data.

## Features
The Patient contract has the following features:

- Minting new tokens
- Transferring tokens between accounts
- Approving accounts to manage tokens on behalf of their owners
- Querying the balance of an account
- Querying the owner of a specific token

The contract emits events when:
- The ownership of any NFT changes
- The approved address for an NFT changes
- An operator's approved status changes

## Setup
Before using this contract, make sure you have the necessary Rust toolchain and the `cargo-contract` utility installed. Then, clone this repository and navigate to its root directory. To compile the contract, run `cargo contract build`.

## Tests
To run the tests, execute `cargo test`.

## Usage
Here are the functions provided by the Patient contract:

- `new(String, String)`: Constructor function that initializes a new contract with a given token name and symbol.
- `balance_of(AccountId)`: Returns the number of unique tokens owned by an account.
- `owner_of(TokenId)`: Returns the owner of a specific token.
- `approve(AccountId, TokenId)`: Approves an account to manage a token on behalf of its owner.
- `get_approved(TokenId)`: Returns the account approved to manage a specific token.
- `transfer(AccountId, TokenId)`: Transfers a token from the caller to a recipient.
- `transfer_from(AccountId, AccountId, TokenId)`: Transfers a token from a sender to a recipient.
- `mint(TokenId)`: Mints a new token with a specific ID.

## Note
This is a Wasm contract and as such doesn't have a standard library. The contract's state is stored in ink! storage. It uses the scale codec for encoding and decoding data.

## Contact
If you have any questions or suggestions, feel free to open an issue or submit a pull request.
