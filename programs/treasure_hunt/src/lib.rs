use anchor_lang::prelude::*;

declare_id!("FR1jX3PWkGr1bsePg9td9e2mc9neEPbjcaABomdWM3ZR");

#[program]
pub mod treasure_hunt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
