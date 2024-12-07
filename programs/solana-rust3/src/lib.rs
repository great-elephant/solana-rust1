use anchor_lang::prelude::*;

declare_id!("3YZmiD8PjFPG7vTva9MtPF2FBw4aRUmho5kXKTYj9cFM");

#[program]
pub mod solana_rust3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
