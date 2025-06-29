use anchor_lang::prelude::*;
use crate::CustomError as BuyError;


struct User {
    username: String, 
    wallets: Vec<Pubkey>,
    user_creditline_account: Pubkey, 
    creditline_balance: u64,
}

impl User {
    fn new(name: &str, user_wallets: Vec<Pubkey>, user_creditline_account: Pubkey, creditline_balance: u64) -> Self {
        Self {
            username: name.to_string(),
            wallets,
            user_creditline_account,
            creditline_balance,
        }
    }

    fn total_sol_balance(&self, total_wallet_balances: &Vec<u64>) -> u64 {
        total_wallet_balances.iter().sum()
    }

    fn purchase(&mut self, amount: u64) {
        if self.creditline_balance >= amount {
            self.creditline_balance -= amount;
            println!("NICE {} purchased {} buy-in!", self.username, amount);
        } elae {
            println!("NOOO you a brokie! Not enough balance on your creditline to purchase this product! Top up!");
        }
    }

    fn buy(&mut self, total_contract_price: u64) {
        println!("== {} clicked BUY at product price {}", self.username, total_contract_price)
        self.purchase(total_contract_price);
    }
}

struct CreditLineAccount {
    balance: u64,
    used: u64,
}

// we are gonna simulate the purchase, so that we can 
// start creating a model, architecture, and audit
// audit scope of sorts
fn main() {
    let program_id = Pubkey::new_unique();
    let user_wallet = Pubkey::new_unique();
    let all__wallet_balances = vec![30, 20, 10]; // simulated lamport balances
    let total_wallet_balances: u64 = all_wallet_balances.iter().sum();

    let user_wallet_pda = Pubkey::find_program_address(&[b"user", user.key().as_ref()], &program_id).0;
    let creditline_pda = Pubkey::find_program_address(&[b"creditline", user.key().as_ref()], &program_id).0;

    let required_contract_purchase_amount = 40;

    //let sol_balance = user_wallet_account.lamports();
    //let creditline_balance = creditline_account.balance;
    //let used_creditline = creditline_account.used;

    let available_credit_line = creditline_account.balance.saturating_sub(used_creditline);
    if available_credit < required_contract_purchase_amount {
        return Err(BuyError::InsufficientCreditline.into());
    }


    let mut user = User::new("Alice", user_wallet, available_credit);

    // Simulate buying product at contract price which is the purchase of buy-in 

    // Not sure if simulation is meant to be in this file at all actually and I am not
    //    sure whether simulation should be in an Anchor scaffolded project at all
    user.buy()
    user.buy(required_contract_purchase_amount)
}