use anchor_lang::prelude::*;
use crate::stake::Stake;

#[account]
pub struct Award {
    pub user: Pubkey,
    pub amount: u64,
    pub token: String,
    pub claimed: bool, 

}

#[derive(Accounts)]
pub struct ClaimAward<'info> {
    #[account(mut, has_one = purchaser)]
    pub stake: Account<'info, Stake>,
    #[account(init_if_needed, payer = purchaser, space = 8 + 32 + 8 + 1, seeds = [b"award", purchaser.key().as_ref()], bump)]
    pub award: Account<'info, Award>, 
    #[account(mut)
    pub purchaser: Signer<'info>],
    pub system_program: Program<'info, System>,
}

#[program]
pub mod awards {
    use super::*;

    #[account(seeds = [b"..."], bump)]
    pub fn claim_award(ctx: Context<ClaimAward>) -> Result<()> {
        let stake = &mut ctx.accounts.stake;
        require!(stake.claimable, CustomError::NotYetClaimable);

        let award = &mut ctx.accounts.award;
        award.user = stake.purchaser;
        award.amount = stake.stake_amount * 2; //
        award.token = "SOL"to_string();
        award.claimed = true;

        stake.claimable = false;

        Ok(())
    }
}

#[error_code]
pub enum CustomError {
    #[msg("No awards to be awarded yet.")]
    NotYetClaimable,
}
