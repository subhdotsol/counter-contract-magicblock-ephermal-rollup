use crate::{constants::COUNTER_SEED, events::CounterInitialized, state::Counter};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init,
        payer = initializer,
        space = 8 + Counter::LEN,
        seeds = [COUNTER_SEED, initializer.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeCounter>) -> Result<()> {
    ctx.accounts.counter.count = 0;

    emit!(CounterInitialized {
        counter: ctx.accounts.counter.key(),
    });

    Ok(())
}
