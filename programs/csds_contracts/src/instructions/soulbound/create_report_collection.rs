// use anchor_lang::prelude::*;
// use mpl_core::instructions::CreateCollectionV2CpiBuilder;

// use crate::{errors::CSDSError, state::ReportCollection};

// pub fn create_report(
//     ctx: Context<CreateReportCollection>,
//     report_id: u64,
//     collection_name: String,
//     collection_uri: String,
//     organization_name: String,
// ) -> Result<()> {
//     if organization_name.len() > 50 {
//         return Err(CSDSError::OrgNameTooLong.into());
//     }

//     let report_collection = &mut ctx.accounts.report_collection;
//     let creator = ctx.accounts.creator.key();

//     // Initialize report collection
//     report_collection.report_id = report_id;
//     report_collection.creator = creator;
//     report_collection.collection_key = ctx.accounts.collection.key();
//     //report_collection.owner_nft = ctx.accounts.owner_nft.key();

//     let update_authority = match &ctx.accounts.update_authority {
//         Some(update_authority) => Some(update_authority.to_account_info()),
//         None => None,
//     };

//     // Create Metaplex Core collection
//     CreateCollectionV2CpiBuilder::new(&ctx.accounts.mpl_core_program)
//         .collection(&ctx.accounts.collection)
//         .payer(&ctx.accounts.creator)
//         .update_authority(update_authority.as_ref())
//         .system_program(&ctx.accounts.system_program) // Added system_program
//         .name(collection_name)
//         .uri(collection_uri)
//         .plugins(vec![])
//         .invoke()?;
//     // .invoke_signed(&[&[
//     //     b"collection",
//     //     creator.as_ref(),
//     //     report_id.to_le_bytes().as_ref(),
//     //     &[ctx.bumps.collection], // Use collection bump
//     // ]])?;

//     Ok(())
// }

// #[derive(Accounts)]
// #[instruction(report_id: u64, collection_name: String, collection_uri: String, organization_name: String)]
// pub struct CreateReportCollection<'info> {
//     #[account(
//         init,
//         payer = creator,
//         space = ReportCollection::MAX_SIZE,
//         seeds = [b"report_collection", creator.key().as_ref(), report_id.to_le_bytes().as_ref()],
//         bump
//     )]
//     pub report_collection: Account<'info, ReportCollection>,
//     /// CHECK: Initialized by Metaplex Core
//     #[account(mut)]
//     pub collection: Signer<'info>,
//     /// CHECK: this account will be checked by the mpl_core program
//     pub update_authority: Option<UncheckedAccount<'info>>,
//     /// CHECK: Initialized by Metaplex Core
//     #[account(mut)]
//     pub creator: Signer<'info>,
//     pub system_program: Program<'info, System>,
//     #[account(address = mpl_core::ID)]
//     pub mpl_core_program: AccountInfo<'info>,
// }
