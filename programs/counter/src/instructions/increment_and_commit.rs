use crate::{errors::CounterError, state::Counter};
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::ephem::commit_accounts;

use super::IncreaseCounter;

pub fn handler(ctx: Context<IncreaseCounter>, increase_by: u64) -> Result<()> {
    let counter = &mut ctx.accounts.counter;

    counter.count = counter
        .count
        .checked_add(increase_by)
        .ok_or(CounterError::Overflow)?;

    commit_accounts(
        &ctx.accounts.initializer,
        vec![counter.to_account_info()],
        &ctx.accounts.magic_context,
        &ctx.accounts.magic_program,
    )?;

    Ok(())
}
