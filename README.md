# 💳 Creditline Trade Strategy Buy-In Program (Solana FFF - Rust + Anchor)

This project is a **Solana-based financial primitive** developed for the **Rektoff application**, designed to manage **on-chain creditlines, buy-in trade strategies, trade history logging, and market maker liquidity flows** using **Solana PDAs, Anchor framework, and CPIs**.

---

## 🚀 Overview

- ✅ Built in Rust using Anchor framework with front-end logic on Typescript
- ✅ Integrates creditline issuance and claim return flows basing off of user history transactions and available balances
- ✅ Tracks user investment, user Solana addresses history, multisig signing, and payouts
- ✅ Priority profit payouts to market makers via custom PDA routing
- ✅ Includes frontend prototype for credit usage, strategy buy-ins, and asset management

---

## 🔄 Flow Summary

1. **Users** connect to Market Maker and initialize:
   - `user_pda`
   - `creditline_pda`
   - `creditline_history_pda`
   - `user_trade_history_pda`

2. **Liquidity Providers** fund the `market_maker_loan_vault_pda` via CPI from their own `market_maker_loan_vault_pda`.

3. Users sign a multisig to buy into a trade strategy.

4. Payout logic distributes:
   - First to `market_maker` (priority)
   - Then to `user_creditline_pda`
   - Logs to `claim_return_pda` and `trade_history_pda`

5. Frontend displays credit balance, strategy timers, trade receipts, and PDAs.

---

## 📁 Key Folders

- `programs/ffff/` — User Buy-in Anchor smart contract logic (Rust/Solana PDA/Anchor)
- `programs/creditline_balance/`- User credit issuances Anchor smart contract logic (Rust/Solana PDA/Anchor)
- `app/` — Frontend (Typescript)

---

## 🧪 Status

✅ Established logic and PDA architecture complete  
✅ GitHub repo linked  
🛠 Multisig validation, Smart Contract Deployment, & CPI bridging in progress

---

## 🤝 For Rektoff Hackathon

This project is built for the application to **Rektoff Bootcamp** to showcase real-world usage of **Solana smart contracts**, **financial layering**, and **on-chain portfolio strategies**.

---

## 📜 License

MIT License — Use it, fork it, wreck it. Just give credit to Nerofea - tag on X @NerofeaOfficial.

---

