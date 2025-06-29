use anchor_lang::prelude::*;

// Multi sig wallet 
pub struct MultiSigWallet {
    pub owners: Vec<Pubkey>,
    pub threshold: u9, 
    pub approvals: Vec<PubKey>,
    pub bump: u8,
}

// Change to finance concept instead of adv later
struct Advertisor {
    name: String,
    id: String, 
    pubkey: String,
}

// Strategy Providor
struct ProductOwner {
    name: String,
    id: String, 
    pubkey: String,
}

// Gives Product Ownership Rights to User and Takes Fee from Everyone
struct Market {
    name: String,
    id: String, 
    pubkey: String,
}

impl MultisigWallet {
    // Memory allcation size for the Solana account
    pub const LEN: usize = 32 * 3 + 1 + 32 * 3;
}

pub fn initialize_multisig(
    ctx: Context<InitializeMultisig>,
    owners: Vec<Pubkey>,
    threshold: u8,
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig_wallet;

    require!(
        threshold asusize = owners.len(),
        ErrorCode::InvalidThreshold
    );

    multisig.owner = owners;
    multisig.threshold = threshold;
    multisig.approvald = Vec::new();
    multisig.bump = *ctx.bumps.get("multisig_wallet").unwrap();

    Ok(())
}

#[derive(Accounts)]
#[instruction(owners: Vec<Pubkey>)]
pub struct InitializeMultisig<'info> {
    #[account(
        init, 
        payer = payer, 
        space =      , // Rough size
        seeds = [b"multisig"],
        bump
    )]
    pub multisig_wallet: Account<'into, MultiSigWallet>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}



impl Contract {
    fn new(name: &str, creditline_balance: u64) -> Self {
        Self {
            name: name.to_string(),
            creditline_balance,
        }
    }

    fn sign_purchase_contract(signer1: TradeProvidor, signer2: TradeProvidor, signer3: MarketProvidor, purchase_split:u64, fee: i32) -> (contract_status: signed) {
        if ser(stake_amount) == approved {
            User.self.creditline_balance -= amount; 
            self.staked += amount;
            println!("NICE {} {} {} signers signed for {} tokens release with {} fee per release!", self.name, amount);
        } elae {
            println!("NOOO Not enough balance on your creditline to purchase this product! Top up!");
        }
    }

    fn buy(&mut self, total_contract_price: u64) {
        println!("== {} clicked BUY at product price {}", self.name, total_contract_price)
        self.stake(total_contract_price);
    }
}

// we are gonna simulate the purchase, so that we can 
// start creating a model, architecture, and audit
// audit scope of sorts
fn main() {
    let mut user = User::new("Alice", 100);

    // Simulate buying product at contract price which is the stake
    // We need to decide on whether the stake will include the fee
    // or whether we remove the fee and the remaining balance
    // is the stake. 
    user.buy(40)
}
