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

         pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
           
         }

## Transferring Tokens
To transfer SPL tokens between accounts, you can use the transfer_token function. This function handles the transfer of tokens from one account to another.
    
        pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()> {
            
        }

### Testing the Smart Contract
1. **Set Up Test Environment**:
   Initialize a new keypair:
   
           solana-keygen new --outfile ~/.config/solana/id.json
   Airdrop SOL to your test account:

           solana airdrop 2
2. **Run tests**:

       anchor test

### Directory Structure

   ***programs/***: Contains the source code of the smart contract.
   ***tests/***: Contains test scripts to validate the smart contract functionality.
   ***target/***: Contains the compiled output of the smart contract.

### Smart Contract details

## Program ID
Ensure you update the declare_id! macro with your own program ID:

        declare_id!("5QzXzm5rVK8AdcQjgQRAUsvhFCSHXu2nxpHEExXktqoi");

## Context Structures
The context structures for minting and transferring tokens are defined as follows:

# Mint Token Context

        #[derive(Accounts)]
        pub struct MintToken<'info> {
            #[account(mut)]
            pub mint: UncheckedAccount<'info>,
            pub token_program: Program<'info, Token>,
            #[account(mut)]
            pub token_account: UncheckedAccount<'info>,
            #[account(mut)]
            pub authority: Signer<'info>,
        }

# TransferToken Context

        #[derive(Accounts)]
        pub struct TransferToken<'info> {
            pub token_program: Program<'info, Token>,
            #[account(mut)]
            pub from: UncheckedAccount<'info>,
            #[account(mut)]
            pub to: AccountInfo<'info>,
            #[account(mut)]
            pub from_authority: AccountInfo<'info>,
        }

### Conclusion
This smart contract demonstrates the basic functionality for minting and transferring SPL tokens on the Solana blockchain using the Anchor framework. Feel free to extend and modify the code to suit your specific requirements.








