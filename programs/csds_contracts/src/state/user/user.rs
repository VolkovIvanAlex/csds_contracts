use anchor_lang::prelude::*;

#[account]
pub struct UserAssetData {
    pub id: u64,
    pub authority: Pubkey, // The producer's wallet
    pub created_at: i64,
}

impl UserAssetData {
    pub const MAX_SIZE: usize = 8  // discriminator
        + 8                       // id
        + 32                      // asset
        + 8; // created_at
}
