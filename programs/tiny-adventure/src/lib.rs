use anchor_lang::prelude::*;

declare_id!("DXxLHjAjv1xyV367pNLzFGSVdmUSD9UJRGKhjuvxm79D");

#[program]
pub mod tiny_adventure {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
