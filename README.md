### Token Contract on Solana using Anchor

## Overview
This repository contains a simple Rust-based smart contract implemented using the Anchor framework on the Solana blockchain. The smart contract includes two main functions:
1. Minting a new SPL token.
2. Transferring the SPL token between two accounts.

## Prerequisites
Before you begin, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html)

### Getting Started

## Clone the Repository
    git clone https://github.com/yourusername/token-contract
    cd token-contract

## Build the Project
    anchor build

### Deploy the Program
1. **Start a local Solana test validator**:
   
    ```sh
    solana-test-validator
2. **In a new terminal window, deploy the program**:
   
   ```sh
   anchor deploy

### Instructions
## Minting Tokens
To mint new SPL tokens, you can use the mint_token function. This function mints tokens to a specified token account.

```sh
     pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
       
     }




