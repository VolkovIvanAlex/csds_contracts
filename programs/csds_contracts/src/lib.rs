use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("J4E53PAX9fJvEH65dsrDgDDDuTD1bu5dbwpkCbjBvDay");

#[program]
pub mod csds {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // pub fn initialize_user(ctx: Context<InitUser>, id: u64) -> Result<()> {
    //     init_user(ctx, id)
    // }

    // pub fn mint_soulbound_nft(
    //     ctx: Context<MintSoulboundNFT>,
    //     data: MintSoulboundNFTArgs,
    // ) -> Result<()> {
    //     instructions::soulbound::mint_soulbound_nft::mint_soulbound_nft_handler(ctx, data)
    // }

    // pub fn update_soulbound_nft(
    //     ctx: Context<UpdateSoulboundNFT>,
    //     data: UpdateSoulboundNFTArgs,
    // ) -> Result<()> {
    //     instructions::soulbound::update_soulbound_nft::update_soulbound_nft_handler(ctx, data)
    // }

    pub fn create_report(
        ctx: Context<CreateReport>,
        report_id: u64,
        content_uri: String,
        collection_name: String,
        collection_uri: String,
    ) -> Result<()> {
        instructions::create_report(ctx, report_id, content_uri, collection_name, collection_uri)
    }

    pub fn share_report(
        ctx: Context<ShareReport>,
        report_id: u64,
        share_index: u64,
        content_uri: String,
    ) -> Result<()> {
        share_report::share_report(ctx, report_id, share_index, content_uri)
    }

    pub fn revoke_share(ctx: Context<RevokeShare>, report_id: u64, share_index: u64) -> Result<()> {
        revoke_share::revoke_share(ctx, report_id, share_index)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
