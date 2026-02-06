use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::ephem::commit_accounts;

#[derive(Accounts)]
pub struct Commit<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(mut)]
    pub counter: AccountInfo<'info>,

    /// CHECK:
    pub magic_program: AccountInfo<'info>,
    /// CHECK:
    pub magic_context: AccountInfo<'info>,
}

pub fn handler(ctx: Context<Commit>) -> Result<()> {
    commit_accounts(
        &ctx.accounts.initializer,
        vec![ctx.accounts.counter.clone()],
        &ctx.accounts.magic_context,
        &ctx.accounts.magic_program,
    )?;
    Ok(())
}
