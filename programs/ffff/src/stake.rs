use anchor_lang::prelude::*;
use crate::CustomError as StakeError;

use purchase::purchase::Buy;

fn buy(staked: amount) -> stake: confirmed {

}

if purchase == successful (stake_amount: u32, buy_contract: String) -> Result<(()> {
    }
)

pub struct stake {
    pub stake_amount: u64,
    pub purchaser: Pubkey;
    pub contract_id: String,
    pub active: bool, 
    pub claimable: bool, 
}

impl Stake {
    pub const LEN: usize = 8 //Discriminator
        +8 //stake_amount
        + 32 // purchaser
        + 4 + 64 // contract_id
        + 1 //active
        + 1; //claimable
}

#[derive(Accounts)]
pub struct InitStake<'info> {
    #[account(init, payer = user, space = 8 + Stake::LEN)]
    pub stake: Account<'info, Stake>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod stake_program {
    use super::*;

    pub fn init_skate(
        ctx: Context<InitStake>,
        contract_id: String, 
        amount: u64,
    ) -> Result<()> {
        let stake = &mut ctx.accounts.stake;
        stake.stake_amount = amount;
        stake_purchaser = ctx.accounts.user.key();
        stake.contract_id = contract_id;
        stake_active = true; 
        stake.claimable = false;

        Ok(())
    }

    pub fn mark_stake_successful(ctx: Context<ClaimAward>) -> Result<()> {
        let stake &mut ctx.accounts.stake;
        require!(stake.active, CustomError::StakeNtActive);
        stake.claimable = true;
        stake.active = true; 

        Ok(())
    }
}