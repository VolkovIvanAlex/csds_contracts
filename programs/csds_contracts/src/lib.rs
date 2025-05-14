use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("67BhTPzgtPcLkKJ4cUaMUBcnTeCRxTRvpjfXfmrrF2Hp");

#[program]
pub mod csds {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_report(
        ctx: Context<CreateReport>,
        report_id: u64,
        report_name: String,
        content_uri: String,
        collection_name: String,
        collection_uri: String,
        organization_name: String,
    ) -> Result<()> {
        instructions::create_report(
            ctx,
            report_id,
            report_name,
            content_uri,
            collection_name,
            collection_uri,
            organization_name,
        )
    }

    pub fn share_report(
        ctx: Context<ShareReport>,
        report_id: u64,
        report_name: String,
        share_index: u64,
        content_uri: String,
    ) -> Result<()> {
        share_report::share_report(ctx, report_id, report_name, share_index, content_uri)
    }

    pub fn revoke_share(ctx: Context<RevokeShare>, report_id: u64, share_index: u64) -> Result<()> {
        revoke_share::revoke_share(ctx, report_id, share_index)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
