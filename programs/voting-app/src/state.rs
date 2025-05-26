use anchor_lang::prelude::*;

/// On-chain data for a Proposal
#[account]
pub struct Proposal {
    pub description: String,
    pub title:       String,
    pub choices:     Vec<Choice>,
    pub date_start:  u64,
    pub date_end:    u64,
    pub creator:     Pubkey,
}

/// One option within a Proposal
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Choice {
    pub name:  String,
    pub count: u64,
}

/// Tracks that a given voter has cast exactly one vote
#[account]
pub struct Vote {
    pub choice: u8,
}