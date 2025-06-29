use solana_sdk::signature::{Keypair, Signer, read_keypair_file, write_keypair_file};
use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::path::Path;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::{Client, Cluster, Program};
use std::rc:Rc;

mod buy;
mod product;
mod initialise_stake;
mod purchase;
mod release;
mod awards; 
mod stake;

use buy::User;
use product::initialize_multisig;
use purchase::{initialize_stake, release};

use awards::{Contract, User};

// User needs to add their existing wallet in order to buy in to strategy. 
// For this concept, we are just creating a test wallet with 2 SOL
fn main() {
    // 1. Connect to Devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // 2. Generate new wallet
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey();
    println!("✅ New wallet: {}", pubkey);

    // 3. Airdrop 2 SOL
    let airdrop_sig = client.request_airdrop(&pubkey, 2_000_000_000).expect("Airdrop failed");
    client
        .confirm_transaction(&airdrop_sig)
        .expect("Failed to confirm transaction");
    println!("💸 Airdropped 2 SOL to {}", pubkey);

    // 4. Save keypair to file
    let path = format!("user_wallets/{}.json", pubkey);
    std::fs::create_dir_all("user_wallets").unwrap();
    write_keypair_file(&keypair, Path::new(&path)).expect("Failed to save keypair");
    println!("📁 Saved keypair to: {}", path);

    let (pda, bump) = Pubkey:find_program_address(&[user.key().as_ref(), v"wallet"], program_id)

    let mut user = User::new("Alice", 100);
    let buy_in_fee = buy_in_price + 2%
    let total_buy_in_contract_cost = buy_in_price + buy_in_fee

    let buy_in_price = BuyInPrice::new("BuyInFirstOne", 40)
    let buy_in_fee = BuyInFee::new("BuyInFirstOneFee", 0.8)
    let stake = total_contract_price - buy_in_fee - product1_trade_fee - product2_trade_fee - product3_trade_fee  
    // The user is signing three diff multisig contracts w/1 buy btn
    // Buy has initiated purchase 
    // Successful purchase and multi-sig wallet signature has initiated the stake
    // The successful purchase and multi-sig wallet signature has initiated
    //    the release to the multisigwallet owners (trade providors)

    let balance = ctx.accounts.wallet.to_account_info().lamports();
    let total_available_sol = buy::check_all_balances(&pubkey, &program_id, &client)
        .expect("Failed to fetch wallet balances");
    println!("Total available SOL across PDAs: {} lamports", total_available_sol);

    user.buy(buy_in_price);
    product.initialize_multisig(3);
    purchase.initialise_stake(stake);
    purchase.release();
    stake.awards();

    let payer = Rc::new(Keypair::new());

    // Connect to Devnet & get the program 
    let client = Client::new(Cluster::Devnet, payer::clone());
    let program = client.program("5t9551cj7A3rc3xofwNV4sUkz3cWgH2HuSzJrtAJubbG");

    let purchase = multisigwallet::initialize_multisig() && purchase::purchase(); 
        .request()
        .accounts(transfer_sol::accounts::TransferSolWithProgram {
            payer: payer.pubkey(),
            recipient: multi
        })


    let tx = program
        .request()
        .accounts(purchase_program::accounts::InitPurchase {
            purchase: purchase_key.pubkey(),
            user: payer.pubkey(),
            system_program: anchor_client::solana_sdk::system_program::ID, 
        })
        .args(purcahse_program::instruction::InitializePurchase {
            product_id: "AITokenStrategy".to_string(),
            purchase_id: "GG82373".to_string(),
            contract: "FullfillmentOrder".to_string(),
            metadata: "by BHM, AGD, and BCC; 40 hours left; $ 7500".to_string();
            fee: 2%,
            total_contract_price: 40,
        })
        .signer(&payer)
        .signer(&purchase_key)
        .send();

}




