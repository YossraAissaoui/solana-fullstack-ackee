use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(event_name: String, comment_id: u64)]
pub struct RemoveComment<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        mut,
        seeds = [
            event_name.as_bytes(),
            EVENT_SEED.as_bytes(),
            birthday_event.creator.as_ref(), 
            ],
        bump = birthday_event.bump
    )]
    pub birthday_event: Account<'info, BirthdayEvent>,
}

pub fn handler(
    ctx: Context<RemoveComment>,
    _event_name: String,
    comment_id: u64,
) -> Result<()> {
    let birthday_event = &mut ctx.accounts.birthday_event;
    let author = ctx.accounts.author.key();

    // Find and remove the comment
    if let Some(pos) = birthday_event
        .comments
        .iter()
        .position(|c| c.comment_id == comment_id)
    {
        // Verify comment author
        require!(
            birthday_event.comments[pos].comment_author == author,
            ErrorCode::Unauthorized
        );

        birthday_event.comments.remove(pos);
        Ok(())
    } else {
        Err(ErrorCode::CommentNotFound.into())
    }
}