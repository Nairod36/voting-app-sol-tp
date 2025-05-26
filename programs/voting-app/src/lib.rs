use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod instructions;

use crate::state::*;

#[derive(Accounts)]
#[instruction(description: String, title: String, choices: Vec<String>, date_start: u64, date_end: u64)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 +
                4 + description.len() +
                4 + title.len() +
                4 + choices.len() * (4 + 200 + 8) +
                8 + // date_start
                8 + // date_end
                32, // creator
        seeds = [b"proposal", creator.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut, seeds = [b"proposal", proposal.creator.as_ref(), proposal.title.as_bytes()], bump)]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init,
        payer = voter,
        space = 8 + 1,
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteProposal<'info> {
    #[account(
        mut,
        close = creator,
        has_one = creator,
        seeds = [b"proposal", creator.key().as_ref(), proposal.title.as_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    pub creator: Signer<'info>,
}

declare_id!("3EsuQiYf7amAw6AVwA3XtAKKrDEftFQXJAEY6yPJnpyE");

#[program]
pub mod voting_app {
    use super::*;
    use super::{CreateProposal, CastVote, DeleteProposal};

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