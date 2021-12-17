use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::*;

// #[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AppInstruction {
    /// Create a proposal
    Create,

    /// Vote and stake token
    Upvote(UpvoteArgs),
}
