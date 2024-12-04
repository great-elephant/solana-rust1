use anchor_lang::prelude::*;

declare_id!("AgLd6tvSA9J3PH91KikYxNMRYqLQMi5aSKyofGECuuSP");

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
