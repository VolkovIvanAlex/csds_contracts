use anchor_lang::prelude::*;
use mpl_core::{
    instructions::CreateV2CpiBuilder,
    types::{FreezeDelegate, Plugin, PluginAuthority, PluginAuthorityPair},
};

use crate::{
    errors::soulbound::CSDSError,
    state::{ReportCollection, ReportData},
};

pub fn share_report(
    ctx: Context<ShareReport>,
    report_id: u64,
    report_name: String,
    share_index: u64,
    content_uri: String,
) -> Result<()> {
    let report_collection = &ctx.accounts.report_collection;
    let creator = ctx.accounts.creator.key();

    // Verify creator
    require!(
        report_collection.creator == creator,
        CSDSError::Unauthorized
    );
    require!(
        report_collection.report_id == report_id,
        CSDSError::InvalidReportId
    );

    // Create share NFT
    CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program)
        .asset(&ctx.accounts.share_nft)
        .collection(Some(&ctx.accounts.collection))
        .authority(Some(&ctx.accounts.creator))
        .payer(&ctx.accounts.creator) // Explicitly set payer
        .system_program(&ctx.accounts.system_program) // Added system_program
        .name(format!("Report {} (Shared)", report_name))
        .uri(content_uri.clone())
        // .plugins(vec![PluginAuthorityPair {
        //     plugin: Plugin::FreezeDelegate(FreezeDelegate { frozen: true }),
        //     authority: Some(PluginAuthority::Address {
        //         address: (ctx.accounts.creator.key()),
        //     }),
        // }])
        .invoke()?;

    // Initialize report data for share NFT
    let report_data = &mut ctx.accounts.share_data;
    report_data.report_id = report_id;
    report_data.content_uri = content_uri;
    report_data.is_owner_nft = false;
    report_data.shared_with = Some(ctx.accounts.shared_org.key());

    Ok(())
}

#[derive(Accounts)]
#[instruction(report_id: u64, report_name: String, share_index: u64, content_uri: String)]
pub struct ShareReport<'info> {
    #[account(
        seeds = [b"report_collection", creator.key().as_ref(), report_id.to_le_bytes().as_ref()],
        bump
    )]
    pub report_collection: Account<'info, ReportCollection>,
    #[account(
        init,
        payer = creator,
        space = ReportData::MAX_SIZE,
        seeds = [b"share_nft", creator.key().as_ref(), report_id.to_le_bytes().as_ref(), share_index.to_le_bytes().as_ref()],
        bump
    )]
    pub share_data: Account<'info, ReportData>,
    /// CHECK: Initialized by Metaplex Core
    #[account(mut)]
    pub collection: Signer<'info>,
    #[account(mut)]
    pub share_nft: Signer<'info>,
    #[account(mut)]
    pub creator: Signer<'info>,
    /// CHECK: Organization to share with
    pub shared_org: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: AccountInfo<'info>,
}
