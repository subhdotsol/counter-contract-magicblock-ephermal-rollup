use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::cpi::undelegate_account;

#[derive(Accounts)]
pub struct Undelegate<'info> {
    #[account(mut)]
    pub counter: AccountInfo<'info>,

    /// CHECK:
    pub delegation_buffer: AccountInfo<'info>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Undelegate>, pda_seeds: Vec<Vec<u8>>) -> Result<()> {
    undelegate_account(
        &ctx.accounts.counter,
        ctx.program_id,
        &ctx.accounts.delegation_buffer,
        &ctx.accounts.initializer,
        &ctx.accounts.system_program,
        pda_seeds,
    )?;
    Ok(())
}
