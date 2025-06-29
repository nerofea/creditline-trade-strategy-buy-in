use solana_sdk::signature::{Keypair, Signer};
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
    let keypair = Keypair::new();
    let keypair_bytes = keypair.to_bytes();

    let path = "target/deploy/ffff-keypair.json";

    let mut file = File::create(Path::new(path)).expect("Unable to create file");
    let json_array = Vec<String> = keypair_bytes.iter().map(|b| b.to_string()).collect();
    let json = format!("[{}]", json_array.join(","));
    file.write_all(json.as_bytes()).expect("Unable to write keypair")

    println!("New keypair generated ay {}!")
    println!("Public Key: {}", keypair.pubkey());
}