use crate::errors::MintSoulboundNftErrorCode;
use crate::state::user::user::UserAssetData;
use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::{self},
        Metadata,
    },
    token::Token,
};
use mpl_core::{
    instructions::CreateV2CpiBuilder,
    types::{FreezeDelegate, Plugin, PluginAuthorityPair},
};

pub fn mint_soulbound_nft_handler(
    ctx: Context<MintSoulboundNFT>,
    args: MintSoulboundNFTArgs,
) -> Result<()> {
    // require_keys_eq!(
    //     ctx.accounts.admin.key(),
    //     ctx.accounts.config.admin,
    //     MintSoulboundNftErrorCode::Unauthorized
    // );

    let MintSoulboundNFTArgs { uri, user_id } = args;

    let user_asset_data = &mut ctx.accounts.user_asset_data;
    let asset_account = &ctx.accounts.asset_account;
    let user = &ctx.accounts.user;
    let asset_authority = &ctx.accounts.asset_authority;
    let mpl_core_program = &ctx.accounts.mpl_core_program;
    let system_program = &ctx.accounts.system_program;
    let csds_program = &ctx.accounts.csds_program;

    //user_asset_data.asset = asset_account.key();

    let asset_account_info = asset_account.to_account_info();
    let asset_authority_account_info = asset_authority.to_account_info();
    let user_account_info = user.to_account_info();

    let asset_account_seeds: &[&[&[u8]]] = &[&[
        b"soulbound_asset",
        &user_id.as_bytes(),
        &csds_program.key().to_bytes(),
        &[ctx.bumps.asset_account],
    ]];

    let asset_authority_seeds: &[&[&[u8]]] = &[&[
        b"asset_authority",
        &csds_program.key().to_bytes(),
        &asset_account.key().to_bytes(),
        &[ctx.bumps.asset_authority],
    ]];

    let mut builder = CreateV2CpiBuilder::new(mpl_core_program);
    let builder = builder
        .asset(&asset_account_info)
        .name(String::from("CSDS Soulbound"))
        .uri(uri.clone())
        .authority(Some(&asset_authority_account_info))
        .payer(&user_account_info)
        .owner(Some(&user_account_info))
        .update_authority(Some(&asset_authority_account_info))
        .system_program(system_program)
        .plugins(vec![PluginAuthorityPair {
            plugin: Plugin::FreezeDelegate(FreezeDelegate { frozen: true }),
            authority: None,
        }]);

    builder
        .invoke_signed(&[asset_account_seeds[0], asset_authority_seeds[0]])
        .map_err(|_| MintSoulboundNftErrorCode::AssetCreationError)?;

    Ok(())
}
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct MintSoulboundNFTArgs {
    uri: String,
    user_id: String,
}

#[derive(Accounts)]
#[instruction(args: MintSoulboundNFTArgs)]
pub struct MintSoulboundNFT<'info> {
    #[account(mut,
        seeds = [b"user_asset_data", args.user_id.as_bytes(),  csds_program.key().as_ref()],
        bump,
    )]
    pub user_asset_data: Account<'info, UserAssetData>,

    #[account(
        mut,
        seeds = [b"soulbound_asset", args.user_id.as_bytes(), csds_program.key().as_ref()],
        bump,
        seeds::program = csds_program.key()
    )]
    pub asset_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"asset_authority", csds_program.key().as_ref(), asset_account.key().as_ref()],
        bump,
        seeds::program = csds_program.key()
    )]
    pub asset_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"metadata", mpl_token_metadata_program.key().as_ref(), asset_account.key().as_ref()],
        bump,
        seeds::program = mpl_token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"metadata", mpl_token_metadata_program.key().as_ref(), asset_account.key().as_ref(), b"edition"],
        bump,
        seeds::program = mpl_token_metadata_program.key(),
    )]
    pub master_edition_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    // #[account(mut)]
    // pub admin: Signer<'info>,

    // #[account(
    //     seeds = [b"config"],
    //     bump,
    // )]
    // pub config: Account<'info, Config>,
    #[account(address = crate::ID)]
    pub csds_program: AccountInfo<'info>,

    #[account(address = mpl_core::ID)]
    pub mpl_core_program: AccountInfo<'info>,

    #[account(address = mpl_token_metadata::ID)]
    pub mpl_token_metadata_program: Program<'info, Metadata>,

    pub token_program: Program<'info, Token>,

    #[account(address = solana_program::system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(address = solana_program::sysvar::instructions::ID)]
    pub sysvar_instructions: AccountInfo<'info>,
}
