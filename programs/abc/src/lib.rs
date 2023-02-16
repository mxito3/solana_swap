use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod abc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let from_account  = ctx.accounts.from.to_account_info();
        let to_account =  ctx.accounts.to.to_account_info();
        let token = ctx.accounts.token_program.to_account_info();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    /// CHECK
    // #[account(mut)]
    // pub from :AccountInfo<'info>
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK
    #[account()]
    pub token_program: AccountInfo<'info>,
}
