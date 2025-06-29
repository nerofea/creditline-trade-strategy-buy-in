use anchor_lang::prelude::*;

declare_id!("EJLmiVjwb2sMy4336ZK99LWeWLjuhHoe7jjq8eZ2kWNN");

#[program]
pub mod ffff {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: Market Maker {:?}, where we connect 
        your business creditline needs with the best trade providors", ctx.program_id);
        Ok(())
    }

    #[error_code]
    pub enum CustomError {
        #[msg("Insufficient creditline.")]
        InsufficientCreditline,
        #[msg("Unauthorized signer.")]
        UnauthorizedSigner,
        #[msg("Already approved.")]
        AlreadyApproved,
        #[msg("Award not yet claimable.")]
        NotYetClaimable,
    }

}

pub mod buy;
pub mod purchase; 
pub mod stake;
pub mod release;
pub mod awards;

pub use buy::*;
pub use purchase::*;
pub use stake::*;
pub use release::*;
pub use awards::*;

#[derive(Accounts)]
pub struct Initialize {}
