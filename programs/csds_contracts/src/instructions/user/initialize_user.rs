use anchor_lang::prelude::*;

use crate::state::UserAssetData;

pub fn init_user(ctx: Context<InitUser>, id: u64) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.id = id;
    user.authority = ctx.accounts.authority.key();
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct InitUser<'info> {
    #[account(
        init,
        seeds = [b"user", authority.key().as_ref()],
        bump,
        payer = authority,
        space = UserAssetData::MAX_SIZE,
    )]
    pub user: Account<'info, UserAssetData>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
