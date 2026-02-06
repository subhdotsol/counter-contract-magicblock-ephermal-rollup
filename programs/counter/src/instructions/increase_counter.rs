use crate::{errors::CounterError, events::CounterIncreased, state::Counter};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct IncreaseCounter<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"counter", initializer.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, Counter>,

    /// CHECK: required for ER commit paths
    pub magic_program: AccountInfo<'info>,
    /// CHECK:
    pub magic_context: AccountInfo<'info>,
}

pub fn handler(ctx: Context<IncreaseCounter>, increase_by: u64) -> Result<()> {
    let counter = &mut ctx.accounts.counter;

    counter.count = counter
        .count
        .checked_add(increase_by)
        .ok_or(CounterError::Overflow)?;

    emit!(CounterIncreased {
        counter: counter.key(),
        new_value: counter.count,
    });

    Ok(())
}
