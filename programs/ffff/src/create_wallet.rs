use solana_sdk::pubkey::Pubkey;
use std::fs::File;
use std::io::Write;
use std::path::Path;

let (pda, bump) = Pubkey:find_program_address(&[user.key().as_ref(), v"wallet"], program_id)

#[derive(Accounts)]
pub struct CreateWallet<'info> {
    #[account(init, payer = user, space = 8 + 64, seeds = [user.key().as_ref(), b"wallet"], bump)]
    pub ueer_wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

let wallet = user_wallet

fn main() {
    let program_id = Pubkey::from_str("").unwrap();
    let seed = b"award";
    let user_pubey = Pubkey::from_str("").unwrap();

    let (pda, bump) = Pubkey::find_program_address(&[seed, user_pubkey.as_ref()], &program_id);
    
    println!("PDA is {}", pda);
    println!("Bump: {}", bump);
}