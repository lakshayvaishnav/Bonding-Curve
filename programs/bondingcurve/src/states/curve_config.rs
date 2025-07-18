use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,

    /*
    The maximum amount of SOL (in lamports) that can be deposited into the bonding curve.
    When this limit is reached, the curve is considered "complete".
     */
    pub curve_limit: u64, // Lmports to complete the bonding curve

    // Curve token/sol amount reserves
    pub initial_virtual_token_reserve: u64,
    pub initial_virtual_sol_reserve: u64,
    pub initial_real_token_reserve: u64,
    pub total_token_supply: u64,

    // Fee precentages
    pub buy_fee_percentage: f64,
    pub sell_fee_percentage: f64,
    pub migration_fee_percentage: f64,

    pub reserved: [[u8; 8]; 8],
}

/*

Why Only a Single “real token” Field?
1. The “Token” in Config is the Base Token
The config is for the token being launched (the base token).
The “real token reserve” is the amount of this base token held by the curve at launch.
2. SOL (Quote Token) is Handled Separately
The amount of SOL in the curve is tracked in a different field:
initial_virtual_sol_reserve (virtual SOL)
The actual SOL balance is managed by the program’s accounts and updated as users buy/sell.
3. Why Not a “real SOL reserve” in Config?
The actual SOL held by the curve is not set in the config, but is managed dynamically as users interact with the curve.
The config sets the initial state (virtual reserves for both tokens, real reserve for the base token), but the real SOL reserve is just the account’s balance.
4. Virtual Reserves for Both, Real Reserve for Base
Virtual reserves exist for both tokens to smooth the curve.
Real reserve is only needed for the base token at launch, because:
The curve starts with a known amount of the new token to sell.
The SOL reserve starts at zero (or is built up as users buy the token).

*/
