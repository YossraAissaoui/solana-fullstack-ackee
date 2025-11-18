use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(event_name: String)]
pub struct ConfirmAttendance<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            event_name.as_bytes(),
            EVENT_SEED.as_bytes(), 
            birthday_event.creator.as_ref()
            ], 
        bump = birthday_event.bump
    )]
    pub birthday_event: Account<'info, BirthdayEvent>,
}

pub fn handler(
    ctx: Context<ConfirmAttendance>, 
    _event_name: String
    ) -> Result<()> {
    let birthday_event = &mut ctx.accounts.birthday_event;
    let signer = ctx.accounts.signer.key();

    // Validate event is still in the future
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        birthday_event.event_date > current_time,
        ErrorCode::EventPassed
    );

    // Check if signer already has an RSVP
    if let Some(pos) = birthday_event
        .rsvps
        .iter()
        .position(|r| r.invited_person == signer)
    {
        // Handle existing RSVP
        if birthday_event.rsvps[pos].is_coming {
            // Toggle off: remove the vote
            birthday_event.coming_count = birthday_event.coming_count.saturating_sub(1);
            birthday_event.rsvps.remove(pos);
        } else {
            // Switch from Declined to Confirmed
            {
                let rsvp = &mut birthday_event.rsvps[pos];
                rsvp.is_coming = true;
            }
            birthday_event.busy_count = birthday_event.busy_count.saturating_sub(1);
            birthday_event.coming_count += 1;
        }
    } else {
        // Check if RSVP list is at max capacity
        require!(
            birthday_event.rsvps.len() < 5,
            ErrorCode::TooManyRSVPs
        );

        // First vote: Confirmed
        birthday_event.coming_count += 1;
        birthday_event.rsvps.push(RSVP {
            invited_person: signer,
            is_coming: true,
        });
    }

    Ok(())
}