use crate::events::CounterCommitted;
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;

use super::Commit;

pub fn handler(ctx: Context<Commit>) -> Result<()> {
    commit_and_undelegate_accounts(
        &ctx.accounts.initializer,
        vec![ctx.accounts.counter.clone()],
        &ctx.accounts.magic_context,
        &ctx.accounts.magic_program,
    )?;

    emit!(CounterCommitted {
        counter: ctx.accounts.counter.key(),
    });

    Ok(())
}
