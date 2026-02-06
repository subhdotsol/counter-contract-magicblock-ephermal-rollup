use anchor_lang::prelude::*;

declare_id!("2yzu2LNwt8rhbXespx1yRzMB88GNehigooLuZz21qK1V");

pub mod state;

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
