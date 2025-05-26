use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode as AppErrorCode;
use crate::{CreateProposal, CastVote, DeleteProposal};

/// Logique métier pour créer une proposition
pub fn create_proposal(
    ctx: Context<CreateProposal>,
    description: String,
    title: String,
    choices: Vec<String>,
    date_start: u64,
    date_end: u64,
) -> Result<()> {
    let prop = &mut ctx.accounts.proposal;
    require!(choices.len() <= 5, AppErrorCode::TooManyChoices);
    prop.description = description;
    prop.title = title;
    prop.date_start = date_start;
    prop.date_end = date_end;
    prop.creator = *ctx.accounts.creator.key;
    prop.choices = choices
        .into_iter()
        .map(|name| Choice { name, count: 0 })
        .collect();
    Ok(())
}

/// Logique métier pour voter
pub fn cast_vote(
    ctx: Context<CastVote>,
    choice_index: u8,
) -> Result<()> {
    let clock = Clock::get()?;
    let prop = &mut ctx.accounts.proposal;
    require!(clock.unix_timestamp as u64 >= prop.date_start, AppErrorCode::VoteTooEarly);
    require!(clock.unix_timestamp as u64 <= prop.date_end,   AppErrorCode::VoteTooLate);
    require!((choice_index as usize) < prop.choices.len(),   AppErrorCode::ChoiceNotFound);

    prop.choices[choice_index as usize].count = prop.choices[choice_index as usize]
        .count
        .checked_add(1)
        .ok_or(AppErrorCode::Overflow)?;
    let vote = &mut ctx.accounts.vote;
    vote.choice = choice_index;
    Ok(())
}

/// Logique métier pour supprimer une proposition (après délai)
pub fn delete_proposal(
    ctx: Context<DeleteProposal>,
) -> Result<()> {
    let clock = Clock::get()?;
    let prop = &ctx.accounts.proposal;
    require!(
        clock.unix_timestamp as u64 >= prop.date_end + 30 * 24 * 60 * 60,
        AppErrorCode::TooEarlyToDelete
    );
    Ok(())
}