use anchor_lang::prelude::*;
use mpl_core::instructions::BurnV1CpiBuilder;

use crate::{
    errors::soulbound::CSDSError,
    state::{ReportCollection, ReportData},
};

pub fn revoke_share(ctx: Context<RevokeShare>, report_id: u64, share_index: u64) -> Result<()> {
    let report_collection = &ctx.accounts.report_collection;
    let creator = ctx.accounts.creator.key();
    let share_data = &ctx.accounts.share_data;

    // Verify creator and share NFT
    require!(
        report_collection.creator == creator,
        CSDSError::Unauthorized
    );
    require!(
        report_collection.report_id == report_id,
        CSDSError::InvalidReportId
    );
    require!(!share_data.is_owner_nft, CSDSError::NotShareNFT);
    require!(
        share_data.shared_with == Some(ctx.accounts.shared_org.key()),
        CSDSError::ShareNFTNotFound
    );

    // Burn share NFT
    BurnV1CpiBuilder::new(&ctx.accounts.mpl_core_program)
        .asset(&ctx.accounts.share_nft)
        .collection(Some(&ctx.accounts.collection))
        .authority(Some(&ctx.accounts.creator))
        .payer(&ctx.accounts.creator)
        .invoke()?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(report_id: u64, share_index: u64)]
pub struct RevokeShare<'info> {
    #[account(
        seeds = [b"report_collection", creator.key().as_ref(), report_id.to_le_bytes().as_ref()],
        bump
    )]
    pub report_collection: Account<'info, ReportCollection>,
    #[account(
        mut,
        seeds = [b"share_nft", creator.key().as_ref(), report_id.to_le_bytes().as_ref(), share_index.to_le_bytes().as_ref()],
        bump,
        close = creator
    )]
    pub share_data: Account<'info, ReportData>,
    /// CHECK: Validated by Metaplex Core
    #[account(mut)]
    pub collection: Signer<'info>,
    /// CHECK: Validated by Metaplex Core
    #[account(mut)]
    pub share_nft: Signer<'info>,
    #[account(mut)]
    pub creator: Signer<'info>,
    /// CHECK: Organization to revoke share from
    pub shared_org: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: AccountInfo<'info>,
}
