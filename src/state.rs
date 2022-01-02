use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, borsh::try_from_slice_unchecked, program_error::ProgramError, pubkey::Pubkey,
};

// ======== ======== ======== ======== ======== ======== ======== ========

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Copy, Debug, Default, PartialEq)]
pub struct ProposalData {
    /// Creator
    pub creator: Pubkey,

    /// Governance mint address
    pub gov_mint: Pubkey,

    /// Governance token store by
    pub gov_store: Pubkey,

    /// Proposal begin at unix timestamp
    pub begin_ts: u64,

    /// Proposal duration, unix seconds
    pub duration: u64,

    /// Power of proposal now
    pub power: u64,

    /// Coefficient of power to token amount
    pub power_coefficient: u64,

    /// Stake token to create a proposal
    pub stake: u64,

    /// Is creator redeem it's token
    pub is_done: bool,
}

impl ProposalData {
    pub const LEN: usize = 32 * 3 + 8 * 5 + 1;

    pub fn from_account_info(a: &AccountInfo) -> Result<ProposalData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn is_initialized(&self) -> bool {
        if self.begin_ts == 0 && self.duration == 0 && self.power == 0 && self.is_done == false {
            return false;
        } else {
            return true;
        }
    }

    pub fn is_ended(&self) -> bool {
        if self.is_initialized() == true && crate::utils::now_timestamp() > self.begin_ts + self.duration {
            return true;
        }
        return false;
    }
}

// ======== ======== ======== ======== ======== ======== ======== ========

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, Default, PartialEq)]
pub struct UpvoteArgs {
    /// Power of vote
    pub power: u64,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Copy, Debug, Default, PartialEq)]
pub struct VoteData {
    pub voter: Pubkey,
    pub amount: u64, // 100,000,000 total supply, 6 decimals
    pub is_done: bool,
}

impl VoteData {
    pub const LEN: usize = 32 + 8 + 1;

    pub fn from_account_info(a: &AccountInfo) -> Result<VoteData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}
