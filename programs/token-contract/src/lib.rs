use anchor_lang::prelude::*;

declare_id!("FrE1dW61BTde2RNUgtR4euwB8wjDW8Fe4oWcVMgvcmVc");

#[program]
pub mod token_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
