use anchor_lang::prelude::*;

#[account]
pub struct Report {
    pub author_pubkey: Pubkey, // Reference to author
    pub token_mint: Pubkey,    //reference to created token address
    pub created_at: i64,
}

impl Report {
    pub const MAX_SIZE: usize = 8  // discriminator
        + 32                      // author_pubkey
        + 32                      // token_mint
        + 8; // created_at
}
