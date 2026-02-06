use crate::constants::COUNTER_SEED;
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::cpi::*;

#[derive(Accounts)]
pub struct Delegate<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        mut,
        seeds = [COUNTER_SEED, initializer.key().as_ref()],
        bump
    )]
    pub counter: AccountInfo<'info>,

    /// CHECK:
    pub owner_program: AccountInfo<'info>,
    /// CHECK:
    pub delegation_buffer: AccountInfo<'info>,
    /// CHECK:
    pub delegation_record: AccountInfo<'info>,
    /// CHECK:
    pub delegation_metadata: AccountInfo<'info>,
    /// CHECK:
    pub delegation_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

    /// CHECK:
    pub validator: Option<AccountInfo<'info>>,
}

pub fn handler(ctx: Context<Delegate>) -> Result<()> {
    let seeds = &[
        COUNTER_SEED,
        ctx.accounts.initializer.key.as_ref(),
        &[ctx.bumps.counter],
    ];

    let accounts = DelegateAccounts {
        payer: ctx.accounts.initializer.to_account_info(),
        pda: ctx.accounts.counter.to_account_info(),
        owner_program: ctx.accounts.owner_program.to_account_info(),
        buffer: ctx.accounts.delegation_buffer.to_account_info(),
        delegation_record: ctx.accounts.delegation_record.to_account_info(),
        delegation_metadata: ctx.accounts.delegation_metadata.to_account_info(),
        delegation_program: ctx.accounts.delegation_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    let config = DelegateConfig {
        validator: ctx.accounts.validator.as_ref().map(|v| v.key()),
        ..Default::default()
    };

    delegate_account(accounts, &[seeds], config)?;
    Ok(())
}
