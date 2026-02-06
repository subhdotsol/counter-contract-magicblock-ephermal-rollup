use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::cpi::{
    delegate_account, undelegate_account, DelegateAccounts, DelegateConfig,
};
use ephemeral_rollups_sdk::ephem::{commit_accounts, commit_and_undelegate_accounts};

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

declare_id!("2yzu2LNwt8rhbXespx1yRzMB88GNehigooLuZz21qK1V");

#[program]
pub mod counter {
    use super::*;

    // --------------------------------
    // InitializeCounter
    // --------------------------------
    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter initialized: {}", counter.count);
        Ok(())
    }

    // --------------------------------
    // IncreaseCounter
    // --------------------------------
    pub fn increase_counter(ctx: Context<IncreaseCounter>, increase_by: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += increase_by;
        msg!("Counter increased: {}", counter.count);
        Ok(())
    }

    // --------------------------------
    // Delegate
    // --------------------------------
    pub fn delegate(ctx: Context<Delegate>) -> Result<()> {
        let seeds = &[
            b"counter",
            ctx.accounts.initializer.key.as_ref(),
            &[ctx.bumps.counter],
        ];

        let delegate_accounts = DelegateAccounts {
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

        delegate_account(delegate_accounts, &[seeds], config)?;
        Ok(())
    }

    // --------------------------------
    // Commit
    // --------------------------------
    pub fn commit(ctx: Context<Commit>) -> Result<()> {
        commit_accounts(
            &ctx.accounts.initializer,
            vec![ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;
        Ok(())
    }

    // --------------------------------
    // CommitAndUndelegate
    // --------------------------------
    pub fn commit_and_undelegate(ctx: Context<Commit>) -> Result<()> {
        commit_and_undelegate_accounts(
            &ctx.accounts.initializer,
            vec![ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;
        Ok(())
    }

    // --------------------------------
    // IncrementAndCommit
    // --------------------------------
    pub fn increment_and_commit(ctx: Context<IncreaseCounter>, increase_by: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += increase_by;

        commit_accounts(
            &ctx.accounts.initializer,
            vec![ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;

        Ok(())
    }

    // --------------------------------
    // IncrementAndUndelegate
    // --------------------------------
    pub fn increment_and_undelegate(ctx: Context<IncreaseCounter>, increase_by: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += increase_by;

        commit_and_undelegate_accounts(
            &ctx.accounts.initializer,
            vec![ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;

        Ok(())
    }

    // --------------------------------
    // Undelegate
    // --------------------------------
    pub fn undelegate(ctx: Context<Undelegate>, pda_seeds: Vec<Vec<u8>>) -> Result<()> {
        undelegate_account(
            &ctx.accounts.counter.to_account_info(),
            ctx.program_id,
            &ctx.accounts.delegation_buffer.to_account_info(),
            &ctx.accounts.initializer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
            pda_seeds,
        )?;
        Ok(())
    }
}
