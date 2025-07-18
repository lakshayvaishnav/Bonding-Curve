use anchor_lang::prelude::*;
pub mod states;

declare_id!("3721oo4bjNRm5XbnZjmVFeJon5P3KTDzbZBGhe61qn2j");

#[program]
pub mod bondingcurve {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
