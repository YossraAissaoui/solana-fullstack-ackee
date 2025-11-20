use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod states;

pub use instructions::*;
pub use states::*;

declare_id!("FLf5wn4AFa6ssQAhVYHjkEwoTyJPi7UQ7gZnv5jEWUJM");

#[program]
pub mod birthday_invite {
    use super::*;

    // Create a new birthday event
    pub fn initialize_bday_event(
        ctx: Context<CreateBirthdayEvent>,
        event_name: String,
        event_date: i64,
    ) -> Result<()> {
        instructions::initialize_bday_event::handler(ctx, event_name, event_date)
    }

    // Confirm attendance (RSVP coming)
    pub fn confirm_attendance(
        ctx: Context<ConfirmAttendance>,
        event_name: String,
    ) -> Result<()> {
        instructions::confirm_attendance::handler(ctx, event_name)
    }

    // Decline attendance (RSVP busy)
    pub fn decline_attendance(
        ctx: Context<DeclineAttendance>,
        event_name: String,
    ) -> Result<()> {
        instructions::decline_attendance::handler(ctx, event_name)
    }

    // Add a comment to the event
    pub fn add_comment(
        ctx: Context<AddComment>,
        event_name: String,
        comment_text: String,
    ) -> Result<()> {
        instructions::add_comment::handler(ctx, event_name, comment_text)
    }

    // Remove a comment from the event
    pub fn remove_comment(
        ctx: Context<RemoveComment>,
        event_name: String,
        comment_id: u64,
    ) -> Result<()> {
        instructions::remove_comment::handler(ctx, event_name, comment_id)
    }
}