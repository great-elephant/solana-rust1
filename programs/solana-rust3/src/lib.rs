use anchor_lang::prelude::*;

declare_id!("6VEpHxSBWrwy7kgz2nQmGSAJnaBN6hGqarb2SjhsGBUd");

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
