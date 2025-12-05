use anchor_lang::prelude::*;

declare_id!("8pHMThgBKvZNJTR7uJd1jrsTNapsCb9H8VBmTPBNsoZM");

#[program]
pub mod notes_app {
    use super::*;

    pub fn create_note(ctx: Context<CreateNote>, note_id: String, note: String) -> Result<()>{
        require!(note.len() <= 50, ErrorCode::NoteTooLong);
        ctx.accounts.note_pda.note = note;
        Ok(())
    }
}

#[account]
pub struct Note {
    pub note: String,
}

#[derive(Accounts)]
#[instruction(note_id: String)]
pub struct CreateNote<'info> {
    #[account(init, payer = signer, space = 8 + 4 + 50, seeds = [b"note", signer.key().as_ref(), note_id.as_bytes()], bump)]
    pub note_pda: Account<'info, Note>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Note is too long (max 50 characters).")]
    NoteTooLong,
}