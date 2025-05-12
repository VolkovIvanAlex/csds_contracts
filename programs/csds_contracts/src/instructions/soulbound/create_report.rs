use anchor_lang::prelude::*;
use mpl_core::instructions::{CreateCollectionV2CpiBuilder, CreateV2CpiBuilder};

use crate::state::{ReportCollection, ReportData};

pub fn create_report(
    ctx: Context<CreateReport>,
    report_id: u64,
    content_uri: String,
    collection_name: String,
    collection_uri: String,
) -> Result<()> {
    let report_collection = &mut ctx.accounts.report_collection;
    let creator = ctx.accounts.creator.key();

    // Initialize report collection
    report_collection.report_id = report_id;
    report_collection.creator = creator;
    report_collection.collection_key = ctx.accounts.collection.key();
    report_collection.owner_nft = ctx.accounts.owner_nft.key();

    // Create Metaplex Core collection
    CreateCollectionV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .collection(&ctx.accounts.collection.to_account_info())
        .update_authority(Some(&ctx.accounts.update_authority)) // Use creator as update_authority
        .payer(&ctx.accounts.creator)
        .system_program(&ctx.accounts.system_program) // Added system_program
        .name(collection_name)
        .uri(collection_uri)
        .plugins(vec![])
        .invoke()?;
    // .invoke_signed(&[&[
    //     b"collection",
    //     creator.as_ref(),
    //     report_id.to_le_bytes().as_ref(),
    //     &[ctx.bumps.collection], // Use collection bump
    // ]])?;

    // Create owner NFT
    CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program)
        .asset(&ctx.accounts.owner_nft)
        .collection(Some(&ctx.accounts.collection))
        .authority(Some(&ctx.accounts.creator))
        .payer(&ctx.accounts.creator) // Explicitly set payer
        .system_program(&ctx.accounts.system_program) // Added system_program
        .name(format!("Report {} Owner", report_id))
        .uri(content_uri.clone())
        //.invoke()?;
        .invoke_signed(&[&[
            b"owner_nft",
            creator.as_ref(),
            report_id.to_le_bytes().as_ref(),
            &[ctx.bumps.owner_nft], // Use owner_nft bump
        ]])?;

    // Initialize report data for owner NFT
    let report_data = &mut ctx.accounts.report_data;
    report_data.report_id = report_id;
    report_data.content_uri = content_uri;
    report_data.is_owner_nft = true;
    report_data.shared_with = None;

    Ok(())
}

#[derive(Accounts)]
#[instruction(report_id: u64, content_uri: String, collection_name: String, collection_uri: String)]
pub struct CreateReport<'info> {
    #[account(
        init,
        payer = creator,
        space = ReportCollection::MAX_SIZE,
        seeds = [b"report_collection", creator.key().as_ref(), report_id.to_le_bytes().as_ref()],
        bump
    )]
    pub report_collection: Account<'info, ReportCollection>,
    #[account(
        init,
        payer = creator,
        space = ReportData::MAX_SIZE,
        seeds = [b"report_data", creator.key().as_ref(), report_id.to_le_bytes().as_ref()],
        bump
    )]
    pub report_data: Account<'info, ReportData>,
    // #[account(
    //         init,
    //         payer = creator,
    //         space = 100, // Adjust based on Metaplex Core collection size
    //         seeds = [b"collection", creator.key().as_ref(), report_id.to_le_bytes().as_ref()],
    //         bump
    //     )]
    #[account(mut)]
    pub collection: Signer<'info>,
    #[account(
            init,
            payer = creator,
            space = 100, // Adjust based on Metaplex Core NFT size
            seeds = [b"owner_nft", creator.key().as_ref(), report_id.to_le_bytes().as_ref()],
            bump
        )]
    pub owner_nft: AccountInfo<'info>, // PDA for owner NFT
    /// CHECK: Must be the same as creator or a valid authority
    pub update_authority: Signer<'info>, // Changed to Signer, required
    // /// CHECK: Initialized by Metaplex Core
    // #[account(mut)]
    // pub owner_nft: AccountInfo<'info>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: AccountInfo<'info>,
}
