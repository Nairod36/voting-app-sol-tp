use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod contexts;
pub mod instructions;

declare_id!("3EsuQiYf7amAw6AVwA3XtAKKrDEftFQXJAEY6yPJnpyE");

pub mod voting_app {
    use super::*;
    use super::contexts::*;

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        description: String,
        title: String,
        choices: Vec<String>,
        date_start: u64,
        date_end: u64,
    ) -> Result<()> {
        instructions::create_proposal(ctx, description, title, choices, date_start, date_end)
    }

    pub fn cast_vote(
        ctx: Context<CastVote>,
        choice_index: u8,
    ) -> Result<()> {
        instructions::cast_vote(ctx, choice_index)
    }

    pub fn delete_proposal(
        ctx: Context<DeleteProposal>,
    ) -> Result<()> {
        instructions::delete_proposal(ctx)
    }
}