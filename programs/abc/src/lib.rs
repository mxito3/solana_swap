use anchor_lang::prelude::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

use anchor_spl::token::transfer;
#[program]
pub mod abc {
    use super::*;
    

    
    pub fn handler(ctx: Context<Initialize>) -> Result<()> {
        // let from_account = &mut ctx.accounts.from;
        // let to_account_info = &ctx.accounts.to;
        // let authority_info = &ctx.accounts.authority;
        // let token_program_info = &ctx.accounts.token_program;
    
        // let amount = from_account.amount;
        // let from_account_info = &ctx.accounts.from.to_account_info();
        Ok(())
    }

    

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     let from_account  = ctx.accounts.from.to_account_info();
    //     let to_account =  ctx.accounts.to.to_account_info();
    //     let token = ctx.accounts.token_program.to_account_info();

    //     Ok(())
    // }

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




// #[derive(Accounts)]
// pub struct Withdraw<'info> {
//     #[account(mut)]
//     from: AccountInfo<'info>,
//     #[account(mut)]
//     to: AccountInfo<'info>,
//     // #[account(signer)]
//     // authority: AccountInfo<'info>,
//     #[account(mut)]
//     token_program: AccountInfo<'info>,
// }
