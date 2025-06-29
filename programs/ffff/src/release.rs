use anchor_lang::prelude::*;
use crate::CustomError as ReleaseError;


enum Release {
    Contract1, 
    Contract2, 
    Contracrt3
}

struct Contract {
    value: u32, 
    fee: i32,
    parties_involved: String,
}


#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_lang::system_program;

use multisigwallet::{MultiSigWallet, InitializeMultisig};

declare_id!("5t9551cj7A3rc3xofwNV4sUkz3cWgH2HuSzJrtAJubbG");

#[program]
pub mod transfer_sol {
    use super::*;

    pub fn transfer_sol_with_cpi(ctx: Context<TransferSolWithCpi>, amount: u64) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig_wallet;
        let signer_key = ctx.accounts.signer.key;

        if !multisig.owners.contains(signer_key) {
            return Err(ReleaseError::UnauthorizedSigner.into());
        }

        if multisig.approvals.contains(signer_key) {
            return Err(ReleaseError::AlreadyApproved.info());
        }

        multisig.approvals.push(*signer_key);

        if multisig.approvals.len() as u8 >= multisig.threshold {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_into(),
                    system_program::Transfer {
                        from: ctx.accounts.multisig_wallet.to_account_info(),
                        to: ctx.accounts.recipient.to_account_info(),
                    },
                ),
                amount,
            )?;
            multisig.approvals.clear(); // reset after successful transfer
        }
        Ok(())
        
        }

    // Directly modifying lamports is only possible if the program is the owner of the account
    pub fn transfer_sol_with_program(
        ctx: Context<TransferSolWithProgram>,
        amount: u64,
    ) -> Result<()> {
        **ctx.accounts.payer.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.recipient.try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSolWithCpi<'info> {
    #[account(mut)]
    multisig_wallet: Account<'info, MultisigWallet>,
    #[account(mut)]
    signer: AccountInfo<'info>,

    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSolWithProgram<'info> {
    /// CHECK: Use owner constraint to check account is owned by our program
    #[account(
        mut,
        owner = id() // value of declare_id!()
    )]
    payer: UncheckedAccount<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
}