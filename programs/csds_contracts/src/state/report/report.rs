use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ReportCollection {
    pub report_id: u64,
    pub collection_key: Pubkey, // Metaplex Core collection address
    pub owner_nft: Pubkey,      // Owner NFT address
    pub creator: Pubkey,        // Original creator of the report
}

#[account]
#[derive(Default)]
pub struct ReportData {
    pub report_id: u64,
    pub content_uri: String,         // URI to report content (e.g., IPFS)
    pub is_owner_nft: bool,          // True for owner NFT, false for share NFT
    pub shared_with: Option<Pubkey>, // Organization shared with (for share NFTs)
}

impl ReportCollection {
    pub const MAX_SIZE: usize = 8  // discriminator
        + 8                       // report_id
        + 32                      // collection_key
        + 32                      // owner_nft
        + 32                      // creator
        ;
}

impl ReportData {
    pub const MAX_SIZE: usize = 8  // discriminator
        + 8                       // report_id
        + 260                      // content_uri
        + 1                      // is_owner_nft
        + 33                     // shared_with
        ;
}
