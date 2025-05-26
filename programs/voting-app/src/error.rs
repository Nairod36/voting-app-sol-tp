use anchor_lang::prelude::*;

/// Codes d’erreur du programme
#[error_code]
pub enum ErrorCode {
    #[msg("Too many choices, maximum 5 allowed.")]
    TooManyChoices,
    #[msg("Voting period hasn't started yet.")]
    VoteTooEarly,
    #[msg("Voting period has ended.")]
    VoteTooLate,
    #[msg("Choice not found.")]
    ChoiceNotFound,
    #[msg("Overflow occurred.")]
    Overflow,
    #[msg("Too early to delete proposal.")]
    TooEarlyToDelete,
}