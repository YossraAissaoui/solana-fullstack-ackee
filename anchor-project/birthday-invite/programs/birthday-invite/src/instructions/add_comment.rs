use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(event_name: String)]
pub struct AddComment<'info> {
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
    ctx: Context<AddComment>,
    _event_name: String,
    comment_text: String,
) -> Result<()> {
    // Validate comment text length
    require!(
        !comment_text.is_empty() && comment_text.len() <= 500,
        ErrorCode::InvalidComment
    );

    let birthday_event = &mut ctx.accounts.birthday_event;

    // Check if comments list is at max capacity
    require!(
        birthday_event.comments.len() < 5,
        ErrorCode::TooManyComments
    );

    let comment_id = birthday_event.comments.len() as u64;

    birthday_event.comments.push(Comment {
        comment_author: ctx.accounts.author.key(),
        comment_id,
        content: comment_text,
    });

    Ok(())
}