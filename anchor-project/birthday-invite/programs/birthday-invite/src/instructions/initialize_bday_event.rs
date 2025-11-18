use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(event_name: String)]
pub struct CreateBirthdayEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = BirthdayEvent::INIT_SPACE + 8,
        seeds = [
            event_name.as_bytes(),
            EVENT_SEED.as_bytes(),
            creator.key().as_ref()], 
        bump  
    )]
    pub birthday_event: Account<'info, BirthdayEvent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateBirthdayEvent>,
    event_name: String,
    event_date: i64,
) -> Result<()> {
    // Validate event name
    require!(
        !event_name.is_empty() && event_name.len() <= 32,
        ErrorCode::InvalidEventName
    );

    // Validate event date is in the future
    let current_time = Clock::get()?.unix_timestamp;
    require!(event_date > current_time, ErrorCode::PastDateNotAllowed);

    // Initialize the birthday event
    let birthday_event = &mut ctx.accounts.birthday_event;
    birthday_event.creator = ctx.accounts.creator.key();
    birthday_event.event_name = event_name;
    birthday_event.event_date = event_date;
    birthday_event.coming_count = 0;
    birthday_event.busy_count = 0;
    birthday_event.rsvps = vec![];
    birthday_event.comments = vec![];
    birthday_event.bump = ctx.bumps.birthday_event;

    Ok(())
}