use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("6ugnFAbncGmfwKrHNwnTmTcnvCz12mcw9RUc96qEwqtq");

#[program]
pub mod csds_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitUser>, id: u64) -> Result<()> {
        init_user(ctx, id)
    }

    pub fn mint_soulbound_nft(
        ctx: Context<MintSoulboundNFT>,
        data: MintSoulboundNFTArgs,
    ) -> Result<()> {
        instructions::soulbound::mint_soulbound_nft::mint_soulbound_nft_handler(ctx, data)
    }

    pub fn update_soulbound_nft(
        ctx: Context<UpdateSoulboundNFT>,
        data: UpdateSoulboundNFTArgs,
    ) -> Result<()> {
        instructions::soulbound::update_soulbound_nft::update_soulbound_nft_handler(ctx, data)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
