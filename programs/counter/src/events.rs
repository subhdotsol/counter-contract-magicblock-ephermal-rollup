use anchor_lang::prelude::*;

#[event]
pub struct CounterInitialized {
    pub counter: Pubkey,
}

#[event]
pub struct CounterIncreased {
    pub counter: Pubkey,
    pub new_value: u64,
}

#[event]
pub struct CounterCommitted {
    pub counter: Pubkey,
}
