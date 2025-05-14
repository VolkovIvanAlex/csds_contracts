use anchor_lang::prelude::*;
use mpl_core::{
    instructions::{CreateCollectionV2CpiBuilder, CreateV2CpiBuilder},
    types::{Attribute, Attributes, FreezeDelegate, Plugin, PluginAuthority, PluginAuthorityPair},
};

use anchor_spl::metadata::{
    mpl_token_metadata::{self},
    Metadata,
};

use crate::{
    errors::CSDSError,
    state::{ReportCollection, ReportData},
};

pub fn create_report(
    ctx: Context<CreateReport>,
    report_id: u64,
    report_name: String,
    content_uri: String,
    collection_name: String,
    collection_uri: String,
    organization_name: String,
) -> Result<()> {
    if organization_name.len() > 50 {
        return Err(CSDSError::OrgNameTooLong.into());
    }
    if report_name.len() > 50 {
        return Err(CSDSError::ReportNameTooLong.into());
    }

    let report_collection = &mut ctx.accounts.report_collection;
    let creator = ctx.accounts.creator.key();

    // Initialize report collection
    report_collection.report_id = report_id;
    report_collection.creator = creator;
    report_collection.collection_key = ctx.accounts.collection.key();
    report_collection.owner_nft = ctx.accounts.owner_nft.key();

    // Create owner NFT with attributes
    let attributes = Attributes {
        attribute_list: vec![
            Attribute {
                key: "report_id".to_string(),
                value: report_id.to_string(),
            },
            Attribute {
                key: "creator".to_string(),
                value: creator.to_string(),
            },
            Attribute {
                key: "organization_name".to_string(),
                value: organization_name,
            },
        ],
    };

    let attributes_plugin = PluginAuthorityPair {
        plugin: Plugin::Attributes(attributes),
        authority: Some(PluginAuthority::Address {
            address: ctx.accounts.creator.key(),
        }), // Creator controls attributes
    };

    // Create Metaplex Core collection
    CreateCollectionV2CpiBuilder::new(&ctx.accounts.mpl_core_program)
        .collection(&ctx.accounts.collection)
        .update_authority(Some(&ctx.accounts.update_authority)) // Use creator as update_authority
        .payer(&ctx.accounts.creator)
        .system_program(&ctx.accounts.system_program) // Added system_program
        .name(collection_name)
        .uri(collection_uri)
        .plugins(vec![attributes_plugin])
        //.plugins(vec![])
        .invoke()?;

    // Create owner NFT
    CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program)
        .asset(&ctx.accounts.owner_nft)
        .collection(Some(&ctx.accounts.collection))
        .authority(Some(&ctx.accounts.creator))
        .payer(&ctx.accounts.creator) // Explicitly set payer
        .owner(Some(&ctx.accounts.creator))
        .system_program(&ctx.accounts.system_program)
        .name(format!("Report {} (Owner)", report_name))
        .uri(content_uri.clone())
        .plugins(vec![PluginAuthorityPair {
            plugin: Plugin::FreezeDelegate(FreezeDelegate { frozen: true }),
            authority: None,
        }])
        // .plugins(vec![attributes_plugin])
        .invoke()?;

    // Initialize report data for owner NFT
    let report_data = &mut ctx.accounts.report_data;
    report_data.report_id = report_id;
    report_data.content_uri = content_uri;
    report_data.is_owner_nft = true;
    report_data.shared_with = None;

    Ok(())
}

#[derive(Accounts)]
#[instruction(report_id: u64, content_uri: String, collection_name: String, collection_uri: String, organization_name: String, report_name: String)]
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

    #[account(
            mut,
            seeds = [b"metadata", mpl_token_metadata_program.key().as_ref(), creator.key().as_ref()],
            bump,
            seeds::program = mpl_token_metadata_program.key(),
        )]
    pub metadata_account: UncheckedAccount<'info>,

    // #[account(
    //         mut,
    //         seeds = [b"metadata", mpl_token_metadata_program.key().as_ref(), creator.key().as_ref(), b"edition"],
    //         bump,
    //         seeds::program = mpl_token_metadata_program.key(),
    //     )]
    // pub master_edition_account: UncheckedAccount<'info>,
    /// CHECK: Initialized by Metaplex Core
    #[account(mut)]
    pub collection: Signer<'info>,
    /// CHECK: Initialized by Metaplex Core
    #[account(mut)]
    pub owner_nft: Signer<'info>,
    /// CHECK: Must be the same as creator or a valid authority
    pub update_authority: Signer<'info>, // Changed to Signer, required
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(address = mpl_token_metadata::ID)]
    pub mpl_token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: AccountInfo<'info>,
}
