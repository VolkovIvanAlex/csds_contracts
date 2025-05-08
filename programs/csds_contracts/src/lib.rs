use anchor_lang::prelude::*;

declare_id!("6ugnFAbncGmfwKrHNwnTmTcnvCz12mcw9RUc96qEwqtq");

#[program]
pub mod csds_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
