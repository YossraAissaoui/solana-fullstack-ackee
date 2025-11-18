use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Event name must be 1-32 bytes")]
    InvalidEventName,

    #[msg("Event date must be in the future")]
    PastDateNotAllowed,

    #[msg("Event has already passed")]
    EventPassed,

    #[msg("Maximum 5 RSVPs allowed for this event")]
    TooManyRSVPs,

    #[msg("Maximum 5 comments allowed for this event")]
    TooManyComments,

    #[msg("Comment content must be 1-500 bytes")]
    InvalidComment,

    #[msg("Only comment author can delete")]
    Unauthorized,

    #[msg("Comment not found")]
    CommentNotFound,

    #[msg("Event not found")]
    EventNotFound,

    #[msg("Invalid date format")]
    InvalidDate,
}