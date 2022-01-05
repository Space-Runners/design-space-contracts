use solana_program::program_error::ProgramError;
use thiserror::Error;

#[macro_export]
macro_rules! ferror {
    ($s:expr) => {{
        msg!("ferror {}", $s);
        Err(ProgramError::Custom(0xfe as u32))
    }};
}

#[derive(Error, Debug, Copy, Clone)]
pub enum AppError {
    #[error("Invalid signer")]
    InvalidSigner = 0xfa01,

    #[error("Invalid derived key")]
    InvalidDerivedKey = 0xfa02,

    #[error("Not Rent Exempt")]
    NotRentExempt = 0xfa03,

    #[error("Invalid owner")]
    InvalidOwner = 0xfa04,

    #[error("Already initialized")]
    AlreadyInitialized = 0xfa05,

    #[error("Uninitialized")]
    Uninitialized = 0xfa06,

    #[error("Invalid associated address")]
    InvalidAssociatedAddress = 0xfa07,

    #[error("Invalid eq pubkey")]
    InvalidEqPubkey = 0xfa08,

    #[error("Token transfer failed")]
    TokenTransferFailed = 0xfa09,

    #[error("Checked calculate failed")]
    CheckedCalculateFailed = 0xfa0a,
}

impl From<AppError> for ProgramError {
    fn from(err: AppError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
