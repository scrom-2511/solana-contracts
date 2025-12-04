use anchor_lang::prelude::*;

declare_id!("99xdYPo967PFiwNpMSKTem2xBmdWUQ1qpxBFR4NUb4SV");

#[program]
pub mod info_keeper {
    use super::*;

    pub fn init(ctx: Context<Initialize>, name: String, age: i32, email: String) -> Result<()> {
        ctx.accounts.name_pda.name = name;
        ctx.accounts.age_pda.age = age;
        ctx.accounts.email_pda.email = email;
        Ok(())
    }
}

#[account]
pub struct Name {
    pub name: String,
}

#[account]
pub struct Age {
    pub age: i32,
}

#[account]
pub struct Email {
    pub email: String,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 4,
        seeds = [b"name", signer.key().as_ref()],
        bump
    )]
    pub name_pda: Account<'info, Name>,

    #[account(
        init,
        payer = signer,
        space = 8 + 4,
        seeds = [b"age", signer.key().as_ref()],
        bump
    )]
    pub age_pda: Account<'info, Age>,

    #[account(
        init,
        payer = signer,
        space = 8 + 4,
        seeds = [b"email", signer.key().as_ref()],
        bump
    )]
    pub email_pda: Account<'info, Email>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
