use anchor_lang::prelude::*;
use crate::CustomError as PurchaseError;


// The buy purchase consists of 3 products
// One product can consist of up to 3 trade providors for a
//   collaborative strategy.
#[account]
pub struct purchase { 
    fee: u32,
    total_contract_price: i32,

    staked: u64, 
    product_id: String,

    purchaser: String, // the pda or the username 
    purchase_id: String,

    contract: String, //Acts as tag //Contract tag multi-sig-wallet against creditline loan on stake 
    
    metadata: String,

    // The contract (purchase) id = the multisig transaction signature
    tx: String,
}

impl Purchase { // bit value sizes. 4 is the suffix value of Anchor
    pub const LEN: usize = 8 // Discriminator
    + 4 + 4 + 8  // fee + price + staked
    + 4 + 64     // product_id
    + 4 + 64     // purchaser
    + 4 + 64     // purchase_id
    + 4 + 64     // contract
    + 4 + 128    // metadata
    + 4 + 64;    // tx
}

enum Purchase {
    Loan, 
    Buy,
    Sell,
    Order,
    Estimate
}

let action == purchase 

#[account]
// The financial product offered
struct product {
    fee: u32,
    total_contract_cost: i32,

    product_id: String,

    exchange_offerer: String, // the pda registered to the trade owner 

    market: String, // The fee taker

    multi_sig_wallet_id: String, 
    
    // The contract (purchase) id = the multisig transaction signature
    tx: String,
}

#[program]
pub mod purchase_program {
    use super::*;

    pub fn initialize_purchase(
        ctx: Context<InitPurchase>,
        product_id: String,
        purchase_id: String,
        contract: String,
        metadata: String,
        fee: u32,
        total_contract_price: i32,
        days_left_rights_to_purchase: i32; 
    ) -> Result<()> {
        let purchase = &mut ctx.accounts.purchase;
        purchase.product_id = product_id;
        purchase.purchaser = ctx.accounts.user.key().to_string();
        purchase.purchase_id = purchase_id;
        purchase.contract = contract;
        purchase.metadata = metadata;
        purchase.fee = fee;
        purchase.total_contract_price = total_contract_price;
        purchase.tx = "placeholder-tx-hash".to_string(); // You can replace with CPI-derived tx
        purchase.days_left_rights_to_purchase;

        Ok(())
    }
}
